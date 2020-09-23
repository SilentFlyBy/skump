use anyhow::Result;
use rusqlite::{params, Connection, NO_PARAMS};

pub struct Chat {
    pub id: u32,
    pub name: String,
}

pub struct Message {
    pub convo_id: u32,
    pub author: String,
    pub body: String,
}

pub static GET_CONVERSATIONS: &'static str = "SELECT id, displayname FROM Conversations";
pub static GET_MESSAGES: &'static str =
    "SELECT from_dispname, timestamp, body_xml FROM Messages WHERE convo_id=?1";

pub fn get_conversations(db: &Connection) -> Result<Vec<Chat>> {
    let mut select = db.prepare(GET_CONVERSATIONS)?;

    let chats: Vec<Chat> = select
        .query_map(NO_PARAMS, |row| {
            Ok(Chat {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?
        .map(|c| {
            if c.is_ok() {
                return c.unwrap();
            } else {
                return Chat {
                    id: 0,
                    name: String::from(""),
                };
            }
        })
        .collect();

    Ok(chats)
}

pub fn get_messages(db: &Connection, id: u32) -> Result<Vec<Message>> {
    let mut select = db.prepare(GET_MESSAGES)?;
    let mut messages = Vec::new();
    let query = select.query_map(params![id], |row| {
        Ok(Message {
            convo_id: id,
            author: row.get(0)?,
            body: row.get(2)?,
        })
    })?;
    for q in query {
        if q.is_ok() {
            messages.push(q.unwrap());
        }
    }

    Ok(messages)
}
