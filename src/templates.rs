use askama::Template;

use crate::models::chord::Chord;

#[derive(Template)]
#[template(path = "result.html")]
pub struct ResultTemplate {
    pub chords: Vec<Chord>
}