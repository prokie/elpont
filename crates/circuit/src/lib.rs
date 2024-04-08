use parser::{elements::Element, netlist::Netlist};

pub struct Circuit<'a> {
    netlist: Netlist<'a>,
}

impl<'a> Circuit<'a> {
    pub fn new(netlist: Netlist<'a>) -> Self {
        Circuit { netlist }
    }

    /// Count the number of unique nodes in the circuit.
    pub fn count_nodes(&self) -> usize {
        let mut nodes = Vec::new();
        for component in &self.netlist.components {
            match component {
                Element::Resistor(resistor) => {
                    nodes.push(resistor.node1);
                    nodes.push(resistor.node2);
                }
                Element::Capacitor(capacitor) => {
                    nodes.push(capacitor.node1);
                    nodes.push(capacitor.node2);
                }
                Element::VoltageSource(voltage_source) => {
                    nodes.push(voltage_source.node1);
                    nodes.push(voltage_source.node2);
                }
                Element::CurrentSource(current_source) => {
                    nodes.push(current_source.node1);
                    nodes.push(current_source.node2);
                }
            }
        }
        nodes.sort();
        nodes.dedup();
        nodes.len()
    }

    /// Get the nodes in the circuit.
    pub fn get_nodes(&self) -> Vec<&str> {
        let mut nodes = Vec::new();
        for component in &self.netlist.components {
            match component {
                Element::Resistor(resistor) => {
                    nodes.push(resistor.node1);
                    nodes.push(resistor.node2);
                }
                Element::Capacitor(capacitor) => {
                    nodes.push(capacitor.node1);
                    nodes.push(capacitor.node2);
                }
                Element::VoltageSource(voltage_source) => {
                    nodes.push(voltage_source.node1);
                    nodes.push(voltage_source.node2);
                }
                Element::CurrentSource(current_source) => {
                    nodes.push(current_source.node1);
                    nodes.push(current_source.node2);
                }
            }
        }
        nodes.sort();
        nodes.dedup();
        nodes
    }

    /// Count the number of current sources in the circuit.
    pub fn count_current_sources(&self) -> usize {
        let mut current_sources = 0;
        for component in &self.netlist.components {
            if let Element::CurrentSource(_) = component {
                current_sources += 1
            }
        }
        current_sources
    }

    /// Count the number of voltage sources in the circuit.
    pub fn count_voltage_sources(&self) -> usize {
        let mut voltage_sources = 0;
        for component in &self.netlist.components {
            if let Element::VoltageSource(_) = component {
                voltage_sources += 1
            }
        }
        voltage_sources
    }

    /// Count the number of resistors in the circuit.
    pub fn count_resistors(&self) -> usize {
        let mut resistors = 0;
        for component in &self.netlist.components {
            if let Element::Resistor(_) = component {
                resistors += 1
            }
        }
        resistors
    }

    /// Count the number of capacitors in the circuit.
    pub fn count_capacitors(&self) -> usize {
        let mut capacitors = 0;
        for component in &self.netlist.components {
            if let Element::Capacitor(_) = component {
                capacitors += 1
            }
        }
        capacitors
    }
}
