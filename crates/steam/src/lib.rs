#![allow(dead_code)]

use process::ProcessControls;
const BINARY_NAME: &str = "steamcmd.exe";

#[derive(Debug)]
pub struct Account {
    username: String,
    password: String,
}

impl Account {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

#[derive(Debug)]
pub struct AppUpdate {
    app_id: u64,
    beta: Option<String>,
    beta_password: Option<String>,
    validate: bool,
}

impl AppUpdate {
    pub fn new(app_id: u64) -> Self {
        Self {
            app_id,
            beta: None,
            beta_password: None,
            validate: false,
        }
    }

    pub fn beta(mut self, beta: String, password: Option<String>) -> Self {
        self.beta = Some(beta);
        self.beta_password = password;
        self
    }

    pub fn validate(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }
}

// https://developer.valvesoftware.com/wiki/Command_Line_Options#SteamCMD
// Note: app_run, app_status, app_stop | we can probably use these

#[derive(Default, Debug)]
pub struct Steam {
    login: Option<Account>,
    force_install_dir: Option<String>,
    app_update: Option<AppUpdate>,
    workshop_download_item: Vec<(u64, i64)>, // app_id, published_file_id
}

impl Steam {
    pub async fn install() -> anyhow::Result<()> {
        let installer = installer::Installer::new(paths::get_steam_path());
        if installer.is_installed() {
            return Ok(());
        }

        installer.download().await?;
        installer.extract()?;
        installer.cleanup()?;
        installer.update().await?;

        Ok(())
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_from_env() -> Self {
        let mut steam = Self::new();

        if let Ok(username) = std::env::var("STEAM_USERNAME") {
            if let Ok(password) = std::env::var("STEAM_PASSWORD") {
                steam = steam.login(username, password);
            }
        }

        steam
    }

    pub fn login(mut self, username: String, password: String) -> Self {
        self.login = Some(Account::new(username, password));
        self
    }

    pub fn force_install_dir(mut self, dir: String) -> Self {
        self.force_install_dir = Some(dir);
        self
    }

    pub fn app_update(mut self, app: AppUpdate) -> Self {
        self.app_update = Some(app);
        self
    }

    pub fn workshop_download_item(mut self, app_id: u64, published_file_id: i64) -> Self {
        self.workshop_download_item.push((app_id, published_file_id));
        self
    }

    pub fn run(self) -> anyhow::Result<ProcessControls> {
        if !paths::get_steam_path().join(BINARY_NAME).exists() {
            return Err(anyhow::anyhow!("Steam is not installed"));
        }

        let mut process = process::Process::new(paths::get_steam_path().join(BINARY_NAME));

        if let Some(dir) = self.force_install_dir {
            process.arg("+force_install_dir");
            process.arg(dir);
        }

        if let Some(account) = self.login {
            process.arg("+login");
            process.arg(account.username);
            process.arg(account.password);
        }

        if let Some(app) = self.app_update {
            process.arg("+app_update");
            process.arg(app.app_id.to_string());

            if let Some(beta) = app.beta {
                process.arg("-beta");
                process.arg(beta);

                if let Some(password) = app.beta_password {
                    process.arg("-betapassword");
                    process.arg(password);
                }
            }

            if app.validate {
                process.arg("validate");
            }
        }

        if !self.workshop_download_item.is_empty() {
            for (app_id, published_file_id) in self.workshop_download_item {
                process.arg("+workshop_download_item");
                process.arg(app_id.to_string());
                process.arg(published_file_id.to_string());
                process.arg("validate");
            }
        }

        process.arg("+quit");

        process.log_to_file(paths::get_log_path().join("steamcmd.log"));

        Ok(process.start()?)
    }
}

mod installer;
