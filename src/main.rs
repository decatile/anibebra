mod api;

use api::anilibria;

#[tokio::main]
async fn main() {
    let _resp = anilibria::api_request::<anilibria::SearchRequest, anilibria::SearchResponse>(
        anilibria::SearchRequestBuilder::default()
            .search(vec!["nagatoro".into()])
            .build()
            .unwrap(),
        "/v3/title/search",
    )
    .await
    .unwrap();
}
