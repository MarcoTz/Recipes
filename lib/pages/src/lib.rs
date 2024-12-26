use html::{elements::HtmlElement, html_document::HtmlDocument};

pub mod footer;
pub mod header;
pub mod html_head;
pub mod index;
pub mod recipe;
pub mod tag_details;
pub mod tag_overview;

pub trait Page {
    fn render(self, date_format: &str) -> HtmlDocument;
}

pub trait PageComponent {
    fn render(self, date_format: &str) -> HtmlElement;
}
