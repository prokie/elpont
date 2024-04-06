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
            }
        }
        nodes.sort();
        nodes.dedup();
        nodes.len()
    }

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
            }
        }
        nodes.sort();
        nodes.dedup();
        nodes
    }
}
