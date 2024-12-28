use html::{elements::HtmlElement, html_document::HtmlDocument};

pub mod footer;
pub mod header;
pub mod html_head;
pub mod index;
pub mod recipe;
pub mod tag_details;
pub mod tag_overview;

pub use index::Index;
pub use recipe::RecipeDetails;
pub use tag_details::TagDetails;
pub use tag_overview::TagOverview;

pub trait Page {
    fn render(self, date_format: &str) -> HtmlDocument;
}

pub trait PageComponent {
    fn render(self, date_format: &str) -> HtmlElement;
}

/*:root {
  --bg-color:#060E1A;
  --bg-color2:#292F4D;
  --bg-color3:#171E2B;
  --fg-color:#9290C3;
  --link-color:#B3B1FC;
}

body {
  background:var(--bg-color);
  color:var(--fg-color);
  font-family: "Noto Sans", sans-serif;
  font-size:14pt;
}

ul {
  list-style-type:none;
}
li {
  margin:0.5em;
}
li:nth-child(odd) {
  background:var(--bg-color3);
}
li:nth-child(even){
  background:var(--bg-color2);
}

a {
  color:var(--link-color);
}

h1 {
  text-align:center;
  font-size:26pt;
  width:100%;
}
h2 {
  text-align:center;
  font-size:24pt;
  width:100%;
}

input {
  background:var(--bg-color2);
  color:var(--fg-color);
}

select {
  background:var(--bg-color2);
  color:var(--fg-color);
}

#footer {
  margin-top:2em;
  margin-left:auto;
  margin-right:auto;
  width:90%;
  height:10%;
}
#header {
  display:flex;
  justify-self:center;
  align-content:space-around;
  justify-content:space-around;
  gap:1em;
}
#header > * {
  border-radius:1em;
  padding:1em;
}
#header > *:nth-child(even) {
  background:var(--bg-color3);
}
#header > *:nth-child(odd){
  background:var(--bg-color2);
}

.level1 {
  display:flex;
  flex-wrap:wrap;
  align-items:flex-start;
}
#ingredients {
  width:45%;
  margin-left:2em;
}

#steps {
  width:45%;
  margin-right:2em;
}

#notes {
  margin:auto;
  width:95%;
}

#recipe_images {
  width:100%;
  display:flex;

}
.recipe_image{
  margin:auto;
  height:15%;
  width: 15%;
  padding:2em;
}
img {
  height:100%;
  width:100%;
}

#header > *, #footer > *{
  display:inline;
  margin-left:0.5em;
  margin-right:0.5em;
}

#searchbox {
  float:right;
  padding-top:1em;
  padding-bottom:1em;
}

#recipe_list_sort, #tags_list_sort{
  float:right;
  margin-right:2em;
  padding-left:1em;
  padding-top:1em;
  padding-bottom:1em;
}

#created_date {
  float:right;
}
#num_recipes{
  float:right;
}
#github_link{
  float:left;
}

#recipe_list, #tags_list {
  display:flex;
  flex-wrap:wrap;
  align-content:flex-start;
  justify-content: space-around;
  justify-self:center;
  gap:1em;
  margin:1em;
  width:95%;
}

.recipe_letter {
  background:var(--bg-color3);
  padding-left:1em;
  border-radius:1em;
  height:2em;
  font-size:16pt;
  width:100%;
  margin-top:0.5em;
  margin-bottom:0.5em;
}
.tag_item:nth-child(even){
  background:var(--bg-color3);
}
.tag_item:nth-child(odd){
  background:var(--bg-color2);
}
.tag_item{
  border-radius:1em;
  padding:0.5em;
}
.recipe_item {
  border-radius:1em;
  padding:0.5em;
  text-align:center;
  height:3em;
}
.recipe_item:nth-child(odd) {
  background:var(--bg-color2);
}
.recipe_item:nth-child(even) {
  background:var(--bg-color3);
}
.recipe_taglist {
  text-align:left;
  font-size: 8pt;
}

#tags > h2 {
  font-size:12pt;
  text-align:left;
}
#tags{
  text-align:left;
  font-size: 12pt;
  width:100%;
  margin:2em;
}


.spacer {
  width:100%;
}

.hline {
  width:100%;
  height:0.5em;
  background-color:var(--bg-color3);
}
*/
