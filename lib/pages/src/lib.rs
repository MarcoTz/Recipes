use html::{elements::HtmlElement, html_document::HtmlDocument};

pub mod footer;
pub mod header;
pub mod html_head;
pub mod index;
pub mod recipe_details;
pub mod recipes;
pub mod tag_details;
pub mod tag_overview;

pub use index::Index;
pub use recipe_details::RecipeDetails;
pub use tag_details::TagDetails;
pub use tag_overview::TagOverview;

pub trait Page {
    fn render(self, date_format: &str) -> HtmlDocument;
}

pub trait PageComponent {
    fn render(self, date_format: &str) -> HtmlElement;
}

impl<T: PageComponent> PageComponent for Vec<T> {
    fn render(self, date_format: &str) -> HtmlElement {
        self.into_iter()
            .map(|item| item.render(date_format))
            .collect::<Vec<HtmlElement>>()
            .into()
    }
}
