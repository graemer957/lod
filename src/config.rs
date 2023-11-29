use std::{error::Error, fs, fs::File, io::Write, path::PathBuf};
use tempfile::TempDir;
use toml::Table;

pub struct Config {
    temp_dir: Option<TempDir>,
    desktop_applescript_path: PathBuf,
    laptop_applescript_path: PathBuf,
    caffeinate_app: Option<String>,
    caffeinate_options: Option<String>,
}

impl Config {
    /// Attempt to load `Config` from storage
    ///
    /// # Errors
    ///
    /// - File not found if TOML is missing
    /// - Unable to parse TOML
    /// - Creation of temp directory failed
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let toml = fs::read_to_string(format!("{}/.config/lod/config.toml", env!("HOME")))?;
        let toml = toml.parse::<Table>()?;

        let temp_dir = tempfile::tempdir()?;
        dbg!(&temp_dir.path());

        let desktop_applescript_path =
            Self::create_apple_script(&toml, &temp_dir, "desktop_applescript")?;
        let laptop_applescript_path =
            Self::create_apple_script(&toml, &temp_dir, "laptop_applescript")?;
        let caffeinate_app = toml
            .get("caffeinate_app")
            .and_then(|x| x.as_str())
            .map(String::from);
        let caffeinate_options = toml
            .get("caffeinate_options")
            .and_then(|x| x.as_str())
            .map(String::from);

        Ok(Self {
            temp_dir: Some(temp_dir),
            desktop_applescript_path,
            laptop_applescript_path,
            caffeinate_app,
            caffeinate_options,
        })
    }

    fn create_apple_script(
        toml: &Table,
        temp_dir: &TempDir,
        key: &str,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let applescript = toml[key]
            .as_str()
            .ok_or_else(|| format!("`{key}` is missing from config.toml. Please add."))?;
        let path = temp_dir.path().join(format!("{key}.scpt"));
        let mut temp_file = File::create(&path)?;
        write!(temp_file, "{applescript}")?;

        Ok(path)
    }

    pub fn delete_apple_scripts(&mut self) {
        if let Some(temp_dir) = self.temp_dir.take() {
            drop(temp_dir);
        }
    }

    #[must_use]
    pub const fn desktop_applescript_path(&self) -> &PathBuf {
        &self.desktop_applescript_path
    }

    #[must_use]
    pub const fn laptop_applescript_path(&self) -> &PathBuf {
        &self.laptop_applescript_path
    }

    #[must_use]
    pub fn caffeinate_app(&self) -> Option<&str> {
        self.caffeinate_app.as_deref()
    }

    #[must_use]
    pub fn caffeinate_options(&self) -> Option<&str> {
        self.caffeinate_options.as_deref()
    }
}
