use std::path::Path;
use crate::XResult;

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);
    let watcher = RecommendedWatcher::new(
        move |res| {
            block_on(async {
                tx.send(res).await.ok()
            })
        },
        Config::default(),
    ).unwrap();
    Ok((watcher, rx))
}


async fn async_watch(path: &Path) -> XResult<()> {
    let (mut watcher, mut rx) = async_watcher()?;
    let channal = watcher.watch(path, RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        println!("changed: {:?}", res)
    }

    Ok(())
}
