use anyhow::Result;
use rusqlite::{params, Connection, Error, NO_PARAMS};

pub struct Chat {
    pub id: u32,
    pub name: String,
}

pub struct Message {
    pub convo_id: u32,
    pub author: String,
    pub display_name: String,
    pub body: String,
}

pub struct User {
    pub name: String,
}

pub static GET_CONVERSATIONS: &'static str = "SELECT id, displayname FROM Conversations";
pub static GET_MESSAGES: &'static str =
    "SELECT from_dispname, author, timestamp, body_xml FROM Messages WHERE convo_id=?1";
pub static GET_SELF_USER: &'static str = "SELECT skypename FROM Accounts";

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
            display_name: row.get(0)?,
            author: row.get(1)?,
            body: row.get(3)?,
        })
    })?;
    for q in query {
        if q.is_ok() {
            messages.push(q?);
        }
    }

    Ok(messages)
}

pub fn get_self_user(db: &Connection) -> Result<User, Error> {
    let mut select = db.prepare(GET_SELF_USER)?;
    let query = select.query_map(params![], |row| Ok(User { name: row.get(0)? }))?;
    let first = query.into_iter().nth(0).unwrap();
    first
}
