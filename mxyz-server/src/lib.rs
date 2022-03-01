
#[macro_use] extern crate rocket;
use rocket::fs::{ FileServer, relative };
use rocket_dyn_templates::Template;

mod views;


#[rocket::main]
pub async fn server_init() -> Result<(), rocket::Error> {

    let file_server = FileServer::from(
        relative!("static")
    );
    let routes = routes![
        views::index::route,
        views::routes,
    ];

    rocket::build()
        .mount("/static", file_server)
        .mount("/", routes)
        .attach(Template::fairing())
        .launch()
        .await
}



// #[launch]
// pub fn rocket() -> _ {  // TODO: async

//     let file_server = FileServer::from(
//         relative!("static")
//     );
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

