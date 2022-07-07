// ---------------------------------------
// IOracle hardware controll
// ---------------------------------------
pub fn rest() {
    println!("resting");
}

pub fn read() -> (String, String) {
    println!("reading");
    ("111000".to_string(), "000111".to_string())
}

pub fn display(_hexagram: String) {
    println!("displaing");
}
