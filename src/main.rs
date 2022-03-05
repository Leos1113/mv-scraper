use bson::Document;
use chrono::{TimeZone, Utc};
use dotenv::dotenv;
use futures::stream::TryStreamExt;
use futures::Stream;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client as MongoClient,
};
use reqwest::{Client, Response, StatusCode};
use scraper::{Html, Selector};
use std::{env, error::Error};

mod entities;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let mongo_client = MongoClient::with_options(options)?;

    let client: Client = Client::new();

    let forums: Vec<entities::forum::Forum> = get_forums_info(client).await.unwrap();
    let db = mongo_client.database("mediavida");

    for forum in forums {
        let new_forum = doc! {
            "title": forum.title.clone(),
            "link": forum.link,
            "description": forum.description,
            "scraped": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
        };

        let collection = db.collection::<Document>("forums");

        let filter = doc! { "title": forum.title };

        let mut cursor = collection.find(filter, None).await?;

        if let Some(_doc) = cursor.try_next().await? {
            continue;
        }

        let insert_result = collection.insert_one(new_forum.clone(), None).await?;
        println!("New document ID: {}", insert_result.inserted_id);
    }

    Ok(())
}

async fn get_forums_info(client: Client) -> Result<Vec<entities::forum::Forum>, Box<dyn Error>> {
    let url: &str = "https://www.mediavida.com/foro/";

    let result: Response = client.get(url).send().await.unwrap();

    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong"),
    };

    let parsed_html = Html::parse_document(&raw_html);

    let link_selector = &Selector::parse(".c-main > ul > li > a").unwrap();
    let title_selector = &Selector::parse(".info-col > strong").unwrap();
    let description_selector = &Selector::parse(".info-col > p").unwrap();

    let mut forums: Vec<entities::forum::Forum> = Vec::new();
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

        forums.push(entities::forum::Forum::new(title, link, description));
    }

    if !forums.is_empty() {
        Ok(forums)
    } else {
        Err("Error! Can't read the forums!!!!".into())
    }
}
