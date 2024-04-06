use crate::capacitor::{parse_capacitor, Capacitor};
use crate::resistor::{parse_resistor, Resistor};
pub struct Netlist<'a> {
    pub title: &'a str,
    pub resistors: Vec<Resistor<'a>>,
    pub capacitors: Vec<Capacitor<'a>>,
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
    let mut capacitors = Vec::new();

    for line in lines {
        if let Ok(resistor) = parse_resistor(line) {
            resistors.push(resistor);
        } else if let Ok(capacitor) = parse_capacitor(line) {
            capacitors.push(capacitor);
        }
    }
    Ok(Netlist {
        title,
        resistors,
        capacitors,
    })
}
