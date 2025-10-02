use crate::utils::getting_input;

async fn init_crawler() {
    let (url_tx,  url_rx) = tokio::sync::mpsc::channel(10);

    let url_tx_clone = url_tx.clone();
    let getting_input_fut = tokio::spawn(async move{
        getting_input(url_tx_clone).await
    });

    let processing_url_fut = tokio::spawn(async move {

    });

}