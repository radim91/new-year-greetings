use chrono::prelude::*;

fn main() {
    let next_year: i32 = Utc::now().year() + 1;
    print!("{}", next_year);
}
