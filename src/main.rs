mod api;

use api::anilibria;

#[tokio::main]
async fn main() {
    let _resp = anilibria::search_titles(
        anilibria::SearchRequestBuilder::default()
            .search(vec!["nagatoro".into()])
            .build()
            .unwrap(),
    )
    .await
    .unwrap();
}
