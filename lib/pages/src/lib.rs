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

pub struct RenderParameters {
    pub date_format: String,
    pub depth: usize,
}

impl RenderParameters {
    pub fn get_link_prefix(&self) -> String {
        let mut prefix = "../".to_owned();
        for _ in 1..=self.depth {
            prefix += "../";
        }
        prefix.to_owned()
    }
}

impl Default for RenderParameters {
    fn default() -> RenderParameters {
        RenderParameters {
            date_format: "%d.%m.%Y".to_owned(),
            depth: 0,
        }
    }
}

pub trait Page {
    fn render(self, params: &mut RenderParameters) -> HtmlDocument;
}

pub trait PageComponent {
    fn render(self, params: &mut RenderParameters) -> HtmlElement;
}

impl<T: PageComponent> PageComponent for Vec<T> {
    fn render(self, params: &mut RenderParameters) -> HtmlElement {
        self.into_iter()
            .map(|item| item.render(params))
            .collect::<Vec<HtmlElement>>()
            .into()
    }
}
