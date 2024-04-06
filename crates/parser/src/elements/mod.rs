use self::{capacitor::Capacitor, resistor::Resistor};

pub mod capacitor;
pub mod resistor;

pub enum Element<'a> {
    Resistor(Resistor<'a>),
    Capacitor(Capacitor<'a>),
}
