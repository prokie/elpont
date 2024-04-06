use parser::netlist::Netlist;

pub struct Simulator<'a> {
    netlist: Netlist<'a>,
}

impl<'a> Simulator<'a> {
    pub fn new(netlist: Netlist<'a>) -> Self {
        Simulator { netlist }
    }
}

fn main() {}
