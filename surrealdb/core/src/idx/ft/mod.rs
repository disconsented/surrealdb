pub mod analyzer;
pub mod fulltext;
pub mod highlighter;
pub mod offset;

pub(super) type Position = u32;
pub type DocLength = u64;
pub type TermFrequency = u64;
pub(super) type Score = f32;

pub type MatchRef = u8;
