pub mod capacitor;
pub mod current_source;
pub mod inductor;
pub mod resistor;
pub mod voltage_source;

pub enum Element<'a> {
    Resistor(resistor::Resistor<'a>),
    Capacitor(capacitor::Capacitor<'a>),
    VoltageSource(voltage_source::VoltageSource<'a>),
    CurrentSource(current_source::CurrentSource<'a>),
    Inductor(inductor::Inductor<'a>),
}
