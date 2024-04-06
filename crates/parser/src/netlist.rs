use crate::{parse_resistor, Resistor};

pub struct Netlist<'a> {
    pub title: &'a str,
    pub resistors: Vec<Resistor<'a>>,
}

pub fn parse_netlist(netlist: &str) -> Result<Netlist, &'static str> {
    let mut lines: Vec<&str> = netlist.trim().lines().map(|line| line.trim()).collect();

    if lines.is_empty() {
        return Err("Netlist is empty");
    }

    if lines.pop() != Some(".END") {
        return Err("Netlist does not end with .END");
    }

    let title = lines.remove(0);

    let mut resistors = Vec::new();
    for line in lines {
        let resistor = parse_resistor(line)?;
        resistors.push(resistor);
    }
    Ok(Netlist { title, resistors })
}
