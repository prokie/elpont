use parser::elements::Element;
use parser::netlist::parse_netlist;
#[test]
fn test_parse_netlist() {
    let netlist = "
        My Circuit
        R1 1 0 10k
        R2 2 0 10k
        R3 3 0 10k
        C1 1 0 1u
        .END
    ";
    let parsed_netlist = parse_netlist(netlist).unwrap();
    assert_eq!(parsed_netlist.title, "My Circuit");
    assert_eq!(parsed_netlist.components.len(), 4);
    let mut resistor_count = 0;
    if let Element::Resistor(resistor) = &parsed_netlist.components[0] {
        resistor_count += 1;
        assert_eq!(resistor.name, "R1");
        assert_eq!(resistor.node1, "1");
        assert_eq!(resistor.node2, "0");
        assert_eq!(resistor.value, "10k");
    }
    assert_eq!(resistor_count, 1);
}
