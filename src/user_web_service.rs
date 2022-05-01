extern crate rocket;

use mysql::Pool;
use crate::{user};
use crate::user::PageView;
use mysql::*;
use mysql::prelude::*;
use actix_web::{App, HttpServer};

#[get("/user/<id>")]
pub fn hello(id : i64) -> String
{
    let pool: Pool = mysql::Pool::new("mysql://root:password@localhost:3306/rust_db").unwrap();

    let all_persons: Vec<user::PageView> =
        pool.prep_exec(format!("SELECT id, user, file_path, time from pageview WHERE id = '{}'", id), ())
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (id, user, file_path, time) = mysql::from_row(row);
                    user::PageView {
                        id,
                        user,
                        file_path,
                        time,
                    }
                }).collect()
            }).unwrap(); // Unwrap `Vec<Person>`
    format!("id: {}, user: {}, file path: {}, time: {}", all_persons[0].id, all_persons[0].user, all_persons[0].file_path, all_persons[0].time)
}

#[post("/users?<user>&<file_path>")]
pub unsafe fn hello_name(user: String, file_path: String) -> String
{
    let pool: Pool = mysql::Pool::new("mysql://root:password@localhost:3306/rust_db").unwrap();
    let all_persons = pool.prep_exec(format!("insert into PageView (user, file_path, time) VALUES ('{}', '{}', CURRENT_DATE());", user, file_path), ());
    format!("Added User, {:?}!", all_persons)
}