pub enum Analyses {
    AC,
    DCSweep,
    Distortion,
    Noise,
    OperatingPoint,
    PoleZero,
    Sensitivity,
    Transient,
}

pub fn parse_analysis(line: &str) -> Result<Analyses, &'static str> {
    match line {
        ".ac" => Ok(Analyses::AC),
        ".dc" => Ok(Analyses::DCSweep),
        ".disto" => Ok(Analyses::Distortion),
        ".noise" => Ok(Analyses::Noise),
        ".pz" => Ok(Analyses::PoleZero),
        ".sens" => Ok(Analyses::Sensitivity),
        ".op" => Ok(Analyses::OperatingPoint),
        ".tran" => Ok(Analyses::Transient),
        _ => Err("Unknown analysis"),
    }
}
