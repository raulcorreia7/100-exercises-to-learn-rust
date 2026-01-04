use rocket::serde::{Deserialize, Serialize};

use crate::store::TicketId;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TicketPatch {
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<Status>,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", crate = "rocket::serde")]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
#[serde(transparent, crate = "rocket::serde")]
pub struct TicketTitle(String);

#[derive(Debug, thiserror::Error)]
pub enum TicketTitleError {
    #[error("The title cannot be empty")]
    Empty,
    #[error("The title cannot be longer than 50 bytes")]
    TooLong,
}

impl TicketTitle {
    pub fn new(value: String) -> Result<Self, TicketTitleError> {
        validate_title(&value)?;
        Ok(Self(value))
    }
}

fn validate_title(title: &str) -> Result<(), TicketTitleError> {
    if title.is_empty() {
        Err(TicketTitleError::Empty)
    } else if title.len() > 50 {
        Err(TicketTitleError::TooLong)
    } else {
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
#[serde(transparent, crate = "rocket::serde")]
pub struct TicketDescription(String);

#[derive(Debug, thiserror::Error)]
pub enum TicketDescriptionError {
    #[error("The description cannot be empty")]
    Empty,
    #[error("The description cannot be longer than 500 bytes")]
    TooLong,
}

impl TicketDescription {
    pub fn new(value: String) -> Result<Self, TicketDescriptionError> {
        validate_desc(&value)?;
        Ok(Self(value))
    }
}

fn validate_desc(description: &str) -> Result<(), TicketDescriptionError> {
    if description.is_empty() {
        Err(TicketDescriptionError::Empty)
    } else if description.len() > 500 {
        Err(TicketDescriptionError::TooLong)
    } else {
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TicketValidationError {
    #[error("Invalid title")]
    Title(TicketTitleError),
    #[error("Invalid description")]
    Description(TicketDescriptionError),
    #[error("Patch must include at least one field")]
    EmptyPatch,
}

impl TicketDraft {
    pub fn validate(&self) -> Result<(), TicketValidationError> {
        validate_title(&self.title.0).map_err(TicketValidationError::Title)?;
        validate_desc(&self.description.0).map_err(TicketValidationError::Description)?;
        Ok(())
    }
}

impl TicketPatch {
    pub fn validate(&self) -> Result<(), TicketValidationError> {
        if self.title.is_none() && self.description.is_none() && self.status.is_none() {
            return Err(TicketValidationError::EmptyPatch);
        }
        if let Some(title) = &self.title {
            validate_title(&title.0).map_err(TicketValidationError::Title)?;
        }
        if let Some(description) = &self.description {
            validate_desc(&description.0).map_err(TicketValidationError::Description)?;
        }
        Ok(())
    }
}
