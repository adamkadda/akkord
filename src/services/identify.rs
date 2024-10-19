use itertools::Itertools;

use crate::{error::{Error, Result}, models::{chord::Chord, interval::{get_intervals, is_valid_inversion}}};
use super::{naming::name, processing::*};

pub fn identify(notes: Vec<i8>) -> Result<Vec<Chord>> {
    if !check(&notes) { return Err(Error::InvalidNotes) }
    
    let cleaned = clean(notes);

    let norm = normalize(cleaned);
    let k = norm.len();

    let inversions = norm.into_iter().permutations(k);

    let mut chords: Vec<Chord> = Vec::new();

    for inversion in inversions {
        let root = inversion[0];
        let intervals = get_intervals(inversion);

        if is_valid_inversion(&intervals) {
            if let Ok(c) = name(root, intervals) { chords.push(c) }
        }
    }
    
    chords.sort_by(|a, b| b.conciseness.cmp(&a.conciseness));

    Ok(chords)
}