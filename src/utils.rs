use tokio::io::AsyncBufReadExt;

async fn process_input(tx: tokio::sync::mpsc::Sender<String>) {
    let mut buf = tokio::io::BufReader::new(tokio::io::stdin());
    loop {
        let mut url = String::new();
        if let Err(_) = buf.read_line(&mut url).await {
            println!("Error occur while reading url...");
            continue
        }
        if let Err(_) = tx.send(url).await {
            break
        }
    }
}