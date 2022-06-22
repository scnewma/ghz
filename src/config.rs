use std::path::PathBuf;

fn homedir() -> PathBuf {
    dirs::home_dir().expect("could not get home directory path")
}

pub struct Config {
    cache_dir: PathBuf,
}

impl Config {
    pub fn cache_dir(&self) -> PathBuf {
        self.cache_dir.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cache_dir: default_cache_dir(),
        }
    }
}

fn default_cache_dir() -> PathBuf {
    let mut buf = homedir();
    buf.push(".ghz");
    buf.push("cache");
    buf
}
