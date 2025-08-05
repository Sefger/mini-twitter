mod models;
use chrono::{ NaiveDate};
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
struct User{
    name: String,
    surname: String,
    nickname: String,
    birthday_date: NaiveDate,
    status: String
    //friends: Vec<User>
    //subscriber: Vec<user>
}
fn main() {
    println!("Hello, world!");
}
