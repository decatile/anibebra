use derive_builder::Builder;
use reqwest::Url;
use serde::Serialize;

const API_ANILIBRIA_HOST: &str = "https://api.anilibria.tv";

#[derive(Builder, Serialize)]
#[serde_with::skip_serializing_none]
#[builder(setter(strip_option))]
pub struct TitleRequest {
    id: Option<i32>,
    code: Option<String>,
    torrent_id: Option<i32>,
    filter: Option<Vec<String>>,
    remove: Option<Vec<String>>,
    include: Option<Vec<String>>,
    description_type: Option<String>,
    playlist_type: Option<String>,
}

pub async fn title(request: TitleRequest) -> reqwest::Result<reqwest::Response> {
    let params = serde_url_params::to_string(&request).unwrap();
    let url = format!("{API_ANILIBRIA_HOST}/v3/title?{params}");
    reqwest::get(Url::parse(&url).unwrap()).await
}
