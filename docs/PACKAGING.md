# 📦 Mimo Packaging & Distribution Guide

This document specifies the official packaging, release engineering, and cross-distribution publishing scheme for **Mimo (Minimalist Mihomo Manager)**.

---

## 🏛️ Release Architecture Matrix

| Package Type | Target Distros | Target Architectures | Build / Packaging Tool | Output Artifact Format |
|---|---|---|---|---|
| **DEB Package** | Debian, Ubuntu, Linux Mint, Pop!_OS | `x86_64` (amd64), `aarch64` (arm64), `armv7` (armhf) | `NFPM` | `mimo_1.4.2_amd64.deb` |
| **RPM Package** | Fedora, RHEL, Rocky Linux, AlmaLinux | `x86_64`, `aarch64`, `armhfp` | `NFPM` | `mimo-1.4.2-1.x86_64.rpm` |
| **Arch Linux** | Arch Linux, Manjaro, EndeavourOS | `x86_64`, `aarch64` | `AUR (mimo-bin PKGBUILD)` | `mimo-1.4.2-x86_64.pkg.tar.zst` |
| **Universal Binary** | Any Linux Distribution | `x86_64`, `aarch64`, `armv7` | `Cargo` / `Cross` | `mimo-1.4.2-x86_64.tar.gz` |

| **Installer Script** | Any Linux Distribution | Auto-detect CPU & Distro | `bash` + `curl` | `install.sh` |

---

## 🚀 One-Line Installation

```bash
curl -fsSL https://raw.githubusercontent.com/aimy1/Mimo/main/scripts/install.sh | bash
```

---

## 🔒 Security & Supply Chain Verification

- **SHA-256 Checksums**: Generated automatically for all release binaries (`SHA256SUMS.txt`).
- **Capabilities**: `CAP_NET_ADMIN` capability is applied post-install for non-root TUN mode transparent proxying.
