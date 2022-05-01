use crate::user;
use crate::user::PageView;
use rocket::response::{content, status};
use rocket::response::content::Json;
use rocket::response::status::Accepted;

pub fn get_users() -> content::Json<&'static str>
{
    let pool = mysql::Pool::new("mysql://root:password@localhost:3306/rust_db").unwrap();

    let all_persons: Vec<user::PageView> =
        pool.prep_exec("SELECT id, user, file_path, time from pageview", ())
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

    for person in all_persons.iter() {

        println!("{}, {}, {}, {}", person.id, person.user, person.file_path, person.time);
    }
    Json("{
                 'status': 'success',
                 'message': 'Hello API!'
             }")
}