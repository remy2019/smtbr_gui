use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MultipleChoice {
    pub a: String,
    pub b: String,
    pub c: String,
    pub d: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Problem {
    pub chapter: u32,
    pub prob_num_rel: u32,
    pub prob_num_ab: u32,
    pub mcq: bool,
    pub tf: bool,
    pub mc: Option<MultipleChoice>,
    pub question: String,
    pub answer: String,
}

impl MultipleChoice {
    pub fn new(a: String, b: String, c: String, d: String) -> Self {
        MultipleChoice { a, b, c, d }
    }
}

impl Problem {
    pub fn new(
        chapter: u32,
        prob_num_rel: u32,
        prob_num_ab: u32,
        mcq: bool,
        tf: bool,
        mc: Option<MultipleChoice>,
        question: String,
        answer: String,
    ) -> Self {
        Problem {
            chapter,
            prob_num_rel,
            prob_num_ab,
            mcq,
            tf,
            mc,
            question,
            answer,
        }
    }
}
