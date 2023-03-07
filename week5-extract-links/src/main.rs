use std::env;
use scraper::{Html, Selector};
use reqwest::blocking::Client;

fn main() {
    // Get the URL from the command-line input
    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    // Make an HTTP request and get the response body
    let client = Client::new();
    let response = client.get(url).send().unwrap();
    let body = response.text().unwrap();

    // Parse the HTML document
    let document = Html::parse_document(&body);

    // Define the selector to find links
    let link_selector = Selector::parse("a").unwrap();

    // Find all links and print their URLs
    for link in document.select(&link_selector) {
        if let Some(href) = link.value().attr("href") {
            println!("{}", href);
        }
    }
}

// use error_chain::error_chain;
// use select::document::Document;
// use select::predicate::Name;

// error_chain! {
//     foreign_links {
//         ReqError(reqwest::Error);
//         IoError(std::io::Error);
//     }
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let res = reqwest::get("https://www.rust-lang.org/learn")
//     .await?
//     .text()
//     .await?;

// Document::from(res.as_str())
// .find(Name("a"))
// .filter_map(|n| n.attr("href"))
// .for_each(|x| println!("{}", x));

// Ok(())
// }