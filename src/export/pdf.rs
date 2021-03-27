use crate::db;
use anyhow::Result;
use wkhtmltopdf::*;

pub fn to_pdf(messages: Vec<db::Message>, user: db::User) -> Result<()> {
    let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app.builder();
    Ok(())
}
