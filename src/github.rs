use serde::{Deserialize, Serialize};
use std::process::{self, Command};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub name_with_owner: String,
}

pub fn repo_list() -> anyhow::Result<Vec<Repo>> {
    let out = exec(&vec![
        "repo",
        "list",
        "--limit",
        "1000",
        "--json",
        "nameWithOwner",
    ])?;
    let repos: Vec<Repo> = serde_json::from_slice(&out.stdout)?;

    Ok(repos)
}

fn exec(args: &[&str]) -> anyhow::Result<process::Output> {
    Ok(Command::new("gh").args(args).output()?)
}
