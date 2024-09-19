use clap::{Arg, Command};
use reqwest::Client;
use serde::Deserialize;
use std::io::Write;
use tokio::time::{sleep, Duration, Instant};

#[derive(Deserialize, Debug)]
struct Meme {
    #[serde(rename = "postLink")]
    post_link: String,
    title: String,
    url: String,
    author: String,
}

async fn fetch_memes(client: &Client, amount: usize) -> Vec<Meme> {
    let mut memes = Vec::new();
    let mut fetched_count = 0;
    let start_time = Instant::now();

    for _ in 0..amount {
        let res = client
            .get("https://meme-api.com/gimme")
            .send()
            .await
            .expect("Failed to fetch meme")
            .json::<Meme>()
            .await
            .expect("Failed to parse meme");

        memes.push(res);
        fetched_count += 1;

        let elapsed = start_time.elapsed().as_secs_f64();
        let rate = fetched_count as f64 / elapsed;

        print!("\rFetched {}/{} meme{}... {:.2} memes/sec", fetched_count, amount, if fetched_count == 1 { "" } else { "s" }, rate);
        std::io::stdout().flush().expect("Failed to flush stdout");

        sleep(Duration::from_secs(1)).await;
    }

    println!("\rFetched a total of {} meme{} in {:.2} seconds. Override with the --amount (int) arg", fetched_count, if fetched_count == 1 { "" } else { "s" }, start_time.elapsed().as_secs_f64());
    println!();

    memes
}

#[tokio::main]
async fn main() {
    let matches = Command::new("MemeGen")
        .version("1.0.0")
        .author("Lncvrt")
        .about("Fetches memes from an API")
        .arg(
            Arg::new("amount")
                .short('a')
                .long("amount")
                .value_name("NUMBER")
                .help("Sets the number of memes to fetch")
                .default_value("3")
                .num_args(1)
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();

    let amount = matches.get_one::<usize>("amount").cloned().unwrap_or(3);

    let client = Client::new();

    println!(
        "Fetching {} meme{}...",
        amount,
        if amount == 1 { "" } else { "s" }
    );
    println!();

    let memes = fetch_memes(&client, amount).await;

    for meme in memes {
        println!("Title: {}", meme.title);
        println!("Author: {}", meme.author);
        println!("Link: {}", meme.post_link);
        println!("Meme URL: {}", meme.url);
        println!();
    }
}
