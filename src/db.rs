pub static GET_CONVERSATIONS: &'static str = "SELECT id, displayname FROM Conversations";
pub static GET_MESSAGES: &'static str =
    "SELECT from_dispname, timestamp, body_xml FROM Messages WHERE convo_id=?1";

pub fn select() {}
