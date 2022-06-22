use crate::{cache::Cache, config::Config, github};

pub fn run(cfg: &Config) -> anyhow::Result<()> {
    // TODO: would be nice to not clone here
    let mut cache = Cache::empty(cfg.cache_dir.clone());
    cache.repositories = Some(github::repo_list("scnewma")?);

    cache.save()?;
    Ok(())
}
