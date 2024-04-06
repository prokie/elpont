// src/parser/lib.rs

mod resistor;

pub use resistor::{parse_resistor, Resistor};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_resistor() {
        let line = "R1 1 0 10k";
        let resistor = parse_resistor(line).unwrap();
        assert_eq!(resistor.name, "R1");
        assert_eq!(resistor.node1, 1);
        assert_eq!(resistor.node2, 0);
        assert_eq!(resistor.value, "10k");

        let invalid_line = "Invalid line";
        assert!(parse_resistor(invalid_line).is_none());
    }
}
