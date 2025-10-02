#[tokio::main]
async fn main() {

    //Local runtime for executing !Send futures
    let local = tokio::task::LocalSet::new();

    //Channels for communication between local rt and regular rt
    let (url_tx,  mut url_rx) = tokio::sync::mpsc::channel(10);
    let (content_tx, content_rx) = tokio::sync::mpsc::channel(10);

    let local_rt_fut = local.run_until(async {
        tokio::task::spawn_local(async move {
            let content_tx_clone = content_tx.clone();
            loop {
                let content_tx_clone = content_tx_clone.clone();
                let url = match url_rx.recv().await {
                    Some(url) => url,
                    None => continue
                };
                tokio::task::spawn_local(async move {
                    rs_crawler::crawler::parse_web_page(content_tx_clone, url).await
                });
            }
        }).await.unwrap();
    });

    let crawler_fut = rs_crawler::crawler::init_crawler(url_tx, content_rx);

    tokio::join!(
        local_rt_fut,
        crawler_fut
    );
}
