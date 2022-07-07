// ---------------------------------------
// IOracle periferals
// ---------------------------------------
pub fn rest() {
    println!("resting");
}

pub fn read() -> (String, String) {
    println!("reading");

    ("000000".to_string(), "111000".to_string())
}

pub fn display(_hexagram: String) {
    println!("displaing");
}
