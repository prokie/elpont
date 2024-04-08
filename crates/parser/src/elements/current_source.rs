pub struct CurrentSource<'a> {
    pub name: &'a str,
    pub node1: &'a str,
    pub node2: &'a str,
    pub value: &'a str,
}

pub fn parse_current_source(line: &str) -> Result<CurrentSource<'_>, &'static str> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 4 || !parts[0].starts_with('I') {
        return Err("Line does not represent a current source");
    }
    Ok(CurrentSource {
        name: parts[0],
        node1: parts[1],
        node2: parts[2],
        value: parts[3],
    })
}
