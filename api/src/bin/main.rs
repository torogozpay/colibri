/*use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;*/


use api::invoice_handler;

use rocket::launch;
use rocket::routes;

#[launch]
pub fn rocket() -> _ {    
    rocket::build()        
        //.mount("/public", FileServer::from(relative!("static")))
        .mount("/",
            routes![
               // invoice_handler::index,
                invoice_handler::create_invoice,
                invoice_handler::lookup_invoice
            ],
        )
        //.attach(Template::fairing())
        

    
       // 
        
        
}