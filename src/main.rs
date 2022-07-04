use serde::Deserialize;
use std::error::Error;
use colour::{dark_green, yellow};

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        dark_green!("> {}\n", a.title);
        yellow!("> {}\n\n", a.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url: &str =
        "https://newsapi.org/v2/top-headlines?country=us&apiKey=8ad8653ee9104169ae2adae5ba20c02f";
    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}
