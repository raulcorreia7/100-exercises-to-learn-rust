// TODO: Use two variants, one for a title error and one for a description error.
//   Each variant should contain a string with the explanation of what went wrong exactly.
//   You'll have to update the implementation of `Ticket::new` as well.
enum TicketNewError {
    TitleEmpty,
    TitleLonger50Bytes,
    DescriptionEmpty,
    DescriptionLonger50Bytes,
}

fn error_msg(te: TicketNewError) -> String {
    match &te {
        TicketNewError::TitleEmpty => "Title cannot be empty".to_string(),
        TicketNewError::TitleLonger50Bytes => "Title cannot be longer than 50 bytes".to_string(),
        TicketNewError::DescriptionEmpty => "Description cannot be empty".to_string(),
        TicketNewError::DescriptionLonger50Bytes => {
            "Description cannot be longer than 50 bytes".to_string()
        }
    }
}
// TODO: `easy_ticket` should panic when the title is invalid, using the error message
//   stored inside the relevant variant of the `TicketNewError` enum.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description.clone(), status.clone()) {
        Ok(t) => t,
        Err(e) => match e {
            TicketNewError::TitleEmpty | TicketNewError::TitleLonger50Bytes => {
                panic!("{}", error_msg(e))
            }
            TicketNewError::DescriptionEmpty | TicketNewError::DescriptionLonger50Bytes => {
                Ticket::new(title, "Description not provided".to_string(), status)
                    .unwrap_or_else(|msg| panic!("{}", error_msg(msg)))
            }
        },
    }
}

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleLonger50Bytes);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionLonger50Bytes);
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
