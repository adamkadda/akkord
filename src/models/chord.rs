use crate::error::{Error, Result};

pub struct Chord {
    string_notes: Vec<String>,
    pub conciseness: usize,
}