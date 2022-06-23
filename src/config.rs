use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

fn homedir() -> PathBuf {
    dirs::home_dir().expect("could not get home directory path")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackedOwner {
    pub name: String,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    pub config_dir: PathBuf,
    #[serde(skip)]
    pub cache_dir: PathBuf,
    pub owners: Option<Vec<TrackedOwner>>,
}

impl Config {
    pub fn load() -> anyhow::Result<Config> {
        let path = Config::config_file();
        if !path.exists() {
            // TODO: return error here and expect caller to call default()?
            return Ok(Config::default());
        }

        let mut cfg: Config = read_json(path).context("error loading configuration from file")?;
        cfg.config_dir = default_config_dir();
        cfg.cache_dir = default_cache_dir();
        Ok(cfg)
    }

    fn config_file() -> PathBuf {
        default_config_dir().join("config.json")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_dir: default_config_dir(),
            cache_dir: default_cache_dir(),
            owners: None,
        }
    }
}

fn read_json<P, T>(path: P) -> anyhow::Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let file = File::open(path.as_ref()).context("opening config file")?;
    let reader = BufReader::new(file);
    let v = serde_json::from_reader(reader).context("error parsing config json")?;
    Ok(v)
}

fn default_config_dir() -> PathBuf {
    let mut buf = homedir();
    buf.push(".config");
    buf.push("ghz");
    buf
}

fn default_cache_dir() -> PathBuf {
    let mut buf = default_config_dir();
    buf.push("cache");
    buf
}
