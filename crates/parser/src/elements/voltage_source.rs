pub struct VoltageSource<'a> {
    pub name: &'a str,
    pub node1: &'a str,
    pub node2: &'a str,
    pub value: &'a str,
}

pub fn parse_voltage_source(line: &str) -> Result<VoltageSource<'_>, &'static str> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 4 || !parts[0].starts_with('V') {
        return Err("Line does not represent a voltage source");
    }
    Ok(VoltageSource {
        name: parts[0],
        node1: parts[1],
        node2: parts[2],
        value: parts[3],
    })
}
