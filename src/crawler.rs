use crate::content::WebContent;
use crate::utils::getting_input;

pub async fn init_crawler
(
    url_tx: tokio::sync::mpsc::Sender<String>,
    mut content_rx: tokio::sync::mpsc::Receiver<(String, WebContent)>
)
{
    let getting_input_fut = tokio::spawn(async move{
        getting_input(url_tx).await
    });

    let mut parsed_web_pages = std::collections::HashMap::new();

    let writer_fut = tokio::spawn(async move {
        loop {
            let (url, content) = match content_rx.recv().await {
                Some(content) => content,
                None => continue
            };

            println!("url: {} and its content: {:?}", url, content.h1_content);

            if !parsed_web_pages.contains_key(&url) {
                parsed_web_pages.insert(url, content);
            }
        }
    });

    let _ = tokio::join!(
        getting_input_fut,
        writer_fut
    );
}


pub async fn parse_web_page(
    content_tx: tokio::sync::mpsc::Sender<(String, WebContent)>,
    url: String)
{
    let response = match reqwest::get(url.clone()).await {
        Ok(res) => res,
        Err(e) => {
            println!("Error occurred while requesting web page... err: {:?}", e);
            return
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            println!("Error occurred while getting body of response...");
            return
        }
    };

    let document = scraper::Html::parse_document(&body);
    let a_selector = scraper::Selector::parse("a").expect("undefined selector...");
    let h1_selector = scraper::Selector::parse("h1").expect("undefined selector...");

    let a_content = document
        .select(&a_selector)
        .into_iter()
        .map(|a| {
            if let Some(href) = a.attr("href") {
                let content_tx_clone = content_tx.clone();
                let href = href.to_string();
                tokio::task::spawn_local(async move {
                    Box::pin(parse_web_page(content_tx_clone, href)).await
                });
            }
            a.inner_html()
        })
        .collect::<Vec<String>>();

    let h_content = document
        .select(&h1_selector)
        .into_iter()
        .map(|h| h.inner_html())
        .collect::<Vec<String>>();

    let web_content = WebContent::from((h_content, a_content));
    if let Err(_) = content_tx.send((url, web_content)).await {
        println!("Error occurred while sending parsed page to channel...")
    }
}