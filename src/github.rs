use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use std::process::{self, Command};

#[derive(Debug, Serialize, Deserialize)]
pub enum RepoField {
    Name,
    NameWithOwner,
    Url,
    SshUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub name: String,
    pub name_with_owner: String,
    pub url: String,
    pub ssh_url: String,
}

impl Repo {
    pub fn owner(&self) -> Option<&str> {
        match self.name_with_owner.split_once('/') {
            Some((owner, _)) => Some(owner),
            None => None,
        }
    }

    pub fn get(&self, field: &RepoField) -> &str {
        match field {
            RepoField::Name => &self.name,
            RepoField::NameWithOwner => &self.name_with_owner,
            RepoField::Url => &self.url,
            RepoField::SshUrl => &self.ssh_url,
        }
    }
}

pub fn repo_list(owner: &str, limit: Option<u32>) -> anyhow::Result<Vec<Repo>> {
    let out = exec(&[
        "repo",
        "list",
        owner,
        "--limit",
        &limit.unwrap_or(100).to_string(),
        "--json",
        "nameWithOwner,name,url,sshUrl",
    ])
    .context(format!("error listing repositories for owner {}", owner))?;
    if !out.status.success() {
        bail!("error listing repositories for owner {}", owner)
    }
    let repos: Vec<Repo> = serde_json::from_slice(&out.stdout)?;

    Ok(repos)
}

fn exec(args: &[&str]) -> anyhow::Result<process::Output> {
    Command::new("gh")
        .args(args)
        .output()
        .context("error executing gh command")
}
