pub(super) fn check(notes: &Vec<i8>) -> bool {
    for note in notes {
        if *note < 0 || *note > 23 { return false }
    }
    
    true
}

pub(super) fn clean(notes: Vec<i8>) -> Vec<i8> {
    let mut tally = std::collections::HashSet::new();
    
    notes.into_iter()
        .filter(|note| tally.insert(*note))
        .collect()
}

pub(super) fn normalize(notes: Vec<i8>) -> Vec<usize> {
    // standardize notes from different octaves into a single octave
    let mut norm: Vec<usize> = notes.into_iter()
        .map(|note|{
            if note > 11 {
                (note - 12) as usize
            } else {
                note as usize
            }
        })
        .collect();

    norm.sort();

    norm
}