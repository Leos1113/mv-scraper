use scraper::{Html, Selector};
use std::error::Error;

fn main() {
    let client = reqwest::blocking::Client::new();

    let forums_links = match get_forums(client) {
        Ok(links) => println!("{:?}", links),
        Err(e) => {
            println!("Can't get the list of links of the forums! {}", e);
        }
    };
}

fn get_forums(client: reqwest::blocking::Client) -> Result<Vec<String>, Box<dyn Error>> {
    let origin_url = "https://www.mediavida.com/foro/";

    let res = client.get(origin_url).send()?;

    let parsed_html = Html::parse_document(&res.text()?);

    let selector = &Selector::parse(".c-main > ul > li > a").unwrap();

    let forums = parsed_html.select(selector).collect::<Vec<_>>();

    let forum_links: Vec<String> = forums
        .iter()
        .map(|element| element.value().attr("href").unwrap().to_string())
        .collect::<Vec<String>>();

    if !forums.is_empty() {
        Ok(forum_links)
    } else {
        Err("Error! Can't read the forums!!!!".into())
    }
}
