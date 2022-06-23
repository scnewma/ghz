use std::collections::HashMap;

use clap::Args;

use crate::{alfred, cache::Cache, config::Config, github};
use skim::prelude::*;

#[derive(Debug, Args)]
pub struct Opts {
    /// The filter string to match on
    filter: String,
    /// Output in alfred format
    #[clap(long)]
    alfred: bool,
}

pub fn run(cfg: &Config, opts: Opts) -> anyhow::Result<()> {
    let cache = Cache::load(cfg)?;
    match cache.repositories {
        Some(repos) => {
            let engine = ExactOrFuzzyEngineFactory::builder()
                .fuzzy_algorithm(FuzzyAlgorithm::SkimV2)
                .build()
                .create_engine(&opts.filter);

            let matches: Vec<&github::Repo> = repos
                .iter()
                .filter_map(|repo| {
                    engine
                        .match_item(Arc::new(repo.name_with_owner.clone()))
                        .map(|_result| repo)
                })
                .collect();

            if opts.alfred {
                let items: Vec<alfred::Item> = matches
                    .iter()
                    .map(|r| {
                        let mut mods = HashMap::new();
                        mods.insert(
                            "alt",
                            alfred::Mod {
                                arg: &r.ssh_url,
                                subtitle: &r.ssh_url,
                            },
                        );

                        alfred::Item {
                            title: &r.name_with_owner,
                            subtitle: &r.url,
                            r#match: &r.name_with_owner,
                            arg: &r.url,
                            autocomplete: Some(&r.name_with_owner),
                            mods: Some(mods),
                        }
                    })
                    .collect();
                let res = alfred::ScriptFilterResult { items };
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else {
                for m in matches {
                    println!("{}", m.name_with_owner);
                }
            }
            Ok(())
        }
        None => Ok(()),
    }
}
