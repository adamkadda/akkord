pub mod chord;
pub mod interval;
pub mod components;

pub mod third {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Third {
        Minor,
        Major,
    }
}

pub mod triad {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Triad {
        Minor,
        Major,
        Diminished,
        Augmented,
    }
}

pub mod fifth {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Fifth {
        Diminished,
        Perfect,
        Augmented,
    }
}

pub mod seventh {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Seventh {
        Diminished,
        Minor,
        Major,
    }
}