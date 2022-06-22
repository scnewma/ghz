use std::{
    fs::{DirBuilder, File},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{config::Config, github::Repo};

#[derive(Debug, Deserialize, Serialize)]
pub struct Cache {
    #[serde(skip)]
    cache_dir: PathBuf,

    pub repositories: Option<Vec<Repo>>,
}

impl Cache {
    pub fn empty(cache_dir: PathBuf) -> Cache {
        Cache {
            cache_dir,
            repositories: None,
        }
    }

    pub fn load(cfg: &Config) -> anyhow::Result<Cache> {
        Cache::load_from(&cfg.cache_dir())
    }

    pub fn save(self) -> anyhow::Result<()> {
        let repos_cache = self.cache_dir.join("repositories.json");
        write_json(&repos_cache, &self).with_context(|| {
            format!(
                "could not save repositories cache ({})",
                self.cache_dir.display()
            )
        })?;
        Ok(())
    }

    fn load_from(dir: &PathBuf) -> anyhow::Result<Cache> {
        let repos_cache = dir.join("repositories.json");
        if !repos_cache.exists() {
            return Ok(Cache::empty(dir.clone()));
        }

        read_json(&repos_cache).with_context(|| {
            format!(
                "could not load repositories cache ({})",
                repos_cache.display()
            )
        })
    }
}

fn write_json<P, S>(path: P, s: &S) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    S: Serialize,
{
    let path = path.as_ref();
    DirBuilder::new()
        .recursive(true)
        .create(path.parent().unwrap())?;
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, s)?;
    Ok(())
}

fn read_json<P, T>(path: P) -> anyhow::Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let file = File::open(path.as_ref())?;
    let reader = BufReader::new(file);
    let v = serde_json::from_reader(reader)?;
    Ok(v)
}
