mod shiki_api;
mod model;

use title_plugin_system::PluginInfo;
use title_plugin_system::title_plugin::{Title, PlumbaPlugin, TitleImage};
use crate::shiki_api::{ShikimoriApi, ShikiParseError};

struct ShikiTitlePlugin {
    api: ShikimoriApi
}

#[async_trait::async_trait]
impl PlumbaPlugin for ShikiTitlePlugin {
    async fn get_title_with_id(&self, id: usize) -> Result<Title, String> {
        let anime = match self.api.anime(id).await {
            Ok(anime) => anime,
            Err(err) => {
                let err_str = match err {
                    ShikiParseError::JsonParseError(err) => "error when get_title_with_id - 'JsonParseError'. ".to_owned() + &err,
                    ShikiParseError::BodyParseError(err) => "error when get_title_with_id - 'BodyParseError'. ".to_owned() + &err,
                    ShikiParseError::RequestError(err) => "error when get_title_with_id - 'RequestError'. ".to_owned() + &err,
                };
                return Err(err_str)
            }
        };

        Ok(
            Title {
                id: anime.id,
                eng_name: anime.name,
                russian_name: anime.russian,
                score: anime.score,
                description: anime.description,
                image: TitleImage {
                    original: Some(anime.image.original.0),
                    preview: Some(anime.image.preview.0),
                    x96: Some(anime.image.x96.0),
                    x48: Some(anime.image.x48.0),
                },
                url: anime.url.0,
            }
        )
    }

    async fn get_season_titles(&self) -> Result<Vec<Title>, String> {
        let season_anime = match self.api.season_titles().await {
            Ok(anime) => anime,
            Err(err) => {
                let err_str = match err {
                    ShikiParseError::JsonParseError(err) => "error when get_title_with_id - 'JsonParseError'. ".to_owned() + &err,
                    ShikiParseError::BodyParseError(err) => "error when get_title_with_id - 'BodyParseError'. ".to_owned() + &err,
                    ShikiParseError::RequestError(err) => "error when get_title_with_id - 'RequestError'. ".to_owned() + &err,
                };
                return Err(err_str)
            }
        };

        Ok(
            season_anime.iter().map(|anime| {
                let anime = anime.clone();
                Title {
                    id: anime.id,
                    eng_name: anime.name,
                    russian_name: anime.russian,
                    score: anime.score,
                    description: None,
                    image: TitleImage {
                        original: Some(anime.image.original.0),
                        preview: Some(anime.image.preview.0),
                        x96: Some(anime.image.x96.0),
                        x48: Some(anime.image.x48.0),
                    },
                    url: anime.url.0,
                }
            }).collect()
        )
    }

    async fn search(&self, search: String) -> Result<Vec<Title>, String> {
        let search_res = match self.api.search(search.as_str()).await {
            Ok(anime) => anime,
            Err(err) => {
                let err_str = match err {
                    ShikiParseError::JsonParseError(err) => "error when get_title_with_id - 'JsonParseError'. ".to_owned() + &err,
                    ShikiParseError::BodyParseError(err) => "error when get_title_with_id - 'BodyParseError'. ".to_owned() + &err,
                    ShikiParseError::RequestError(err) => "error when get_title_with_id - 'RequestError'. ".to_owned() + &err,
                };
                return Err(err_str)
            }
        };

        Ok(
            search_res.iter().map(|anime| {
                let anime = anime.clone();
                Title {
                    id: anime.id,
                    eng_name: anime.name,
                    russian_name: anime.russian,
                    score: anime.score,
                    description: None,
                    image: TitleImage {
                        original: Some(anime.image.original.0),
                        preview: Some(anime.image.preview.0),
                        x96: Some(anime.image.x96.0),
                        x48: Some(anime.image.x48.0),
                    },
                    url: anime.url.0,
                }
            }).collect()
        )
    }

    async fn query_title(&self) -> Result<Vec<Title>, String> {
        todo!()
    }

    fn plugin_info(&self) -> PluginInfo {
        let icon = include_bytes!("../shikimori_logo.png").as_slice();
        PluginInfo {
            name: String::from("Shikimori plugin"),
            version: String::from("0.0.1"),
            description: String::from("Плагин для просмотра аниме. Сайт shikimori.one"),
            icon_data: icon,
        }
    }

    fn set_mirror(&mut self, mirror: &str) {
        self.api.set_mirror(mirror.to_string())
    }
}

#[no_mangle]
pub fn create_plugin() -> *mut dyn PlumbaPlugin {
    Box::into_raw(Box::new(ShikiTitlePlugin { api: ShikimoriApi::new() }))
}