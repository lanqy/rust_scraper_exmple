use reqwest;
use std::fs::File;
use scraper::{Html, Selector};
use serde_json;

#[derive(serde::Serialize)]
struct ScrapedData {
    url: String,
    outer_text: String,
}

fn main() {
    let urls = vec![
        "https://colorkit.co/shades-of-red/",
        "https://colorkit.co/shades-of-orange/",
        "https://colorkit.co/shades-of-yellow/",
        "https://colorkit.co/shades-of-green/",
        "https://colorkit.co/shades-of-blue/",
        "https://colorkit.co/shades-of-pink/",
        "https://colorkit.co/shades-of-brown/",
        "https://colorkit.co/shades-of-black/",
    ];

    let mut scraped_data = Vec::new();

    #[tokio::main]
    async fn scrape_color_pill(url: &str, scraped_data: &mut Vec<ScrapedData>) -> Result<(), reqwest::Error> {
        // let response = reqwest::get(url).await?;
        let response = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .expect("send");
        println!("{:?}", response);
        let body = response.text().await?;
        let document = Html::parse_document(&body);
    
        let color_shade = Selector::parse(".color-pill").expect("Could not create selector.");
    
        for element in document.select(&color_shade) {
            let outer_text = element.text().collect::<String>();
    
            scraped_data.push(ScrapedData { url: url.to_string(), outer_text });
        }
    
        Ok(())
    }

    for url in urls {
        scrape_color_pill(&url, &mut scraped_data).unwrap();
    }

    // Save the scraped data to a JSON file
    let mut file = File::create("scraped_data.json").unwrap();
    serde_json::to_writer_pretty(&mut file, &scraped_data).unwrap();

    println!("Scraped data saved to scraped_data.json");
}
