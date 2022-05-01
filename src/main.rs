#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate mysql;

use mongodb::Client;
use mongodb::options::ClientOptions;

mod user_web_service;
mod user_services;
mod user;

fn main() {

    rocket::ignite()
        .mount("/",
               routes![
               user_web_service::hello,
               user_web_service::hello_name
           ],
        )
        .launch();
}
