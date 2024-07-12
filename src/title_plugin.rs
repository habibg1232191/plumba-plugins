use serde::{Deserialize, Serialize};
use crate::PluginInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleImage {
    pub original: Option<String>,
    pub preview: Option<String>,
    pub x96: Option<String>,
    pub x48: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    pub id: usize,
    pub eng_name: String,
    pub russian_name: String,
    pub score: String,
    pub description: Option<String>,
    pub image: TitleImage,
    pub url: String,
}

impl Title {
    pub fn as_json(&self) -> serde_json::error::Result<String> {
        serde_json::to_string(self)
    }
}

pub fn to_json(j: Vec<Title>) -> serde_json::error::Result<String> {
    serde_json::to_string(&j)
}

// pub trait QueryParamTitle {
//     fn query() -> Self where Self: Sized;
// }

#[async_trait::async_trait]
pub trait PlumbaPlugin: Sync + Send {
    async fn get_title_with_id(&self, id: usize) -> Result<Title, String>;
    async fn get_season_titles(&self) -> Result<Vec<Title>, String>;
    async fn search(&self, search: String) -> Result<Vec<Title>, String>;
    async fn query_title(&self) -> Result<Vec<Title>, String>;
    fn plugin_info(&self) -> PluginInfo;
    fn set_mirror(&mut self, mirror: &str);
}