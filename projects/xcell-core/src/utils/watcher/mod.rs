use futures::executor::block_on;
use super::*;

#[tokio::test]
async fn main() {
    let rott = PathBuf::from("./");
    println!("{}", rott.display());

    if let Err(e) = async_watch(&rott).await {
        println!("error: {:?}", e)
    }
}



async fn async_watch(path: &Path) -> XResult<()> {
    let (mut watcher, mut rx) = async_watcher().unwrap();
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    while let Some(res) = rx.next().await {
        println!("changed: {:?}", res)
    }

    Ok(())
}
