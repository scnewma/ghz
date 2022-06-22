use crate::{cache::Cache, config::Config, github};

pub fn run(cfg: &Config) -> anyhow::Result<()> {
    let mut cache = Cache::empty(cfg.cache_dir());
    cache.repositories = Some(github::repo_list()?);

    cache.save()?;
    Ok(())
}
