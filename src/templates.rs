use askama::Template;

use crate::models::chord::Chord;

#[derive(Template)]
#[template(path = "identifier.html")]
pub struct IdentifierTemplate;

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate;

#[derive(Template)]
#[template(path = "result.html")]
pub struct ResultTemplate {
    pub chords: Vec<Chord>
}