use failure::Fallible;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use rocket_contrib::templates::tera::Context;

fn default_config_file() -> PathBuf {
    OsStr::new("Config.toml").to_os_string().into()
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub appearance: Appearance,
    pub google_sign_in: Option<GoogleSignIn>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Appearance {
    pub color: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct GoogleSignIn {
    pub client_id: String,
    pub domain_filter: String,
}

impl Config {
    pub fn load() -> Fallible<Self> {
        let buffer = Self::load_as_string(default_config_file())?;

        Ok(::toml::from_str(&buffer)?)
    }

    fn load_as_string(filename: PathBuf) -> Fallible<String> {
        let mut buffer = String::new();
        File::open(filename)?.read_to_string(&mut buffer)?;

        Ok(buffer)
    }
}

impl GoogleSignIn {
    pub fn init_tera(&self, context: &mut Context) {
        context.insert("google_sign_in", &true);
        context.insert("client_id", &self.client_id);
    }

    pub fn check_domain(&self, mail: &str) -> bool {
        if self.domain_filter.len() == 0 {
            return true;
        }

        self.domain_filter
            .split_whitespace()
            .any(|filter| mail.ends_with(filter))
    }
}
