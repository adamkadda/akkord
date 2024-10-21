use itertools::Itertools;

use crate::error::{Error, Result};
use crate::models::chord::Chord;
use crate::models::interval::{get_intervals, is_valid_inversion};

use super::processing::{check, clean, normalize};

pub fn identify(notes: Vec<i8>) -> Result<Vec<Chord>> {
    if !check(&notes) { return Err(Error::InvalidNotes) }
    
    let cleaned = clean(notes);

    let norm = normalize(cleaned);
    let k = norm.len();

    let inversions = norm.into_iter().permutations(k);

    let mut chords: Vec<Chord> = Vec::new();

    for inversion in inversions {
        let notes = inversion.clone();
        let intervals = get_intervals(inversion);

        if is_valid_inversion(&intervals) {
            if let Ok(c) = Chord::new(notes, intervals) { chords.push(c) }
        }
    }
    
    chords.sort_by(|a, b| b.score.cmp(&a.score));

    Ok(chords)
}