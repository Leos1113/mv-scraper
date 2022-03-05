use reqwest::{blocking::Client, blocking::Response, StatusCode};
use scraper::{Html, Selector};
use std::error::Error;

#[derive(Debug)]
struct Forum {
    title: String,
    link: String,
    description: String,
}

fn main() {
    let client: Client = Client::new();

    let forums: Vec<Forum> = get_forums_info(client).unwrap();

    println!("{:?}", forums);
}

fn get_forums_info(client: Client) -> Result<Vec<Forum>, Box<dyn Error>> {
    let url: &str = "https://www.mediavida.com/foro/";

    let result: Response = client.get(url).send().unwrap();

    let raw_html = match result.status() {
        StatusCode::OK => result.text().unwrap(),
        _ => panic!("Something went wrong"),
    };

    let parsed_html = Html::parse_document(&raw_html);

    let link_selector = &Selector::parse(".c-main > ul > li > a").unwrap();
    let title_selector = &Selector::parse(".info-col > strong").unwrap();
    let description_selector = &Selector::parse(".info-col > p").unwrap();

    let mut forums: Vec<Forum> = Vec::new();
    for element in parsed_html.select(&link_selector) {
        let link = element.value().attr("href").unwrap().to_string();
        let mut title_element = element.select(&title_selector);
        let mut decription_element = element.select(&description_selector);

        let mut title = String::new();
        let mut description = String::new();

        match title_element.next() {
            Some(element_ref) => {
                title = element_ref.inner_html().to_string();
            }
            _ => {}
        }

        match decription_element.next() {
            Some(element_ref) => {
                description = element_ref.inner_html().to_string();
            }
            _ => {}
        }

        forums.push(Forum {
            link: link,
            title: title,
            description: description,
        });
    }

    if !forums.is_empty() {
        Ok(forums)
    } else {
        Err("Error! Can't read the forums!!!!".into())
    }
}
