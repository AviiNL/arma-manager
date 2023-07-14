use anyhow::Result;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

const ARCHIVE_NAME: &str = "steamcmd.zip";

pub(crate) struct Installer {
    path: PathBuf,
}

impl Installer {
    pub(crate) fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub(crate) fn is_installed(&self) -> bool {
        self.path.join("steamclient.dll").exists()
    }

    pub(crate) async fn download(&self) -> Result<()> {
        tracing::info!("Downloading steamcmd");
        let url = "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip";
        let mut response = reqwest::get(url).await?;

        tokio::fs::create_dir_all(&self.path).await?;

        let mut dest = {
            let fname = self.path.join(ARCHIVE_NAME);
            File::create(fname).await?
        };

        while let Some(chunk) = response.chunk().await? {
            dest.write_all(&chunk).await?;
        }

        // return the file path
        Ok(())
    }

    pub(crate) fn extract(&self) -> Result<()> {
        tracing::info!("Extracting steamcmd");
        let archive = {
            let fname = self.path.join(ARCHIVE_NAME);
            std::fs::File::open(fname)?
        };

        let mut archive = zip::ZipArchive::new(archive)?;
        archive.extract(&self.path)?;

        Ok(())
    }

    pub(crate) fn cleanup(&self) -> Result<()> {
        tracing::info!("Cleaning up steamcmd");
        let fname = self.path.join(ARCHIVE_NAME);
        std::fs::remove_file(fname)?;

        Ok(())
    }

    pub(crate) async fn update(&self) -> Result<()> {
        tracing::info!("Installing steamcmd");

        let mut process = process::Process::new(paths::get_steam_path().join(super::BINARY_NAME));
        process.arg("+quit");

        process.log_to_file(paths::get_log_path().join("steamcmd.log"));
        let c = process.start()?;

        c.wait().await;

        tracing::info!("Steamcmd installed");

        Ok(())
    }

    #[allow(unused)]
    pub(crate) fn uninstall(&self) -> Result<()> {
        tracing::info!("Uninstalling steamcmd");
        // delete the steamcmd folder
        std::fs::remove_dir_all(&self.path)?;

        Ok(())
    }
}
