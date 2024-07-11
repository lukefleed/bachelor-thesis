use std::fmt::Display;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum Orientation {
    Forward,
    Reverse,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::Forward => write!(f, "+"),
            Orientation::Reverse => write!(f, "-"),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Entry {
    Header {
        version: String,
    },
    Segment {
        id: usize,
        sequence: String,
    },
    Link {
        from: usize,
        from_orient: Orientation,
        to: usize,
        to_orient: Orientation,
    },
    Path {
        name: String,
        segments: Vec<(String, Orientation)>,
    },
    Walk {
        sample: String,
        haplotype_index: usize,
        seq_id: String,
        seq_start: usize,
        seq_end: usize,
        segments: Vec<(String, Orientation)>,
    },
}
