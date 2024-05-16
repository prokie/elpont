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
                Element::Inductor(inductor) => {
                    nodes.push(inductor.node1);
                    nodes.push(inductor.node2);
                }
            }
        }
        nodes.sort_unstable();
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
                Element::Inductor(inductor) => {
                    nodes.push(inductor.node1);
                    nodes.push(inductor.node2);
                }
            }
        }
        nodes.sort_unstable();
        nodes.dedup();
        nodes
    }

    /// Count the number of current sources in the circuit.
    pub fn count_current_sources(&self) -> usize {
        let mut current_sources = 0;
        for component in &self.netlist.components {
            if let Element::CurrentSource(_) = component {
                current_sources += 1;
            }
        }
        current_sources
    }

    /// Count the number of voltage sources in the circuit.
    pub fn count_voltage_sources(&self) -> usize {
        let mut voltage_sources = 0;
        for component in &self.netlist.components {
            if let Element::VoltageSource(_) = component {
                voltage_sources += 1;
            }
        }
        voltage_sources
    }

    /// Count the number of resistors in the circuit.
    pub fn count_resistors(&self) -> usize {
        let mut resistors = 0;
        for component in &self.netlist.components {
            if let Element::Resistor(_) = component {
                resistors += 1;
            }
        }
        resistors
    }

    /// Count the number of capacitors in the circuit.
    pub fn count_capacitors(&self) -> usize {
        let mut capacitors = 0;
        for component in &self.netlist.components {
            if let Element::Capacitor(_) = component {
                capacitors += 1;
            }
        }
        capacitors
    }

    /// Count the number of inductors in the circuit.
    pub fn count_inductors(&self) -> usize {
        let mut inductors = 0;
        for component in &self.netlist.components {
            if let Element::Inductor(_) = component {
                inductors += 1;
            }
        }
        inductors
    }

    /// Get the title of the circuit.
    pub fn get_title(&self) -> &str {
        self.netlist.title
    }

    /// Get the components in the circuit.
    pub fn get_components(&self) -> &Vec<Element<'a>> {
        &self.netlist.components
    }

    /// The G matrix is a nxn matrix of conductances between nodes in the circuit.
    pub fn generate_g_matrx(&self) -> Vec<Vec<f64>> {
        let mut g_matrix = vec![];
        for node in self.get_nodes() {
            let mut row = Vec::new();
            for component in self.get_components() {
                if let Element::Resistor(resistor) = component {
                    if resistor.node1 == node || resistor.node2 == node {
                        row.push(resistor);
                    }
                }
            }
            println!("Node: {}", node);
            for resistor in row {
                println!("Resistor: {}", resistor.name);
            }
        }

        g_matrix
    }

    pub fn generate_mna_matrices(&self) {
        todo!();
    }
}

// test the generate_mna_matrices method

#[cfg(test)]

mod tests {
    use super::*;
    use parser::netlist::Netlist;

    #[test]
    fn test_generate_mna_matrices() {
        let netlist = Netlist {
            title: "Test Circuit",
            components: vec![
                Element::Resistor(parser::elements::resistor::Resistor {
                    name: "R1",
                    node1: "n1",
                    node2: "n2",
                    value: "1.0",
                }),
                Element::Resistor(parser::elements::resistor::Resistor {
                    name: "R2",
                    node1: "n2",
                    node2: "n3",
                    value: "2.0",
                }),
                Element::Resistor(parser::elements::resistor::Resistor {
                    name: "R3",
                    node1: "n3",
                    node2: "n1",
                    value: "3.0",
                }),
            ],
        };
        let circuit = Circuit::new(netlist);
        circuit.generate_mna_matrices();
    }
}
