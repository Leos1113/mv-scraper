use scraper::{Html, Selector};

fn main() {
    let client = reqwest::blocking::Client::new();

    get_forums(client);
    // let origin_url = "https://www.mediavida.com/foro/";
    // let res = client.get(origin_url).send().unwrap();

    // let response_text = res.text().unwrap();
    // //println!("Status for {}: {}", origin_url, response_text);

    // let parsed_html = Html::parse_document(&response_text);

    // //println!("{:?}", parsed_html);

    // let selector = &Selector::parse(".c-main > ul").unwrap();

    // let forums = parsed_html.select(selector).collect::<Vec<_>>();

    // println!("forums retrieved {:?} ", forums);
}

fn get_forums(client: reqwest::blocking::Client) {
    let origin_url = "https://www.mediavida.com/foro/";

    let res = client.get(origin_url).send().unwrap();

    let response_text = res.text().unwrap();

    let parsed_html = Html::parse_document(&response_text);

    //println!("{:?}", parsed_html);

    let selector = &Selector::parse(".c-main > ul").unwrap();

    let forums = parsed_html.select(selector).collect::<Vec<_>>();

    println!("forums retrieved {:?} ", forums);
}
