use itertools::Itertools;

use crate::error::{Error, Result};
use crate::models::chord::Chord;
use crate::models::interval::{get_intervals, is_valid_inversion};

use super::processing::{check, clean, normalize};

pub fn identify(notes: Vec<i8>) -> Result<Vec<Chord>> {
    println!("->> {:<12} - identify", "MIDDLEWARE");

    if !check(&notes) { return Err(Error::InvalidNotes) }
    
    let cleaned = clean(notes);

    let norm = normalize(cleaned);    
    let k = norm.len();

    let inversions = norm.into_iter().permutations(k);

    let mut chords: Vec<Chord> = Vec::new();

    for inversion in inversions {       
        dbg!(format!("{:?}", inversion));

        let notes = inversion.clone();
        let intervals = match get_intervals(inversion) {
            Ok(vec) => vec,
            _ => continue,
        };

        // dbg!(format!("{:?}", &intervals));

        if is_valid_inversion(&intervals) {
            if let Ok(c) = Chord::new(notes, intervals) {
                // dbg!(format!("{:?}", &c));
                chords.push(c);
            }
        }
    }
    
    chords.sort_by(|a, b| b.score.cmp(&a.score));

    Ok(chords)
}

#[cfg(test)]
mod identify_tests {
    use super::*;

    #[test]
    fn test_permutations() {
        let norm: Vec<usize> = Vec::from([4, 8, 11]);
        dbg!(&norm);
        println!();
        let k = norm.len();
        let inversions = norm.into_iter().permutations(k);
        for inversion in inversions { dbg!(format!("{:?}", inversion)); }
    }
}