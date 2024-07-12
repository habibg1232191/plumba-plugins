use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShikiImage {
    pub original: ShikiUrl,
    pub preview: ShikiUrl,
    pub x96: ShikiUrl,
    pub x48: ShikiUrl
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShikiVideo {
    pub id: usize,
    pub url: String,
    pub image_url: String,
    pub player_url: String,
    pub name: String,
    pub kind: String,
    pub hosting: String
}

#[derive(Debug, Serialize, Clone)]
pub struct ShikiUrl(pub String);

impl<'de> Deserialize<'de> for ShikiUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        let s: String = Deserialize::deserialize(deserializer).unwrap();
        let split_str = s.as_str().split("?").collect::<Vec<_>>()[0];
        let res = "https://shikimori.one".to_owned() + split_str;

        Ok(ShikiUrl(res))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShikiScreenshot {
    pub original: ShikiUrl,
    pub preview: ShikiUrl
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShikiAnime {
    pub id: usize,
    pub name: String,
    pub russian: String,
    pub image: ShikiImage,
    pub url: ShikiUrl,
    pub kind: String,
    pub score: String,
    pub episodes: i32,
    pub episodes_aired: i32,
    pub aired_on: Option<String>,
    pub released_on: Option<String>,
    pub rating: String,
    pub english: Vec<Option<String>>,
    pub japanese: Vec<Option<String>>,
    pub synonyms: Vec<Option<String>>,
    pub license_name_ru: Option<String>,
    pub duration: i32,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub description_source: Option<String>,
    pub franchise: Option<String>,
    pub favoured: bool,
    pub anons: bool,
    pub ongoing: bool,
    pub myanimelist_id: i32,
    pub updated_at: DateTime<Utc>,
    pub next_episode_at: Option<DateTime<Utc>>,
    pub fansubbers: Vec<String>,
    pub fandubbers: Vec<String>,
    pub licensors: Vec<String>,
    //genres: [],
    pub studios: Vec<ShikiStudios>,
    videos: Vec<ShikiVideo>,
    screenshots: Vec<ShikiScreenshot>,
    //user_rate: null
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchShikiAnimes {
    pub id: usize,
    pub name: String,
    pub russian: String,
    pub image: ShikiImage,
    pub url: ShikiUrl,
    pub kind: Option<String>,
    pub score: String,
    pub episodes: i32,
    pub episodes_aired: i32,
    pub aired_on: Option<String>,
    pub released_on: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShikiStudios {
    pub id: i32,
    pub name: String,
    pub filtered_name: String,
    pub real: bool,
    pub image: Option<String>
}