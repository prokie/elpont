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

pub fn translate_value(value: &str) -> Result<f64, std::num::ParseFloatError> {
    let factor = match value.chars().last() {
        Some('T') => 1E12,
        Some('G') => 1E9,
        Some('k') => 1E3,
        Some('m') => 1E-3,
        Some('u') => 1e-6,
        Some('n') => 1e-9,
        Some('p') => 1e-12,
        Some('f') => 1e-15,
        _ => return value.parse::<f64>(),
    };

    let numeric_part: &str = &value[..value.len() - 1];

    numeric_part.parse::<f64>().map(|v| v * factor)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_translate_value() {
        assert_eq!(translate_value("1T").unwrap(), 1E12);
        assert_eq!(translate_value("1G").unwrap(), 1E9);
        assert_eq!(translate_value("1k").unwrap(), 1E3);
        assert_eq!(translate_value("1m").unwrap(), 1E-3);
        assert_eq!(translate_value("1u").unwrap(), 1E-6);
        assert_eq!(translate_value("1n").unwrap(), 1E-9);
        assert_eq!(translate_value("1p").unwrap(), 1E-12);
        assert_eq!(translate_value("1f").unwrap(), 1E-15);
        assert_eq!(translate_value("1").unwrap(), 1.0);
        assert_eq!(translate_value("100").unwrap(), 100.0);
    }

    #[test]
    fn test_translate_value_invalid() {
        assert!(translate_value("1z").is_err());
        assert!(translate_value("abc").is_err());
    }
}
