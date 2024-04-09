use crate::elements::{
    capacitor::parse_capacitor, current_source::parse_current_source, inductor::parse_inductor,
    resistor::parse_resistor, voltage_source::parse_voltage_source, Element,
};

pub struct Netlist<'a> {
    pub title: &'a str,
    pub components: Vec<Element<'a>>,
}

pub fn parse_netlist(netlist: &str) -> Result<Netlist, &'static str> {
    let mut lines: Vec<&str> = netlist.trim().lines().map(str::trim).collect();

    if lines.is_empty() {
        return Err("Netlist is empty");
    }

    if lines.pop().map(str::to_uppercase) != Some(".END".to_string()) {
        return Err("Netlist does not end with .END");
    }

    let title = lines.remove(0);

    let mut components = Vec::new();

    for line in lines {
        if let Ok(resistor) = parse_resistor(line) {
            components.push(Element::Resistor(resistor));
        } else if let Ok(capacitor) = parse_capacitor(line) {
            components.push(Element::Capacitor(capacitor));
        } else if let Ok(inductor) = parse_inductor(line) {
            components.push(Element::Inductor(inductor));
        } else if let Ok(voltage_source) = parse_voltage_source(line) {
            components.push(Element::VoltageSource(voltage_source));
        } else if let Ok(current_source) = parse_current_source(line) {
            components.push(Element::CurrentSource(current_source));
        } else {
            return Err("Line does not represent a valid component");
        }
    }
    Ok(Netlist { title, components })
}
