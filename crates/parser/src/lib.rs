// src/parser/lib.rs

mod netlist;
mod resistor;
pub use netlist::{parse_netlist, Netlist};
pub use resistor::{parse_resistor, Resistor};
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_resistor() {
        let line = "R1 1 0 10k";
        let resistor = parse_resistor(line).unwrap();
        assert_eq!(resistor.name, "R1");
        assert_eq!(resistor.node1, "1");
        assert_eq!(resistor.node2, "0");
        assert_eq!(resistor.value, "10k");

        let invalid_line = "Invalid line";
        assert!(parse_resistor(invalid_line).is_err());
    }

    #[test]
    fn test_parse_netlist() {
        let netlist = "
            My Circuit
            R1 1 0 10k
            R2 2 0 10k
            R3 3 0 10k
            .END
        ";
        let parsed_netlist = parse_netlist(netlist).unwrap();
        assert_eq!(parsed_netlist.title, "My Circuit");
        assert_eq!(parsed_netlist.resistors.len(), 3);
        assert_eq!(parsed_netlist.resistors[0].name, "R1");
        assert_eq!(parsed_netlist.resistors[0].node1, "1");
        assert_eq!(parsed_netlist.resistors[0].node2, "0");
        assert_eq!(parsed_netlist.resistors[0].value, "10k");
    }
}
