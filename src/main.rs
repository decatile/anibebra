use reqwest::{Response, Url};

#[tokio::main]
async fn main() {
    let resp = search_titles("nagatoro".into()).await.unwrap();
    println!("{}", resp.text().await.unwrap());
}

async fn search_titles(q: String) -> reqwest::Result<Response> {
    let url = Url::parse_with_params("http://api.anilibria.tv/v2/searchTitles", [("search", q)]).unwrap();
    let resp = reqwest::get(url).await?;
    Ok(resp)
}
