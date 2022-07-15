use std::path::Path;

use futures::{
    channel::mpsc::{channel, Receiver},
    executor::block_on,
    SinkExt,
};
use notify::{Config, Error, Event, RecommendedWatcher, RecursiveMode, Watcher};

use crate::{XError, XErrorKind, XResult};

impl From<Error> for XError {
    fn from(e: Error) -> Self {
        XError {
            kind: Box::new(XErrorKind::RuntimeError { message: e.to_string() }),
            path: None,
            position: None,
            source: Some(Box::new(e)),
        }
    }
}

pub fn file_watcher(path: &Path) -> XResult<Receiver<Result<Event, Error>>> {
    let config = Config::default().with_compare_contents(true);
    let (mut tx, receiver) = channel(1);
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            block_on(async {
                //
                tx.send(res).await.ok().unwrap_or_default()
            })
        },
        config,
    )?;
    watcher.watch(path, RecursiveMode::Recursive)?;
    Ok(receiver)
}
