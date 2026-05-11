use std::fs;

use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, LanguageServerId, Os, Result, Worktree,
};

const EXTENSION_VERSION: &str = "0.1.0";
const PROXY_DIR: &str = "atlas-ls-zed-proxy";

struct Platform {
    os: &'static str,
    arch: &'static str,
    exe: &'static str,
}

struct AtlasHclExtension;

impl AtlasHclExtension {
    fn platform() -> Result<Platform> {
        let (os, arch) = zed::current_platform();
        let (os, exe) = match os {
            Os::Mac => ("darwin", ""),
            Os::Linux => ("linux", ""),
            Os::Windows => ("windows", ".exe"),
        };
        let arch = match arch {
            Architecture::Aarch64 => "arm64",
            Architecture::X8664 => "amd64",
            _ => return Err(format!("unsupported architecture: {arch:?}")),
        };
        Ok(Platform { os, arch, exe })
    }

    fn proxy_path() -> Result<String> {
        let platform = Self::platform()?;
        let filename = format!(
            "atlas-ls-zed-proxy-{}-{}{}",
            platform.os, platform.arch, platform.exe
        );
        let path = format!("{PROXY_DIR}/{EXTENSION_VERSION}/{filename}");

        if fs::metadata(&path).is_err() {
            fs::create_dir_all(format!("{PROXY_DIR}/{EXTENSION_VERSION}"))
                .map_err(|err| format!("failed to create {PROXY_DIR}: {err}"))?;

            let url = format!(
                "https://github.com/edlundin/zed-atlasgo/releases/download/v{EXTENSION_VERSION}/{filename}"
            );
            zed::download_file(&url, &path, DownloadedFileType::Uncompressed).map_err(|err| {
                format!("failed to download Atlas LS Zed proxy from {url}: {err}")
            })?;

            if platform.exe.is_empty() {
                zed::make_file_executable(&path).map_err(|err| {
                    format!("failed to make Atlas LS Zed proxy executable: {err}")
                })?;
            }
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
        let atlas = worktree.which("atlas").ok_or_else(|| {
            "Atlas CLI not found on PATH. Install Atlas from https://atlasgo.io/getting-started/ and ensure `atlas` is available on PATH.".to_string()
        })?;

        Ok(zed::Command {
            command: Self::proxy_path()?,
            args: vec![atlas, "tool".into(), "lsp".into(), "--stdio".into()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(AtlasHclExtension);
