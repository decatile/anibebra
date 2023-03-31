#![allow(dead_code)]

use std::{cmp, collections::HashMap, fmt::Display};

use derive_builder::Builder;
use make_fields_public::public;
use reqwest::Url;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

pub const API_HOST: &str = "http://api.anilibria.tv";

#[derive(Builder, Serialize, Default)]
#[public]
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
#[public]
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
#[public]
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
#[public]
pub struct TitleResponseNames {
    ru: String,
    en: String,
    alternative: Option<String>,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponsePosters {
    small: TitleResponsePoster,
    medium: TitleResponsePoster,
    original: TitleResponsePoster,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponsePoster {
    url: String,
    raw_base64_file: Option<String>,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponseStatus {
    string: String,
    code: i32,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponseTeam {
    voice: Vec<String>,
    translator: Vec<String>,
    editing: Vec<String>,
    decor: Vec<String>,
    timing: Vec<String>,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponseSeason {
    year: i32,
    week_day: i32,
    string: String,
    code: i32,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponseBlocked {
    blocked: bool,
    bakanim: bool,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponsePlayer {
    alternative_player: Option<String>,
    host: String,
    list: HashMap<String, TitleResponsePlayerList>,
    episodes: TitleResponsePlayerEpisodes,
}

#[derive(Deserialize)]
#[public]
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
#[public]
pub struct TitleResponsePlayerListSkips {
    opening: Vec<i32>,
    ending: Vec<i32>,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponsePlayerListHls {
    fhd: Option<String>,
    hd: Option<String>,
    sd: Option<String>,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponsePlayerEpisodes {
    string: String,
    first: i32,
    last: i32,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponseTorrents {
    episodes: TitleResponseTorrentsEpisodes,
    list: Vec<TitleResponseTorrentsItem>,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponseTorrentsEpisodes {
    string: String,
    first: i32,
    last: i32,
}

#[derive(Deserialize)]
#[public]
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
#[public]
pub struct TitleResponseTorrentsItemEpisodes {
    string: String,
    first: i32,
    last: i32,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponseTorrentsItemQuality {
    string: String,
    #[serde(rename = "type")]
    ttype: String,
    resolution: String,
    encoder: String,
    lq_audio: Option<bool>,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponseTorrentsItemMetadata {
    hash: String,
    name: String,
    announce: Option<Vec<String>>,
    created_timestamp: i32,
    files_list: Vec<TitleResponseTorrentsItemMetadataFilesListItem>,
}

#[derive(Deserialize)]
#[public]
pub struct TitleResponseTorrentsItemMetadataFilesListItem {
    file: String,
    size: i32,
    offset: i64,
}

#[derive(Deserialize)]
#[public]
pub struct SearchResponse {
    list: Vec<TitleResponse>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("request serialization failed")]
    RequestSerialization(#[from] serde_url_params::error::Error),
    #[error("url constructing failed")]
    UrlConstructing(#[from] url::ParseError),
    #[error("request failed")]
    Transport(#[from] reqwest::Error),
    #[error("{0}")]
    ResponseDeserialization(FormattedJsonDecodeError),
}

#[derive(Debug)]
#[public]
pub struct FormattedJsonDecodeError {
    inner: serde_json::Error,
    msg: String,
}

impl FormattedJsonDecodeError {
    #[allow(clippy::new_ret_no_self)]
    fn new(inner: serde_json::Error, source: String) -> Error {
        let index = inner.column() - 1;
        let loff = cmp::max(index as isize - 15, 0) as usize;
        let roff = index + 15;
        let mut space_count = 0usize;
        let window = source
            .split('\n')
            .nth(inner.line() - 1)
            .unwrap()
            .chars()
            .skip(loff)
            .take(roff - loff)
            .skip_while(|x| {
                let is_whitespace = x.is_whitespace();
                space_count += is_whitespace as usize;
                is_whitespace
            })
            .collect::<String>();
        let msg = format!(
            "{}\n{}^ {} here",
            window,
            " ".repeat(index - loff - space_count),
            inner
        );
        Error::ResponseDeserialization(FormattedJsonDecodeError { inner, msg })
    }
}

impl Display for FormattedJsonDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub async fn api_request_raw<Req>(route: &str, request: Req) -> Result<reqwest::Response>
where
    Req: Serialize,
{
    let params = serde_url_params::to_string(&request)?;
    let url = format!("{API_HOST}{route}?{params}");
    Ok(reqwest::get(Url::parse(&url)?).await?)
}

pub async fn api_request<Req, Res>(route: &str, request: Req) -> Result<Res>
where
    Req: Serialize,
    Res: DeserializeOwned,
{
    let resp = api_request_raw(route, request).await?;
    let text = resp.text().await?;
    serde_json::from_str(&text).map_err(|x| FormattedJsonDecodeError::new(x, text))
}

pub async fn search_titles(request: SearchRequest) -> Result<SearchResponse> {
    api_request("/v3/title/search", request).await
}

#[cfg(test)]
mod tests {
    use std::io::stdin;

    use super::FormattedJsonDecodeError;

    fn is_ok(string: String) -> bool {
        println!("{string}");
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        ["y", "1", ""].contains(&buf.trim())
    }

    #[test]
    fn test_json_error_display() {
        let json = r#"{"a": 10 xxx}"#;
        let err = serde_json::from_str::<serde_json::Value>(json)
            .map_err(|x| FormattedJsonDecodeError::new(x, json.to_string()))
            .unwrap_err();
        assert!(is_ok(err.to_string()));

        let json = r#"{
            "glossary": {
                "title": "example glossary",
                "GlossDiv": {
                    "title": "S",
                    "GlossList": {
                        "GlossEntry": {
                            "ID": x"SGML",
                            "SortAs": "SGML",
                            "GlossTerm": "Standard Generalized Markup Language",
                            "Acronym": "SGML",
                            "Abbrev": "ISO 8879:1986",
                            "GlossDef": {
                                "para": "A meta-markup language, used to create markup languages such as DocBook.",
                                "GlossSeeAlso": ["GML", "XML"]
                            },
                            "GlossSee": "markup"
                        }
                    }
                }
            }
        }"#;
        let err = serde_json::from_str::<serde_json::Value>(json)
            .map_err(|x| FormattedJsonDecodeError::new(x, json.to_string()))
            .unwrap_err();
        assert!(is_ok(err.to_string()));
    }
}
