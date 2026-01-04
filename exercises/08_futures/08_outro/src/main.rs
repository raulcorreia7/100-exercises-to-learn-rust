use tokio::sync::RwLock;

use outro_08::{create_ticket, get_ticket, index, patch_ticket, store::TicketStore};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let store = RwLock::new(TicketStore::new());
    rocket::build()
        .manage(store)
        .mount("/", routes![index, get_ticket, create_ticket, patch_ticket])
}
