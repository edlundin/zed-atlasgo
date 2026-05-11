use std::fs;

use zed_extension_api::{self as zed, Architecture, LanguageServerId, Os, Result, Worktree};

const PROXY_DIR: &str = "atlas-ls-zed-proxy";

struct BundledProxy {
    filename: &'static str,
    bytes: &'static [u8],
}

struct AtlasHclExtension;

impl AtlasHclExtension {
    fn platform() -> Result<(&'static str, &'static str, &'static str)> {
        let (os, arch) = zed::current_platform();
        let (os, ext) = match os {
            Os::Mac => ("darwin", ""),
            Os::Linux => ("linux", ""),
            Os::Windows => ("windows", ".exe"),
        };
        let arch = match arch {
            Architecture::Aarch64 => "arm64",
            Architecture::X8664 => "amd64",
            _ => return Err(format!("unsupported architecture: {arch:?}")),
        };
        Ok((os, arch, ext))
    }

    fn bundled_proxy() -> Result<BundledProxy> {
        let (os, arch, ext) = Self::platform()?;
        match (os, arch, ext) {
            ("darwin", "arm64", "") => Ok(BundledProxy {
                filename: "atlas-ls-zed-proxy-darwin-arm64",
                bytes: include_bytes!("../bundled-proxy/atlas-ls-zed-proxy-darwin-arm64"),
            }),
            ("darwin", "amd64", "") => Ok(BundledProxy {
                filename: "atlas-ls-zed-proxy-darwin-amd64",
                bytes: include_bytes!("../bundled-proxy/atlas-ls-zed-proxy-darwin-amd64"),
            }),
            ("linux", "arm64", "") => Ok(BundledProxy {
                filename: "atlas-ls-zed-proxy-linux-arm64",
                bytes: include_bytes!("../bundled-proxy/atlas-ls-zed-proxy-linux-arm64"),
            }),
            ("linux", "amd64", "") => Ok(BundledProxy {
                filename: "atlas-ls-zed-proxy-linux-amd64",
                bytes: include_bytes!("../bundled-proxy/atlas-ls-zed-proxy-linux-amd64"),
            }),
            ("windows", "amd64", ".exe") => Ok(BundledProxy {
                filename: "atlas-ls-zed-proxy-windows-amd64.exe",
                bytes: include_bytes!("../bundled-proxy/atlas-ls-zed-proxy-windows-amd64.exe"),
            }),
            _ => Err(format!("no bundled Atlas LS Zed proxy for {os}-{arch}")),
        }
    }

    fn proxy_path() -> Result<String> {
        let proxy = Self::bundled_proxy()?;
        let path = format!("{PROXY_DIR}/{}", proxy.filename);

        if fs::metadata(&path).is_err() {
            fs::create_dir_all(PROXY_DIR)
                .map_err(|err| format!("failed to create {PROXY_DIR}: {err}"))?;
            fs::write(&path, proxy.bytes)
                .map_err(|err| format!("failed to write bundled Atlas LS Zed proxy: {err}"))?;
            zed::make_file_executable(&path)
                .map_err(|err| format!("failed to make Atlas LS Zed proxy executable: {err}"))?;
        }

        Ok(path)
    }
}

impl zed::Extension for AtlasHclExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let server = worktree.which("atlas").ok_or_else(|| {
            "Atlas CLI not found on PATH. Install Atlas from https://atlasgo.io/getting-started/ and ensure `atlas` is available on PATH.".to_string()
        })?;

        Ok(zed::Command {
            command: Self::proxy_path()?,
            args: vec![server, "tool".into(), "lsp".into(), "--stdio".into()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(AtlasHclExtension);
