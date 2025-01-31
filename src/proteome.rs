#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Proteinanalyze {
    pub targetname: String,
    pub tlength: usize,
    pub queryname: String,
    pub querylength: usize,
    pub evalue: f32,
    pub score: f32,
    pub val1: usize,
    pub val2: usize,
    pub ed1: usize,
    pub ed2: usize,
    pub al1: usize,
    pub al2: usize,
    pub desc: String,
}
