pub struct Resistor {
    pub name: String,
    pub node1: u32,
    pub node2: u32,
    pub value: String,
}

pub fn parse_resistor(line: &str) -> Option<Resistor> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 4 && parts[0].starts_with('R') {
        Some(Resistor {
            name: parts[0].to_string(),
            node1: parts[1].parse().unwrap(),
            node2: parts[2].parse().unwrap(),
            value: parts[3].to_string(),
        })
    } else {
        None
    }
}
