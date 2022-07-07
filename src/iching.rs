// ---------------------------------------
// IOracle iching core
// ---------------------------------------
pub struct Answer {
    pub question: String,
    pub answer: String,
    pub hexagram: String,
    pub r_hexagram: String,
}

impl Answer {
    pub fn new(question: String, hexagram: String, r_hexagram: String) -> Self {
        println!("question: {}", question);
        let answer = "42".to_string();
        Answer {
            question,
            answer,
            hexagram,
            r_hexagram,
        }
    }
    pub fn get_by_id(_id: u64) -> Self {
        Answer {
            question: "question".to_string(),
            answer: "answer".to_string(),
            hexagram: "000000".to_string(),
            r_hexagram: "000111".to_string(),
        }
    }
    pub fn save(self) -> u64 {
        42
    }
}
