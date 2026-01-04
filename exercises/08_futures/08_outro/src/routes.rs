use rocket::http::Status;
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;
use rocket::{get, patch, post, State};
use tokio::sync::RwLock;

use crate::data::{Ticket, TicketDraft, TicketPatch};
use crate::store::{apply_patch, TicketId, TicketStore};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, World"
}

#[get("/ticketstore/<id>")]
pub async fn get_ticket(
    store_state: &State<RwLock<TicketStore>>,
    id: u64,
) -> Result<Json<Ticket>, Status> {
    let ticket_handle = {
        let store = store_state.read().await;
        store.get(id.into()).ok_or(Status::NotFound)?
    };
    let ticket = ticket_handle.read().await.clone();
    Ok(Json(ticket))
}

#[post("/ticketstore", format = "json", data = "<draft>")]
pub async fn create_ticket(
    store_state: &State<RwLock<TicketStore>>,
    draft: Json<TicketDraft>,
) -> Result<Created<Json<TicketId>>, Status> {
    let draft = draft.into_inner();
    draft.validate().map_err(|_| Status::BadRequest)?;
    let mut store = store_state.write().await;
    let id = store.add_ticket(draft);

    let location = rocket::uri!(get_ticket(id = u64::from(id))).to_string();
    Ok(Created::new(location).body(Json(id)))
}

#[patch("/ticketstore/<id>", format = "json", data = "<patch>")]
pub async fn patch_ticket(
    store_state: &State<RwLock<TicketStore>>,
    id: u64,
    patch: Json<TicketPatch>,
) -> Result<NoContent, Status> {
    let patch = patch.into_inner();
    patch.validate().map_err(|_| Status::BadRequest)?;
    let ticket_handle = {
        let store = store_state.read().await;
        store.get(id.into())
    }
    .ok_or(Status::NotFound)?;
    apply_patch(&ticket_handle, patch).await;
    Ok(NoContent)
}
