#[macro_use] extern crate rocket;
use rocket::fs::{ FileServer, relative };
use rocket_dyn_templates::Template;

mod views;


#[launch]
pub fn rocket() -> _ {  // TODO: async

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
        .attach(Template::fairing()
    )
}


// use std::collections::HashMap;
// pub fn construct_page_hierarchy<'a>() -> HashMap<
//     u32, ( &'a str, HashMap<
//         u32, ( &'a str, HashMap<
//             u32, ( &'a str, &'a str )
//         >)
//     >)
// > {
//     HashMap::from([
//         (0, ("simulations", HashMap::from([
//             (0, ("nbody-interaction", HashMap::from([
//                 (0, ("2body-kepler", "laws of Kepler")),
//                 (1, ("3body-fig8", "3-body orbits: figure-8")),
//                 (2, ("3body-moon", "3-body orbits: Moon")),
//             ]))),
//             (1, ("cellular-automata", HashMap::from([
//                 (0, ("ising-model", "Ising model")),
//                 (0, ("heat-conduction", "heat conduction")),
//             ]))),
//             (2, ("graphs", HashMap::from([
//                 (0, ("mc-pi", "Monte Carlo pi approximation")),
//             ]))),
//         ]))),
//         (1, ("chronos", HashMap::from([

//         ]))),
//     ]) 
// }

// pub fn construct_inverse_hierarchy<'a>() -> HashMap<
//     &'a str, HashMap<
//         &'a str, HashMap<
//             &'a str, (u32, u32, u32)
//         >
//     >
// > {
//     let page_hierarchy = construct_page_hierarchy();
//     let mut out = HashMap::from([]);
//     for (topic_id, topic_content) in &page_hierarchy {
//         let topic_name = topic_content.0;
//         let mut out_cat = HashMap::from([]);
//         for (cat_id, cat_content) in &topic_content.1 {
//             let cat_name = cat_content.0;
//             let mut out_page = HashMap::from([]);
//             for (page_id, page_content) in &cat_content.1 {
//                 let page_name = page_content.0; 
//                 out_page.insert(
//                     page_name, 
//                     (*topic_id, *cat_id, *page_id)
//                 );
//             }
//             out_cat.insert(cat_name, out_page);
//         }
//         out.insert(topic_name, out_cat);
//     }
//     out
// }

// fn get_page_ids(
//     topic_name: &str, cat_name: &str, page_name: &str
// ) -> (u32, u32, u32) {
//     let hierarchy = construct_inverse_hierarchy();
//     hierarchy[topic_name][cat_name][page_name]
// }
// fn get_page_names<'a>(
//     topic_id: u32, cat_id: u32, page_id: u32
// ) -> (&'a str, &'a str, &'a str) {
//     let hierarchy = construct_page_hierarchy();

//     let topic_name = hierarchy[&topic_id].0;
//     let cat_name = hierarchy[&topic_id].1[&cat_id].0;
//     let page_name = hierarchy[&topic_id].1[&cat_id].1[&page_id].0;

//     (topic_name, cat_name, page_name)
// }


