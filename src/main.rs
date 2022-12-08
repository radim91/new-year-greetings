use chrono::prelude::*;

fn main() {
    let next_year: i32 = Utc::now().year() + 1;
    let mut unsplash_api = String::from("https://api.unsplash.com/search/photos?client_id=JRzZBQwAW24--mdyrlbWwJfIxcuxfsCSlCQNKSDeN4E&query=");
    // add option to change query string
    unsplash_api.push_str("christmas");

    println!("{}", next_year);
    println!("{}", unsplash_api);
}
