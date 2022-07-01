use std::collections::HashMap;

use clap::Args;

use crate::{
    alfred,
    cache::Cache,
    config::{self, AlfredField, Config},
    github::{self, RepoField},
};
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
        Some(repos) => filter_repos(cfg, opts, repos),
        None => Ok(()),
    }
}

fn filter_repos(cfg: &Config, opts: Opts, repos: Vec<github::Repo>) -> anyhow::Result<()> {
    // build a lookup map for owners by name
    let tracked_owners: HashMap<&str, &config::TrackedOwner> = match &cfg.owners {
        Some(owners) => owners
            .iter()
            .map(|owner| (owner.name.as_ref(), owner))
            .collect(),
        None => HashMap::new(),
    };

    let engine = ExactOrFuzzyEngineFactory::builder()
        .fuzzy_algorithm(FuzzyAlgorithm::SkimV2)
        .build()
        .create_engine(&opts.filter);

    // filter repositories to only ones that match the filter value
    let matches: Vec<&github::Repo> = repos
        .iter()
        .filter_map(|repo| {
            let owner = repo.owner().expect("repository name not in valid format");

            // determine what field to filter on. if user provided a filter_on field in their
            // configuration we use that, otherwise use name_with_owner
            let filter_field = match tracked_owners.get(owner) {
                Some(owner_cfg) => match &owner_cfg.filter_on {
                    Some(field) => field,
                    None => &RepoField::NameWithOwner,
                },
                None => &RepoField::NameWithOwner,
            };

            // extract field by name that we should filter on
            let filter_value = repo.get(filter_field).to_owned();

            // use skim to fuzzy match and determine if it's a match
            engine
                .match_item(Arc::new(filter_value))
                .map(|_result| repo)
        })
        .collect();

    if opts.alfred {
        output_alfred(matches, tracked_owners)
    } else {
        output_plaintext(matches);
        Ok(())
    }
}

fn output_plaintext(matches: Vec<&github::Repo>) {
    for m in matches {
        println!("{}", m.name_with_owner);
    }
}

fn output_alfred(
    matches: Vec<&github::Repo>,
    tracked_owners: HashMap<&str, &config::TrackedOwner>,
) -> anyhow::Result<()> {
    let items: Vec<alfred::Item> = matches
        .iter()
        .filter_map(|r| {
            let owner = r.owner().expect("repository name not in valid format");

            let mut mods = HashMap::new();
            mods.insert(
                "alt",
                alfred::Mod {
                    arg: &r.ssh_url,
                    subtitle: &r.ssh_url,
                },
            );

            let mut item = alfred::Item {
                title: &r.name_with_owner,
                subtitle: &r.url,
                r#match: &r.name_with_owner,
                arg: &r.url,
                autocomplete: Some(&r.name_with_owner),
                mods: Some(mods),
            };

            if let Some(owner_cfg) = tracked_owners.get(owner) {
                if let Some(mappings) = &owner_cfg.mappings {
                    for afield in mappings.keys() {
                        let value = r.get(&mappings[afield]);
                        match afield {
                            AlfredField::Title => item.title = value,
                            AlfredField::Subtitle => item.subtitle = value,
                            AlfredField::Match => item.r#match = value,
                            AlfredField::Arg => item.arg = value,
                            AlfredField::Autocomplete => item.autocomplete = Some(value),
                        }
                    }
                }

                Some(item)
            } else {
                None
            }
        })
        .collect();

    let res = alfred::ScriptFilterResult { items };
    println!("{}", serde_json::to_string_pretty(&res)?);
    Ok(())
}
