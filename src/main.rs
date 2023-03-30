mod api;

use api::anilibria;

#[tokio::main]
async fn main() {
    let resp = anilibria::search_titles(
        anilibria::SearchRequestBuilder::default()
            .search(vec!["nagatoro".into()])
            .build()
            .unwrap(),
    )
    .await;
    if let Err(why) = resp {
        println!("{why}");
    }
}
