use anyhow::{bail, Context, Result};
use flate2::read::GzDecoder;
use std::fs::{self, File, Permissions};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub struct CoreDownloader;

impl CoreDownloader {
    /// Get target path where auto-downloaded Mihomo binary is stored (`~/.config/mimo/bin/mihomo`)
    pub fn target_binary_path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not determine home directory")?;
        let bin_dir = home.join(".config").join("mimo").join("bin");
        if !bin_dir.exists() {
            fs::create_dir_all(&bin_dir)?;
        }
        Ok(bin_dir.join("mihomo"))
    }

    /// Detect architecture and return candidate release filenames
    pub fn get_arch_filename() -> Result<&'static str> {
        let arch = std::env::consts::ARCH;
        match arch {
            "x86_64" => Ok("mihomo-linux-amd64-compatible-v1.19.29.gz"),
            "aarch64" => Ok("mihomo-linux-arm64-v1.19.29.gz"),
            "arm" | "armv7" => Ok("mihomo-linux-armv7-v1.19.29.gz"),
            other => bail!("Unsupported system architecture: {}", other),
        }
    }

    /// Download latest Mihomo release binary from GitHub or mirror and unpack
    pub async fn download_and_install<F>(progress_cb: F) -> Result<PathBuf>
    where
        F: Fn(&str),
    {
        let filename = Self::get_arch_filename()?;
        let target_path = Self::target_binary_path()?;

        let urls = [
            format!(
                "https://github.com/MetaCubeX/mihomo/releases/download/v1.19.29/{}",
                filename
            ),
            format!(
                "https://mirror.ghproxy.com/https://github.com/MetaCubeX/mihomo/releases/download/v1.19.29/{}",
                filename
            ),
            format!(
                "https://ghproxy.net/https://github.com/MetaCubeX/mihomo/releases/download/v1.19.29/{}",
                filename
            ),
            format!(
                "https://gh-proxy.com/https://github.com/MetaCubeX/mihomo/releases/download/v1.19.29/{}",
                filename
            ),
            format!(
                "https://gh.ddlc.top/https://github.com/MetaCubeX/mihomo/releases/download/v1.19.29/{}",
                filename
            ),
        ];

        let mut builder = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(90));

        // Use system/env proxy or local 127.0.0.1:7897 / 7890 if set
        if let Ok(proxy_str) = std::env::var("HTTP_PROXY").or_else(|_| std::env::var("http_proxy")) {
            if let Ok(p) = reqwest::Proxy::all(&proxy_str) {
                builder = builder.proxy(p);
            }
        } else if let Ok(p) = reqwest::Proxy::all("http://127.0.0.1:7897") {
            builder = builder.proxy(p);
        }

        let client = builder.build()?;

        let mut last_err = None;
        let mut gz_bytes = Vec::new();

        for url in &urls {
            progress_cb(&format!("正在连接下载源: {}...", url));
            match client.get(url).send().await {
                Ok(resp) if resp.status().is_success() => {
                    progress_cb("正在获取 Mihomo 核心编译包...");
                    match resp.bytes().await {
                        Ok(b) => {
                            gz_bytes = b.to_vec();
                            progress_cb("核心包下载成功，准备解压...");
                            break;
                        }
                        Err(e) => last_err = Some(anyhow::anyhow!("下载数据流失败: {}", e)),
                    }
                }
                Ok(resp) => {
                    last_err = Some(anyhow::anyhow!(
                        "HTTP {} 来自源: {}",
                        resp.status(),
                        url
                    ));
                }
                Err(e) => last_err = Some(anyhow::anyhow!("连接源失败: {}", e)),
            }
        }

        if gz_bytes.is_empty() {
            if let Some(err) = last_err {
                return Err(err);
            } else {
                bail!("下载 Mihomo 核心包失败");
            }
        }

        // Decompress GZ stream
        progress_cb("解压 GZ 编译包中...");
        let mut decoder = GzDecoder::new(&gz_bytes[..]);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .context("解压 Mihomo 二进制文件失败")?;

        // Write binary
        progress_cb("写入二进制文件到 ~/.config/mimo/bin/mihomo...");
        let mut file = File::create(&target_path)
            .context("创建目标二进制文件失败")?;
        file.write_all(&decompressed)?;
        file.flush()?;

        // Grant executable permission (chmod +x / 0755)
        fs::set_permissions(&target_path, Permissions::from_mode(0o755))
            .context("设置 chmod +x 可执行权限失败")?;

        progress_cb("Mihomo 核心安装并授权成功！");
        Ok(target_path)
    }
}
