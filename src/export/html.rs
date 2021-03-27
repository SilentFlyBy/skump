use crate::{db, templates};
use anyhow::Result;
use handlebars::{Handlebars, TemplateRenderError};
use serde::{Serialize, Serializer};

enum HtmlMessageType {
    Me,
    Them,
}

impl Serialize for HtmlMessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            HtmlMessageType::Me => "me",
            HtmlMessageType::Them => "them",
        })
    }
}

#[derive(Serialize)]
struct HtmlMessage {
    message_type: HtmlMessageType,
    text: String,
}
#[derive(Serialize)]
struct Context {
    messages: Vec<HtmlMessage>,
}

pub fn render_html(
    messages: Vec<db::Message>,
    user: db::User,
) -> Result<String, TemplateRenderError> {
    let chat_template = templates::TEMPLATES_DIR.get_file("chat.html").unwrap();
    let context_messages: Vec<HtmlMessage> = messages
        .into_iter()
        .map(|m| HtmlMessage {
            message_type: if m.author == user.name {
                HtmlMessageType::Me
            } else {
                HtmlMessageType::Them
            },
            text: m.body,
        })
        .collect();

    let ctx = Context {
        messages: context_messages,
    };

    let hbs = Handlebars::new();
    hbs.render_template(chat_template.contents_utf8().unwrap(), &ctx)
}

pub fn to_html(messages: Vec<db::Message>, user: db::User) -> Result<()> {
    let html = render_html(messages, user).unwrap();
    print!("{}", html);
    Ok(())
}
