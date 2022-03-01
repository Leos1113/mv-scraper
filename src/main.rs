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

    let response_text = res.text()?;

    let parsed_html = Html::parse_document(&response_text);

    let selector = &Selector::parse(".c-main > ul > li > a").unwrap();

    let forums = parsed_html.select(selector).collect::<Vec<_>>();

    let mut forum_links: Vec<String> = Vec::new();

    for forum in &forums {
        forum_links.push(forum.value().attr("href").unwrap().to_string());
    }

    if forums.len() > 0 {
        return Ok(forum_links);
    } else {
        Err("Error! Can't read the forums!!!!".into())
    }
}
