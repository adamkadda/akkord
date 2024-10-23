use std::mem::Discriminant;

use crate::error::{Error, Result};

use super::{fifth::Fifth, interval::Interval, seventh::Seventh, third::Third, triad::Triad};

pub struct ChordComponents {
    third: Option<Third>,
    sus: Vec<Interval>,
    fifth: Option<Fifth>,
    triad: Option<Triad>,
    seventh: Option<Seventh>,
    lead: Option<Interval>,
    add: Vec<Interval>,
}

impl ChordComponents {
    pub fn build(intervals: Vec<Interval>) -> Result<Self> {
        let mut third: Option<Third> = None;
        let mut sus: Vec<Interval> = Vec::new();
        let mut fifth: Option<Fifth> = None;
        let mut triad: Option<Triad> = None;
        let mut seventh: Option<Seventh> = None;
        let mut lead: Option<Interval> = None;
        let mut add: Vec<Interval> = Vec::new();
        
        for interval in intervals {
            match interval {
                // ignore root
                Interval::Unison => (),
    
                // third, sus & add
                Interval::Min3 => {
                    third = Some(Third::Minor);
                    if sus.is_empty() { add.append(&mut sus); }
                }
                Interval::Maj3 => {
                    if let Some(Third::Minor) = third { add.push(Interval::Min3); }
                    third = Some(Third::Major);
                    if !sus.is_empty() { add.append(&mut sus); }
                }
                Interval::Min2 | Interval::Maj2 | Interval::Perf4 => {
                    if let None = third {
                        sus.push(interval)
                    } else {
                        add.push(interval)
                    }
                }
    
                // fifth or aug4
                Interval::Aug4 | Interval::Dim5 => {
                    match third {
                        None => sus.push(Interval::Aug4),
                        Some(thr) => {
                            fifth = Some(Fifth::Diminished);
                            match thr {
                                Third::Minor => triad = Some(Triad::Diminished),
                                Third::Major => (),
                            }
                        }
                    }
                }
                Interval::Perf5 => {
                    fifth = Some(Fifth::Perfect);
                    match third {
                        None => (),
                        Some(thr) => {
                            match thr {
                                Third::Minor => {
                                    if let Some(Triad::Diminished) = triad {
                                        add.push(Interval::Aug4);
                                    }
                                    triad = Some(Triad::Minor);
                                }
                                Third::Major => triad = Some(Triad::Major),
                            }
                        }
                    }
                }
                Interval::Aug5 | Interval::Min6 => {
                    match fifth {
                        None => {
                            if let Some(thr) = third {
                                match thr {
                                    Third::Major => triad = Some(Triad::Augmented),
                                    Third::Minor => return Err(Error::InvalidInversion),
                                }
                            }
                            fifth = Some(Fifth::Augmented);
                        }
                        Some(_) => add.push(Interval::Min6), // dim triad | min triad
                    }
                }
    
                // sixth or dim7
                Interval::Maj6 | Interval::Dim7 => {
                    if let Some(Triad::Diminished) = triad {
                        seventh = Some(Seventh::Diminished);
                        lead = Some(Interval::Dim7);
                    } else { 
                        lead = Some(Interval::Maj6);
                    }
                }
    
                // seventh
                Interval::Min7 => {
                    seventh = Some(Seventh::Minor);
                    lead = Some(Interval::Min7);
                }
                Interval::Maj7 => {
                    if let Some(Seventh::Minor) = seventh {
                        return Err(Error::InvalidInversion)
                    } else {
                        seventh = Some(Seventh::Major);
                        lead = Some(Interval::Maj7);
                    }
                }
    
                // ninth
                Interval::Min9 => add.push(interval),
                Interval::Maj9 => {
                    match (third, lead) {
                        (Some(Third::Major), Some(Interval::Min7)) => lead = Some(interval),
                        _ => add.push(interval),
                    }
                }
                Interval::Aug9 => add.push(interval),
    
                // eleventh
                Interval::Dim11 => add.push(interval),
                Interval::Perf11 => {
                    if let Some(Interval::Maj9) = lead {
                        lead = Some(interval);
                    } else {
                        add.push(interval);
                    }
                }
                Interval::Aug11 => add.push(interval),
    
                // thirteenth
                Interval::Min13 => add.push(interval),
                Interval::Maj13 => {
                    if let Some(Interval::Perf11) = lead {
                        lead = Some(interval);
                    } else {
                        add.push(interval);
                    }
                }
            }
        }

        Ok(ChordComponents {
            third,
            sus,
            fifth,
            triad,
            seventh,
            lead,
            add,
        })
    }

    pub fn construct_name(self, root: usize) -> Result<(String, usize)> {
        let mut name = String::new();
        let root = note_to_string(root).unwrap();
        name.push_str(&root);

        let mut alterations = String::new();

        let mut conciseness: usize = 100;
        
        if let Some(lead) = self.lead {
            match lead {
                Interval::Maj6 => {

                    /*
                    This logic flow allows us to more easily incorporate susx 
                    functionality later on.

                    Here, we decided to require P5 for X6 and Xm6 chords. Neither 
                    the m3 nor M3 are requirements for the X6 chord.
                    */

                    if let Some(Fifth::Perfect) = self.fifth {
                        if let Some(Third::Minor) = self.third {
                            name.push_str("m6");
                        } else {
                            name.push_str("6");
                        }
                    } else {
                        return Err(Error::InvalidInversion)
                    }
                }
                Interval::Dim7 => name.push_str("dim7"),
                Interval::Min7 => {
                    
                    /*
                    It seems that X7 is the fallback chord for a considerable 
                    fraction of all possible chords, given that X7 chords only 
                    require {P1, P5, m7}.
                    */

                    match (self.third, self.fifth) {
                        (Some(Third::Minor), Some(Fifth::Perfect)) => name.push_str("m7"),
                        (Some(Third::Minor), Some(Fifth::Diminished)) => {
                            name.push_str("m7");
                            alterations.push_str("b5");
                        }
                        (Some(Third::Major), Some(Fifth::Augmented)) => name.push_str("aug7"),
                        (_, Some(Fifth::Perfect)) => name.push_str("7"),
                        (_, _) => return Err(Error::InvalidInversion)
                    }
                }
                Interval::Maj7 => {

                    /*
                    There seem to only be three true chords that have M7 as
                    their highest interval. Namely: Xdim maj7, XmMaj7, Xmaj7.

                    Interestingly the 'maj' or 'm' in the 7th chords do not
                    reliably inform us of the quality of their thirds and
                    fifths, only their sevenths (in most cases).

                    I have a feeling there is something missing here.
                    */

                    match (self.third, self.fifth) {
                        (Some(Third::Minor), Some(Fifth::Diminished)) => name.push_str("dim maj7"),
                        (Some(Third::Minor), Some(fifth)) => {
                            name.push_str("mMaj7");
                            match fifth {
                                Fifth::Diminished => alterations.push_str("b5"),
                                Fifth::Augmented => alterations.push_str("#5"),
                                Fifth::Perfect => (),
                            }
                        }
                        (_, Some(fifth)) => {
                            name.push_str("maj7");
                            match fifth {
                                Fifth::Diminished => alterations.push_str("b5"),
                                Fifth::Augmented => alterations.push_str("#5"),
                                Fifth::Perfect => (),
                            }
                        }
                        _ => return Err(Error::InvalidInversion)
                    }
                }

                Interval::Maj9 => name.push_str("9"),
                Interval::Perf11 => name.push_str("11"),
                Interval::Maj13 => name.push_str("13"),
                _ => return Err(Error::InvalidInversion)
            }

            let mut extensions = String::new();

            // C7(b5)
            if !alterations.is_empty() {
                extensions.push_str("(");
                extensions.push_str(&alterations);
                conciseness -= 3;
            }

            // C7(sus2,4) | C7(b5 sus2,4)
            if !self.sus.is_empty() {

                // prepare extensions string(?)
                // prepare base-string
                // prepare space-separated susses
                // concatenate base-string and susses
                // push onto extensions string

                if extensions.is_empty() {
                    // init extensions string
                    extensions.push_str("(");
                } else {
                    extensions.push_str(" ");
                }

                let mut susses = String::from("sus");
                let mut nums = String::new();
                let mut first_iteration = true;

                for n in self.sus {
                    if first_iteration {
                        nums.push_str(&n.to_string());
                        first_iteration = false;
                    } else {
                        nums.push_str(",");
                        nums.push_str(&n.to_string());
                    }
                    conciseness -= 4;
                }

                susses.push_str(&nums);
                extensions.push_str(&susses);
            }

            // C7(add9,13) | C7(b5 sus2,4 add9,13)
            if !self.add.is_empty() {

                // same logic with susses

                if extensions.is_empty() {
                    // init extensions string
                    extensions.push_str("(");
                } else {
                    extensions.push_str(" ");
                }

                let mut adds = String::from("add");
                let mut nums = String::new();
                let mut first_iteration = true;

                for n in self.add {
                    if first_iteration {
                        nums.push_str(&n.to_string());
                        first_iteration = false;
                    } else {
                        nums.push_str(",");
                        nums.push_str(&n.to_string());
                    }
                    conciseness -= 4;
                }

                adds.push_str(&nums);
                extensions.push_str(&adds);
            }

            extensions.push_str(")");
            name.push_str(&extensions);
        } else {

            // triads
            match self.triad {
                Some(Triad::Major) => (),
                Some(Triad::Augmented) => name.push_str("aug"),
                Some(Triad::Minor) => name.push_str("m"),
                Some(Triad::Diminished) => name.push_str("dim"),
                _ => {

                    // sus + P5
                    match (self.sus[0], self.fifth) {
                        (any, Some(Fifth::Perfect)) => {
                            let s: String = format!("sus{}", self.sus[0].to_string());
                            name.push_str(&s);
                        }
                        (_, _) => return Err(Error::InvalidInversion)
                    }
                }
            }
        }

        Ok((name, conciseness))
    }
}

pub fn note_to_string(num: usize) -> Result<String> {
    let str_note: String = match num {
        0 => "C".to_string(),
        1 => "C#".to_string(),
        2 => "D".to_string(),
        3 => "Db".to_string(),
        4 => "E".to_string(),
        5 => "F".to_string(),
        6 => "F#".to_string(),
        7 => "G".to_string(),
        8 => "Ab".to_string(),
        9 => "A".to_string(),
        10 => "Bb".to_string(),
        11 => "B".to_string(),
        _ => return Err(Error::InvalidRoot)
    };

    Ok(str_note)
}