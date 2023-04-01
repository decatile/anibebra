use std::sync::Arc;

use crossterm::event::{Event, EventStream};
use futures::{stream::FuturesUnordered, FutureExt, StreamExt};
use tokio::sync::Mutex;

use crate::{
    api::anilibria::{self, TitleRequestBuilder, TitleResponse},
    config::Config,
};

pub async fn ui(conf: Arc<Mutex<Config>>) {
    let _state = State::Loading;
    render();
    let _titles = request_watching_titles(
        conf.lock()
            .map(|x| x.watching.iter().map(|x| x.title_id).collect())
            .await,
    )
    .await;
    let mut reader = EventStream::new();
    loop {
        render();
        let event = reader.next().fuse().await;
        match event {
            Some(Ok(event)) => match event {
                Event::Key(_) => {}
                Event::Mouse(_) => todo!(),
                Event::Paste(_) => todo!(),
                Event::Resize(_, _) => todo!(),
                _ => {}
            },
            Some(Err(err)) => {
                render_error(err.to_string());
                break;
            }
            _ => break,
        }
    }
}

fn render() {
    todo!()
}

fn render_error(_string: String) {}

async fn request_watching_titles(title_ids: Vec<i32>) -> anilibria::Result<Vec<TitleResponse>> {
    let size = title_ids.len();
    let tasks = title_ids
        .into_iter()
        .map(|x| {
            tokio::spawn(anilibria::get_title(
                TitleRequestBuilder::default().id(x).build().unwrap(),
            ))
        })
        .collect::<FuturesUnordered<_>>();
    futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(Result::unwrap)
        .fold(Ok(Vec::with_capacity(size)), |a, e| {
            a.and_then(|mut x| {
                e.map(|y| {
                    x.push(y);
                    x
                })
            })
        })
}

enum State {
    Loading,
}
