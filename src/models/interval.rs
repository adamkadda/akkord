use crate::error::{Error, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
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
    Aug9, // 15

    Dim11, // 16
    Perf11, // 17
    Aug11, // 18

    // Perf12 : 19

    Min13, // 20
    Maj13, // 21
}

impl Interval {
    pub fn to_string(&self) -> String {
        let interval_str = match self {
            Interval::Unison => "P1",
            Interval::Min2 => "b2",
            Interval::Maj2 => "2",
            Interval::Min3 => "b3",
            Interval::Maj3 => "3",
            Interval::Perf4 => "4",

            Interval::Aug4 => "#4",
            Interval::Dim5 => "b5",
            
            Interval::Perf5 => "5",
            Interval::Min6 => "b6",
            Interval::Aug5 => "#5",
            Interval::Maj6 => "6",
            Interval::Dim7 => "bb7",
            Interval::Min7 => "b7",
            Interval::Maj7 => "7",
            Interval::Min9 => "b9",
            Interval::Maj9 => "9",
            Interval::Aug9 => "#9",
            Interval::Dim11 => "b11",
            Interval::Perf11 => "11",
            Interval::Aug11 => "#11",
            Interval::Min13 => "b13",
            Interval::Maj13 => "13",
        };

        interval_str.to_string()
    }
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
        15 => Interval::Aug9,

        16 => Interval::Dim11,
        17 => Interval::Perf11,
        18 => Interval::Aug11, // or Dim12
        // 19
        20 => Interval::Min13,
        21 => Interval::Maj13,
        _ => return Err(Error::InvalidInterval),
    };

    Ok(interval)
}

pub fn get_intervals(inversion: Vec<usize>) -> Result<Vec<Interval>> {
    let mut int_intervals: Vec<usize> = Vec::new();

    for (i, cur) in inversion.iter().enumerate() {
        if i == 0 {
            int_intervals.push(0); // technically ignored value
        } else {
            let prev = &inversion[i - 1];
        
            // if the current note is 'higher' than the previous . . .
            let value: usize = match cur > prev {
                true =>  int_intervals[i - 1] + (cur - prev),
                false => int_intervals[i - 1] + (12 - (prev - cur)),
            };
            int_intervals.push(value);
        }
    }

    let mut intervals: Vec<Interval> = Vec::new();

    for i in int_intervals {
        match usize_to_interval(i) {
            Ok(interval) => intervals.push(interval),
            Err(e) => return Err(e),
        }
    }

    Ok(intervals)

}

#[cfg(test)]
mod get_intervals_tests {
    use super::*;

    #[test]
    fn test_get_intervals_4_7_11_0() {
        let inversion: Vec<usize> = Vec::from([4, 7, 11, 0]);
        let correct: Vec<Interval> = Vec::from([
            Interval::Unison,
            Interval::Min3,
            Interval::Perf5,
            Interval::Min6,
        ]);
        assert_eq!(get_intervals(inversion), Ok(correct));
    }

    #[test]
    fn test_get_intervals_0_4_8_7() {
        let inversion: Vec<usize> = Vec::from([2, 6, 9, 1]);
        get_intervals(inversion);
    }
}

pub fn is_valid_inversion(inversion: &Vec<Interval>) -> bool {
    inversion.iter().any(|interval| matches!(
        interval, 
        Interval::Min2 | Interval::Maj2 | Interval::Min3 | Interval::Maj3 | Interval::Perf4
    ))
}