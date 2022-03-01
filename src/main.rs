
fn main() {
    let client = reqwest::blocking::Client::new();
    let origin_url = "https://rolisz.ro/";
    let res = client.get(origin_url).send().unwrap();
    println!("Status for {}: {}", origin_url, res.text().unwrap());
}
