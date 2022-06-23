use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ScriptFilterResult<'a> {
    pub items: Vec<Item<'a>>,
}

#[derive(Debug, Serialize)]
pub struct Item<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub r#match: &'a str,
    pub arg: &'a str,
    pub autocomplete: Option<&'a str>,
    pub mods: Option<HashMap<&'a str, Mod<'a>>>,
}

#[derive(Debug, Serialize)]
pub struct Mod<'a> {
    pub arg: &'a str,
    pub subtitle: &'a str,
}
