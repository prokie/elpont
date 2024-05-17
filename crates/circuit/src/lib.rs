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
        self.get_nodes().len()
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
        nodes.retain(|&node| node != "0");
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
    /// Where n is the number of nodes in the circuit.
    pub fn generate_g_matrix(&self) -> Vec<Vec<f64>> {
        let number_of_nodes = self.count_nodes();
        dbg!(self.get_nodes());
        let mut g_matrix: Vec<Vec<f64>> = Vec::with_capacity(number_of_nodes);

        for row in 0..number_of_nodes {
            g_matrix.push(Vec::with_capacity(number_of_nodes));
            for _ in 0..number_of_nodes {
                g_matrix[row].push(0.0);
            }
        }

        for resistor in self.netlist.components.iter().filter_map(|component| {
            if let Element::Resistor(resistor) = component {
                Some(resistor)
            } else {
                None
            }
        }) {
            let node1 = self
                .get_nodes()
                .iter()
                .position(|&x| x == resistor.node1)
                .unwrap();
            let node2 = self
                .get_nodes()
                .iter()
                .position(|&x| x == resistor.node2)
                .unwrap();
            g_matrix[node1][node1] += 1.0 / resistor.value;
            g_matrix[node2][node2] += 1.0 / resistor.value;
            g_matrix[node1][node2] -= 1.0 / resistor.value;
            g_matrix[node2][node1] -= 1.0 / resistor.value;
        }

        dbg!(&g_matrix);

        g_matrix
    }

    pub fn generate_mna_matrices(&self) {
        dbg!(self.generate_g_matrix());
    }
}

// test the generate_mna_matrices method

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_generate_mna_matrices() {
        let netlist = "
        voltage divider netlist
        V1 in 0 1
        R1 in out 1000
        R2 out 0 1000
        .end
        ";
        let parsed_netlist = parser::netlist::parse_netlist(netlist).unwrap();
        let circuit = Circuit::new(parsed_netlist);
        circuit.generate_mna_matrices();
    }
}
