pub struct CurrentSource<'a> {
    pub name: &'a str,
    pub node1: &'a str,
    pub node2: &'a str,
    pub value: f64,
}

pub fn parse_current_source(line: &str) -> Result<CurrentSource<'_>, &'static str> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 4 || !parts[0].starts_with('I') {
        return Err("Line does not represent a current source");
    }

    let value = super::translate_value(parts[3]).map_err(|_| "Invalid value for voltage source")?;

    Ok(CurrentSource {
        name: parts[0],
        node1: parts[1],
        node2: parts[2],
        value,
    })
}
