pub fn read_or_die(fname: &str) -> String { std::fs::read_to_string(fname).unwrap() }
