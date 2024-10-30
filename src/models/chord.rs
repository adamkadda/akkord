use crate::error::{Error, Result};

use super::components::ChordComponents;
use super::interval::Interval;

#[derive(Debug)]
pub struct Chord {
    pub name: String,
    pub notes: Vec<usize>,
    pub score: usize,
}

impl Chord {
    pub fn new(notes: Vec<usize>, intervals: Vec<Interval>) -> Result<Chord> {
        let parts = match ChordComponents::build(intervals) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
    
        let (name, score) = match parts.construct_name(notes[0]) {
            Ok((n, s)) => (n, s),
            Err(e) => return Err(e),
        };

        let chord = Chord {
            name,
            notes,
            score,
        };
    
        Ok(chord)
    }

    pub fn color(&self) -> String {
        if self.score >= 96 {
            return "green".to_string();
        } else if self.score > 88 {
            return "yellow".to_string();
        } else {
            return "red".to_string();
        };
    }

    pub fn data(&self) -> String {

        /*
        [5, 9, 0, 4] -> ["-7","-3","0","4"]
        5 > -1 -> 5 - 12 = -7
        9 > 5  -> 9 - 12 = -3
        0 < 9  -> 0
        4 > 0  -> 4 - 12 = -8
        */

        let mut prev: i8 = -13;
        let mut higher: bool = false;
        self.notes
            .iter()
            .map(|&note| {
                let cur = note as i8;
                let normd = match ((cur > prev), higher) {
                    (true, false) => cur - 12,
                    (false, false) => {
                        higher = true;
                        cur
                    }
                    (_, true) => cur,
                };
                prev = cur;
                format!("{}", normd)
            })
            .collect::<Vec<String>>()
            .join(",")
    }
}