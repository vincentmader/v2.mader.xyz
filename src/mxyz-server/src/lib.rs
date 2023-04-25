#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::{Ignite, Rocket};
use rocket_dyn_templates::Template;

mod views;

#[rocket::main]
pub async fn main() -> Result<Rocket<Ignite>, rocket::Error> {
    let file_server = FileServer::from(relative!("static"));
    let routes = routes![views::index::route, views::routes,];

    rocket::build()
        .mount("/static", file_server)
        .mount("/", routes)
        .attach(Template::fairing())
        .launch()
        .await
}

// use rocket::{Rocket, Build};

// #[launch]
// pub fn rocket_launch() -> Rocket<Build> {

//     let file_server = FileServer::from(relative!("static"));
//     let routes = routes![
//         views::index::route,
//         views::routes,
//     ];

//     rocket::build()
//         .mount("/static", file_server)
//         .mount("/", routes)
//         .attach(Template::fairing())
//         // .launch()
//         // .await
// }

// #[launch]
// pub fn rocket() -> _ {  // TODO: async

//     let file_server = FileServer::from(relative!("static"));
//     let routes = routes![
//         views::index::route,
//         views::routes,
//     ];

//     rocket::build()
//         .mount("/static", file_server)
//         .mount("/", routes)
//         .attach(Template::fairing()
//     )
// }
