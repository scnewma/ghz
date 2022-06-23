use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use std::process::{self, Command};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub name: String,
    pub name_with_owner: String,
    pub url: String,
    pub ssh_url: String,
}

pub fn repo_list(owner: &str, limit: Option<u32>) -> anyhow::Result<Vec<Repo>> {
    let out = exec(&vec![
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
    Ok(Command::new("gh")
        .args(args)
        .output()
        .context("error executing gh command")?)
}
