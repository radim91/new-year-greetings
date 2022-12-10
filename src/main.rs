use chrono::prelude::*;
use unsplash_api::endpoints::search_photos::SearchPhotos;
use http_api_client_endpoint::Endpoint;
use serde_json::Value;

fn main() {
    let next_year: i32 = Utc::now().year() + 1;
    let unsplash_uri = SearchPhotos::new("JRzZBQwAW24--mdyrlbWwJfIxcuxfsCSlCQNKSDeN4E", "christmas")
        .render_request()
        .unwrap()
        .uri()
        .to_string()
    ;
    let body = reqwest::blocking::get(unsplash_uri).unwrap().text();
    let deserialized: Value = serde_json::from_str(&body.unwrap()).unwrap();
    let results = deserialized.get("results").unwrap();

    if results.is_array() {
        for item in results.as_array().unwrap() {
            //TODO: get regular url, download image, save it
            dbg!(item.get("urls"));
        }
    }

    println!("{}", next_year);
}