use anyhow::{bail, Context, Result};
use flate2::read::GzDecoder;
use std::fs::{self, File, Permissions};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::Duration;

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

    /// Detect architecture and return release filename for given version
    pub fn get_arch_filename(version: &str) -> Result<String> {
        let arch = std::env::consts::ARCH;
        match arch {
            "x86_64" => Ok(format!("mihomo-linux-amd64-compatible-{}.gz", version)),
            "aarch64" => Ok(format!("mihomo-linux-arm64-{}.gz", version)),
            "arm" | "armv7" => Ok(format!("mihomo-linux-armv7-{}.gz", version)),
            "armv6" => Ok(format!("mihomo-linux-armv6-{}.gz", version)),
            "armv5" => Ok(format!("mihomo-linux-armv5-{}.gz", version)),
            "x86" | "i386" | "i686" => Ok(format!("mihomo-linux-386-{}.gz", version)),
            other => bail!("Unsupported system architecture: {}", other),
        }
    }

    /// Check if a local TCP port is actively accepting connections
    fn is_local_port_open(port: u16) -> bool {
        TcpStream::connect_timeout(
            &SocketAddr::from(([127, 0, 0, 1], port)),
            Duration::from_millis(150),
        )
        .is_ok()
    }

    /// Dynamically query GitHub releases to get the latest release tag (e.g. "v1.19.30")
    pub async fn fetch_latest_version(clients: &[reqwest::Client]) -> String {
        let check_urls = [
            "https://github.com/MetaCubeX/mihomo/releases/latest",
            "https://ghproxy.net/https://github.com/MetaCubeX/mihomo/releases/latest",
        ];

        for client in clients {
            for url in &check_urls {
                if let Ok(resp) = client.get(*url).send().await {
                    let final_url = resp.url().as_str();
                    if let Some(tag_part) = final_url.split("/tag/").nth(1) {
                        let tag = tag_part.trim_matches('/').to_string();
                        if tag.starts_with('v') && tag.len() >= 4 {
                            return tag;
                        }
                    }
                }
            }
        }

        // Fallback to stable version if remote detection fails
        "v1.19.30".to_string()
    }

    /// Download latest Mihomo release binary and atomically install it
    pub async fn download_and_install<F>(progress_cb: F) -> Result<PathBuf>
    where
        F: Fn(&str),
    {
        progress_cb("正在检测可用网络连接与代理环境...");

        // 1. Prepare HTTP clients: proxy client (if available) and direct client
        let direct_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(45))
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()?;

        let mut proxy_client = None;
        if let Ok(proxy_str) = std::env::var("HTTP_PROXY").or_else(|_| std::env::var("http_proxy")) {
            if let Ok(p) = reqwest::Proxy::all(&proxy_str) {
                if let Ok(c) = reqwest::Client::builder()
                    .proxy(p)
                    .timeout(Duration::from_secs(45))
                    .redirect(reqwest::redirect::Policy::limited(5))
                    .build()
                {
                    proxy_client = Some(c);
                }
            }
        } else {
            // Probe local proxy ports if active
            let candidate_ports = [7890, 7897, 7891];
            for port in candidate_ports {
                if Self::is_local_port_open(port)
                    && let Ok(p) = reqwest::Proxy::all(format!("http://127.0.0.1:{}", port))
                    && let Ok(c) = reqwest::Client::builder()
                        .proxy(p)
                        .timeout(Duration::from_secs(45))
                        .redirect(reqwest::redirect::Policy::limited(5))
                        .build()
                {
                    proxy_client = Some(c);
                    break;
                }
            }
        }

        let mut query_clients = Vec::new();
        if let Some(ref pc) = proxy_client {
            query_clients.push(pc.clone());
        }
        query_clients.push(direct_client.clone());

        // 2. Dynamically determine latest version tag
        progress_cb("正在获取 Mihomo 官方最新版本号...");
        let version = Self::fetch_latest_version(&query_clients).await;
        progress_cb(&format!("检测到最新内核版本: {}", version));

        let filename = Self::get_arch_filename(&version)?;
        let target_path = Self::target_binary_path()?;

        // 3. Candidate mirror download URLs
        let urls = [
            format!(
                "https://github.com/MetaCubeX/mihomo/releases/download/{}/{}",
                version, filename
            ),
            format!(
                "https://ghproxy.net/https://github.com/MetaCubeX/mihomo/releases/download/{}/{}",
                version, filename
            ),
            format!(
                "https://gh-proxy.com/https://github.com/MetaCubeX/mihomo/releases/download/{}/{}",
                version, filename
            ),
            format!(
                "https://gh.ddlc.top/https://github.com/MetaCubeX/mihomo/releases/download/{}/{}",
                version, filename
            ),
        ];

        let mut last_err = None;
        let mut gz_bytes = Vec::new();

        // 4. Download file stream
        'outer: for url in &urls {
            // For each URL, try with proxy client first (if any), then direct client
            let try_clients: Vec<&reqwest::Client> = match &proxy_client {
                Some(pc) => vec![pc, &direct_client],
                None => vec![&direct_client],
            };

            for client in try_clients {
                progress_cb(&format!("正在连接下载源: {}...", url));
                match client.get(url).send().await {
                    Ok(resp) if resp.status().is_success() => {
                        progress_cb(&format!("连接成功，正在拉取 {} 核心包...", version));
                        match resp.bytes().await {
                            Ok(b) => {
                                gz_bytes = b.to_vec();
                                progress_cb("核心包下载成功，准备解压与安装...");
                                break 'outer;
                            }
                            Err(e) => last_err = Some(anyhow::anyhow!("下载数据流失败: {}", e)),
                        }
                    }
                    Ok(resp) => {
                        last_err = Some(anyhow::anyhow!("HTTP {} 来自: {}", resp.status(), url));
                    }
                    Err(e) => {
                        last_err = Some(anyhow::anyhow!("连接失败 ({}): {}", url, e));
                    }
                }
            }
        }

        if gz_bytes.is_empty() {
            if let Some(err) = last_err {
                return Err(err);
            } else {
                bail!("下载 Mihomo 核心包失败");
            }
        }

        // 5. Decompress GZ stream
        progress_cb("解压 GZ 编译包中...");
        let mut decoder = GzDecoder::new(&gz_bytes[..]);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .context("解压 Mihomo 二进制文件失败")?;

        // 6. Write binary to temporary file first, avoiding ETXTBSY (Text file busy) error
        // when Mihomo is currently executing
        progress_cb("写入新版本二进制文件...");
        let tmp_path = target_path.with_extension("tmp");
        let mut file = File::create(&tmp_path).context("创建临时二进制文件失败")?;
        file.write_all(&decompressed)?;
        file.flush()?;
        drop(file);

        // 7. Grant executable permission (chmod +x / 0755)
        fs::set_permissions(&tmp_path, Permissions::from_mode(0o755))
            .context("设置 chmod +x 可执行权限失败")?;

        // 8. Atomically replace the existing binary
        fs::rename(&tmp_path, &target_path).context("原子替换目标二进制文件失败")?;

        progress_cb(&format!("Mihomo 核心 ({}) 安装成功！", version));
        Ok(target_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_arch_filename() {
        let filename = CoreDownloader::get_arch_filename("v1.19.30").unwrap();
        assert!(filename.contains("v1.19.30"));
        assert!(filename.ends_with(".gz"));
        assert!(filename.starts_with("mihomo-linux-"));
    }

    #[test]
    fn test_target_binary_path() {
        let path = CoreDownloader::target_binary_path().unwrap();
        assert!(path.ends_with("mihomo"));
    }
}
