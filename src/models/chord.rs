use crate::error::{Error, Result};

use super::components::ChordComponents;
use super::interval::Interval;

pub struct Chord {
    pub name: String,
    pub notes: Vec<usize>,
    pub score: usize,
}

impl Chord {
    pub fn new(notes: Vec<usize>, intervals: Vec<Interval>) -> Result<Chord> {
        let parts = ChordComponents::build(intervals).unwrap();
    
        let (name, score)  = parts.construct_name(notes[0]).unwrap();
    
        let chord = Chord {
            name,
            notes,
            score,
        };
    
        Ok(chord)
    }
}