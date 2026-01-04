use std::collections::BTreeMap;
use std::sync::Arc;

use rocket::serde::Serialize;
use tokio::sync::RwLock;

use crate::data::{Status, Ticket, TicketDraft, TicketPatch};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(transparent, crate = "rocket::serde")]
pub struct TicketId(u64);

impl From<u64> for TicketId {
    fn from(value: u64) -> Self {
        TicketId(value)
    }
}

impl From<TicketId> for u64 {
    fn from(value: TicketId) -> Self {
        value.0
    }
}

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<RwLock<Ticket>>>,
    counter: u64,
}

pub(crate) async fn apply_patch(handle: &Arc<RwLock<Ticket>>, patch: TicketPatch) {
    let mut ticket = handle.write().await;
    if let Some(title) = patch.title {
        ticket.title = title;
    }
    if let Some(description) = patch.description {
        ticket.description = description;
    }
    if let Some(status) = patch.status {
        ticket.status = status;
    }
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        let handle = Arc::new(RwLock::new(ticket));
        self.tickets.insert(id, handle);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<Arc<RwLock<Ticket>>> {
        self.tickets.get(&id).cloned()
    }

}

impl Default for TicketStore {
    fn default() -> Self {
        Self::new()
    }
}
