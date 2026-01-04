use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use tokio::sync::RwLock;

use outro_08::{create_ticket, get_ticket, index, patch_ticket, store::TicketStore};

fn rocket() -> rocket::Rocket<rocket::Build> {
    let store = RwLock::new(TicketStore::new());
    rocket::build()
        .manage(store)
        .mount("/", rocket::routes![index, get_ticket, create_ticket, patch_ticket])
}

#[rocket::async_test]
async fn endpoints_happy_path() {
    let client = Client::tracked(rocket())
        .await
        .expect("valid rocket instance");

    let response = client.get("/").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().await.expect("index body");
    assert_eq!(body, "Hello, World");

    let response = client
        .post("/ticketstore")
        .header(ContentType::JSON)
        .body(r#"{"title":"First","description":"Desc"}"#)
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Created);
    let id_body = response
        .into_string()
        .await
        .expect("create response body");
    assert_eq!(id_body.trim(), "0");

    let response = client.get("/ticketstore/0").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().await.expect("get body");
    assert_eq!(
        body,
        r#"{"id":0,"title":"First","description":"Desc","status":"todo"}"#
    );

    let response = client
        .patch("/ticketstore/0")
        .header(ContentType::JSON)
        .body(r#"{"status":"done"}"#)
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NoContent);

    let response = client
        .patch("/ticketstore/0")
        .header(ContentType::JSON)
        .body(r#"{}"#)
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::BadRequest);
}
