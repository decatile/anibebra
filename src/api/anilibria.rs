use std::{backtrace, collections::HashMap, fmt::Display};

use derive_builder::Builder;
use reqwest::Url;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

const API_ANILIBRIA_HOST: &str = "http://api.anilibria.tv";

#[derive(Builder, Serialize, Default)]
#[serde_with::skip_serializing_none]
#[builder(setter(strip_option), default)]
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

#[derive(Builder, Serialize, Default)]
#[serde_with::skip_serializing_none]
#[builder(setter(strip_option), default)]
pub struct SearchRequest {
    search: Option<Vec<String>>,
    year: Option<Vec<String>>,
    season_code: Option<Vec<String>>,
    genres: Option<Vec<String>>,
    team: Option<Vec<String>>,
    voice: Option<Vec<String>>,
    filter: Option<Vec<String>>,
    remove: Option<Vec<String>>,
    include: Option<Vec<String>>,
    description_type: Option<String>,
    playlist_type: Option<String>,
    limit: Option<i32>,
    after: Option<i32>,
    page: Option<i32>,
    items_per_page: Option<i32>,
}

#[derive(Deserialize)]
pub struct TitleResponse {
    id: i32,
    code: String,
    names: TitleResponseNames,
    posters: TitleResponsePosters,
    updated: i32,
    last_change: i32,
    status: TitleResponseStatus,
    genres: Vec<String>,
    team: TitleResponseTeam,
    season: TitleResponseSeason,
    year: Option<i32>,
    week_day: Option<i32>,
    description: String,
    blocked: TitleResponseBlocked,
    player: TitleResponsePlayer,
    torrents: TitleResponseTorrents,
}

#[derive(Deserialize)]
pub struct TitleResponseNames {
    ru: String,
    en: String,
    alternative: Option<String>,
}

#[derive(Deserialize)]
pub struct TitleResponsePosters {
    small: TitleResponsePoster,
    medium: TitleResponsePoster,
    original: TitleResponsePoster,
}

#[derive(Deserialize)]
pub struct TitleResponsePoster {
    url: String,
    raw_base64_file: Option<String>,
}

#[derive(Deserialize)]
pub struct TitleResponseStatus {
    string: String,
    code: i32,
}

#[derive(Deserialize)]
pub struct TitleResponseTeam {
    voice: Vec<String>,
    translator: Vec<String>,
    editing: Vec<String>,
    decor: Vec<String>,
    timing: Vec<String>,
}

#[derive(Deserialize)]
pub struct TitleResponseSeason {
    year: i32,
    week_day: i32,
    string: String,
    code: i32,
}

#[derive(Deserialize)]
pub struct TitleResponseBlocked {
    blocked: bool,
    bakanim: bool,
}

#[derive(Deserialize)]
pub struct TitleResponsePlayer {
    alternative_player: Option<String>,
    host: String,
    list: HashMap<String, TitleResponsePlayerList>,
    episodes: TitleResponsePlayerEpisodes,
}

#[derive(Deserialize)]
pub struct TitleResponsePlayerList {
    episode: f32,
    name: Option<String>,
    uuid: String,
    created_timestamp: i32,
    preview: Option<String>,
    skips: TitleResponsePlayerListSkips,
    hls: TitleResponsePlayerListHls,
}

#[derive(Deserialize)]
pub struct TitleResponsePlayerListSkips {
    opening: Vec<i32>,
    ending: Vec<i32>,
}

#[derive(Deserialize)]
pub struct TitleResponsePlayerListHls {
    fhd: Option<String>,
    hd: Option<String>,
    sd: Option<String>,
}

#[derive(Deserialize)]
pub struct TitleResponsePlayerEpisodes {
    string: String,
    first: i32,
    last: i32,
}

#[derive(Deserialize)]
pub struct TitleResponseTorrents {
    episodes: TitleResponseTorrentsEpisodes,
    list: Vec<TitleResponseTorrentsItem>,
}

#[derive(Deserialize)]
pub struct TitleResponseTorrentsEpisodes {
    string: String,
    first: i32,
    last: i32,
}

#[derive(Deserialize)]
pub struct TitleResponseTorrentsItem {
    torrent_id: i32,
    episodes: TitleResponseTorrentsItemEpisodes,
    quality: TitleResponseTorrentsItemQuality,
    leechers: i32,
    seeders: i32,
    downloads: i32,
    total_size: i64,
    url: String,
    magnet: String,
    uploaded_timestamp: i32,
    raw_base64_file: Option<String>,
    metadata: Option<TitleResponseTorrentsItemMetadata>,
}

#[derive(Deserialize)]
pub struct TitleResponseTorrentsItemEpisodes {
    string: String,
    first: i32,
    last: i32,
}

#[derive(Deserialize)]
pub struct TitleResponseTorrentsItemQuality {
    string: String,
    #[serde(rename = "type")]
    ttype: String,
    resolution: String,
    encoder: String,
    lq_audio: Option<bool>,
}

#[derive(Deserialize)]
pub struct TitleResponseTorrentsItemMetadata {
    hash: String,
    name: String,
    announce: Option<Vec<String>>,
    created_timestamp: i32,
    files_list: Vec<TitleResponseTorrentsItemMetadataFilesListItem>,
}

#[derive(Deserialize)]
pub struct TitleResponseTorrentsItemMetadataFilesListItem {
    file: String,
    size: i32,
    offset: i64,
}

#[derive(Deserialize)]
pub struct SearchResponse {
    list: Vec<TitleResponse>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("request serialization failed")]
    RequestSerializationFailed(#[from] serde_url_params::error::Error),
    #[error("url constructing failed")]
    UrlConstructingFailed(#[from] url::ParseError),
    #[error("request failed")]
    TransportFailed(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub async fn api_request_text<Req>(request: Req, route: &str) -> Result<String>
where
    Req: Serialize,
{
    let params = serde_url_params::to_string(&request)?;
    let url = format!("{API_ANILIBRIA_HOST}{route}?{params}");
    Ok(reqwest::get(Url::parse(&url)?).await?.text().await?)
}

pub async fn api_request<Req, Res>(request: Req, route: &str) -> Result<Res>
where
    Req: Serialize,
    Res: DeserializeOwned,
{
    let params = serde_url_params::to_string(&request)?;
    let url = format!("{API_ANILIBRIA_HOST}{route}?{params}");
    Ok(reqwest::get(Url::parse(&url)?).await?.json().await?)
}

pub async fn search_titles(request: SearchRequest) -> Result<SearchResponse> {
    api_request(request, "/v3/title/search").await
}
