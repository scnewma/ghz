use crate::{cache::Cache, config::Config, github};

pub fn run(cfg: &Config) -> anyhow::Result<()> {
    let mut cache = Cache::empty(cfg.cache_dir.clone());
    cache.repositories = match &cfg.owners {
        Some(owners) => {
            let mut repositories = Vec::new();
            for owner in owners {
                let repos = github::repo_list(&owner.name, owner.limit)?;
                repositories.extend(repos)
            }
            Some(repositories)
        }
        None => None,
    };

    cache.save()?;
    Ok(())
}
