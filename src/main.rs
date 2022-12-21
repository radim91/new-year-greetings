use std::path::Path;
use chrono::prelude::*;
use unsplash_api::endpoints::search_photos::SearchPhotos;
use http_api_client_endpoint::Endpoint;
use serde_json::Value;
use image::ImageFormat;
use random_string::generate;
use imageproc::drawing;
use rusttype::{Font, Scale};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut author: String = String::from("EAT CZECH");

    if &args.len() > &1 {
        author = String::new();
        for arg in &args[1..] {
            author.push_str(arg);
        }
    }

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
            let mut downloaded_image = image::load_from_memory(&res.unwrap()).unwrap();
            let random_string = generate(10, "abcdefghijklmnopqrstuvwxyz1234567890");
            let path_str = format!("./downloads/image-{}.png", random_string);
            let path = Path::new(&path_str);

            const TEXT_COLOR: image::Rgba<u8> = image::Rgba([255,255,255,8]);
            let scale_main: Scale = rusttype::Scale::uniform(80.0);
            let scale_sub: Scale = rusttype::Scale::uniform(40.0);
            let greeting_main = format!("HAPPY NEW YEAR {}", next_year);
            let greeting_sub = format!("from {}", author);
            let font: &Font = &rusttype::Font::try_from_bytes(std::include_bytes!("../fonts/Roboto-Black.ttf")).unwrap();

            drawing::draw_text_mut(&mut downloaded_image, TEXT_COLOR, 40, 40, scale_main, &font, &greeting_main);
            drawing::draw_text_mut(&mut downloaded_image, TEXT_COLOR, 40, 120, scale_sub, font, &greeting_sub);
            downloaded_image.save_with_format(path, ImageFormat::Png).unwrap();
        }
    }
}