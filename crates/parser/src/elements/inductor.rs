pub struct Inductor<'a> {
    pub name: &'a str,
    pub node1: &'a str,
    pub node2: &'a str,
    pub value: &'a str,
}

pub fn parse_inductor(line: &str) -> Result<Inductor<'_>, &'static str> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 4 || !parts[0].starts_with('L') {
        return Err("Line does not represent an inductor");
    }
    Ok(Inductor {
        name: parts[0],
        node1: parts[1],
        node2: parts[2],
        value: parts[3],
    })
}
