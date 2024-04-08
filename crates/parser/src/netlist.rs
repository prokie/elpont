use crate::elements::capacitor::parse_capacitor;
use crate::elements::resistor::parse_resistor;
use crate::elements::Element;

pub struct Netlist<'a> {
    pub title: &'a str,
    pub components: Vec<Element<'a>>,
}

pub fn parse_netlist(netlist: &str) -> Result<Netlist, &'static str> {
    let mut lines: Vec<&str> = netlist.trim().lines().map(|line| line.trim()).collect();

    if lines.is_empty() {
        return Err("Netlist is empty");
    }

    if lines.pop().map(|s| s.to_uppercase()) != Some(".END".to_string()) {
        return Err("Netlist does not end with .END");
    }

    let title = lines.remove(0);

    let mut components = Vec::new();

    for line in lines {
        if let Ok(resistor) = parse_resistor(line) {
            components.push(Element::Resistor(resistor));
        } else if let Ok(capacitor) = parse_capacitor(line) {
            components.push(Element::Capacitor(capacitor));
        }
    }
    Ok(Netlist { title, components })
}
