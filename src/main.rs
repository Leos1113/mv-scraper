use scraper::Html;

fn main() {
    let client = reqwest::blocking::Client::new();
    let origin_url = "https://rolisz.ro/";
    let res = client.get(origin_url).send().unwrap();

    let response_text = res.text().unwrap();
    println!("Status for {}: {}", origin_url, response_text);
    

    let parsed_html = Html::parse_document(&response_text);

    println!("{:?}", parsed_html);
}
