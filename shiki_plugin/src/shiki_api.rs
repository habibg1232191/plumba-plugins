use isahc::{AsyncReadResponseExt, HttpClient};
use url::Url;
use crate::model::{SearchShikiAnimes, ShikiAnime};

pub struct ShikimoriApi {
    url: String,
    http_client: HttpClient
}

impl ShikimoriApi {
    pub fn new() -> Self {
        Self {
            url: SHIKIMORI_API.to_string(),
            http_client: HttpClient::new().unwrap(),
        }
    }

    pub fn set_mirror(&mut self, mirror: String) {
        self.url = mirror;
    }
}

const SHIKIMORI_API: &str = "https://shikimori.one/api";

#[derive(Debug)]
pub enum ShikiParseError {
    JsonParseError(String),
    BodyParseError(String),
    RequestError(String),
}

impl ShikimoriApi {
    pub async fn anime(&self, id: usize) -> Result<ShikiAnime, ShikiParseError> {
        let mut response =
            match self.http_client.get_async(self.url.clone() + "/animes/" + id.to_string().as_str()).await {
                Ok(resp) => resp,
                Err(e) => return Err(ShikiParseError::RequestError(e.to_string()))
            };
        let body = match response.text().await {
            Ok(b) => b,
            Err(e) => return Err(ShikiParseError::BodyParseError(e.to_string()))
        };
        match serde_json::from_str(body.as_str()) {
            Ok(e) => Ok(e),
            Err(e) => Err(ShikiParseError::JsonParseError(e.to_string()))
        }
    }
    
    pub async fn search(&self, search_string: &str) -> Result<Vec<SearchShikiAnimes>, ShikiParseError> {
        let url_string = format!("{}/animes?search={}&limit=50&censored=true", self.url, search_string);
        let url = Url::parse(url_string.as_str()).unwrap().to_string();
        let mut response = match self.http_client.get_async(url).await {
            Ok(resp) => resp,
            Err(e) => return Err(ShikiParseError::RequestError(e.to_string()))
        };
        let body = match response.text().await {
            Ok(b) => b,
            Err(e) => return Err(ShikiParseError::BodyParseError(e.to_string()))
        };
        match serde_json::from_str(body.as_str()) {
            Ok(e) => Ok(e),
            Err(e) => Err(ShikiParseError::JsonParseError(e.to_string()))
        }
    }

    pub async fn season_titles(&self) -> Result<Vec<SearchShikiAnimes>, ShikiParseError> {
        let url_string = format!("{}/animes?limit=20&censored=true&season=summer_2024&order=popularity", self.url);
        let url = Url::parse(url_string.as_str()).unwrap().to_string();
        let mut response = match self.http_client.get_async(url).await {
            Ok(resp) => resp,
            Err(e) => return Err(ShikiParseError::RequestError(e.to_string()))
        };
        let body = match response.text().await {
            Ok(b) => b,
            Err(e) => return Err(ShikiParseError::BodyParseError(e.to_string()))
        };
        match serde_json::from_str(body.as_str()) {
            Ok(e) => Ok(e),
            Err(e) => Err(ShikiParseError::JsonParseError(e.to_string()))
        }
    }
}

#[cfg(test)]
mod test {
    use futures::executor::block_on;
    use crate::shiki_api::ShikimoriApi;

    #[test]
    fn test() {
        block_on(async {
            let api = ShikimoriApi::new().search("Jujutsu kaisen").await.unwrap();
            println!("{:?}", api)
        });
    }
}