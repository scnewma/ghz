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
}
