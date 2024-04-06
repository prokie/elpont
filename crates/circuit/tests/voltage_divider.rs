use parser::netlist::parse_netlist;
use simulator::Circuit;
#[test]
fn test_parse_netlist() {
    let netlist = "
    voltage divider netlist
    V1 in 0 1
    R1 in out 1k
    R2 out 0 1k
    .end
";
    let parsed_netlist = parse_netlist(netlist).unwrap();

    let circuit = Circuit::new(parsed_netlist);

    assert_eq!(circuit.count_nodes(), 3);
    assert_eq!(circuit.get_nodes(), vec!["0", "in", "out"]);
}
