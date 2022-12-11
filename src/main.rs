use chrono::prelude::*;
use unsplash_api::endpoints::search_photos::SearchPhotos;
use http_api_client_endpoint::Endpoint;
use serde_json::Value;
use image::ImageFormat;
use random_string::generate;

fn main() {
    let next_year: i32 = Utc::now().year() + 1;
    let unsplash_uri = SearchPhotos::new("JRzZBQwAW24--mdyrlbWwJfIxcuxfsCSlCQNKSDeN4E", "new year")
        .render_request()
        .unwrap()
        .uri()
        .to_string()
    ;
    let body = reqwest::blocking::get(unsplash_uri).unwrap().text().unwrap();
    let deserialized: Value = serde_json::from_str(&body).unwrap();
    let results = deserialized.get("results").unwrap();

    if results.is_array() {
        for item in results.as_array().unwrap() {
            let mut link = item.get("urls").unwrap().get("regular").unwrap().to_string();

            if link.len() > 0 {
                link.remove(0);
                link.pop();
            }

            let res = reqwest::blocking::get(&link).unwrap().bytes();
            let downloaded_image = image::load_from_memory(&res.unwrap()).unwrap();
            let random_string = generate(10, "abcdefghijklmnopqrstuvwxyz1234567890");
            let path = format!("./downloads/image-{}.png", random_string);

            downloaded_image.save_with_format(path, ImageFormat::Png);
        }
    }

    println!("{}", next_year);
}