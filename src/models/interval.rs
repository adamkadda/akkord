use crate::error::{Error, Result};

#[derive(Debug, PartialEq)]
pub enum Interval {
    Unison, // 0

    Min2, // 1
    Maj2, // 2
    Min3, // 3
    Maj3, // 4
    Perf4, // 5, 

    // 6 (tritone)
    Dim5,
    Aug4,

    Perf5, // 7
    
    // 8
    Min6,
    Aug5,

    // 9
    Maj6,
    Dim7,

    Min7, // 10
    Maj7, // 11

    // Octave : 12

    Min9, // 13
    Maj9, // 14
    
    // Min10 : 15

    Dim11, // 16
    Perf11, // 17
    Aug11, // 18

    // Perf12 : 19

    Min13, // 20
    Maj13, // 21
}

fn usize_to_interval(i: usize) -> Result<Interval> {
    let interval = match i {
        0 => Interval::Unison,
        1 => Interval::Min2,
        2 => Interval::Maj2,
        3 => Interval::Min3,
        4 => Interval::Maj3,
        5 => Interval::Perf4,
        6 => Interval::Aug4, // or Dim5
        7 => Interval::Perf5,
        8 => Interval::Min6, // or Aug5
        9 => Interval::Maj6,
        10 => Interval::Min7,
        11 => Interval::Maj7,
        // 12
        13 => Interval::Min9,
        14 => Interval::Maj9,
        // 15
        // 16
        17 => Interval::Perf11,
        18 => Interval::Aug11, // or Dim12
        // 19
        20 => Interval::Min13,
        21 => Interval::Maj13,
        _ => return Err(Error::InvalidInversion),
    };

    Ok(interval)
}

pub fn get_intervals(inversion: Vec<usize>) -> Vec<Interval> {
    let mut int_intervals: Vec<usize> = Vec::new();

    for (i, cur) in inversion.iter().enumerate() {
        if i == 0 {
            int_intervals.push(0); // technically ignored value
        } else {
            let prev = &inversion[i - 1];
            
            // if the current note is 'higher' than the previous . . .
            let value: usize = match cur > prev {
                true =>  int_intervals[i - 1] + (cur - prev),
                false => int_intervals[i - 1] + (cur - prev) + 12,
            };
            int_intervals.push(value);
        }
    }

    int_intervals.into_iter()
    .filter_map(|i| usize_to_interval(i).ok()) // Extracts Ok values
    .collect()
}

pub fn is_valid_inversion(inversion: &Vec<Interval>) -> bool {
    inversion.iter().any(|interval| matches!(
        interval, 
        Interval::Min2 | Interval::Maj2 | Interval::Min3 | Interval::Maj3 | Interval::Perf4
    ))
}
