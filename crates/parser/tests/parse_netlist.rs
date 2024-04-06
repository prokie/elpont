use parser::netlist::parse_netlist;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::{tempdir, TempDir};

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
    assert_eq!(parsed_netlist.resistors.len(), 3);
    assert_eq!(parsed_netlist.resistors[0].name, "R1");
    assert_eq!(parsed_netlist.resistors[0].node1, "1");
    assert_eq!(parsed_netlist.resistors[0].node2, "0");
    assert_eq!(parsed_netlist.resistors[0].value, "10k");
}

// #[test]
// fn test_ngspice_netlist() {
//     let netlist = "
//     My Circuit
//     R1 1 0 100
//     R2 2 0 200
//     .op
//     .END
//     ";
//     let trimmed_netlist = trim_lines(netlist);

//     let dir = tempdir().expect("Failed to create temporary directory");
//     let file_path = write_to_temp_file(&trimmed_netlist, &dir);
//     run_ngspice(&file_path);
// }

fn trim_lines(netlist: &str) -> String {
    netlist
        .trim()
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n")
}

fn write_to_temp_file(netlist: &str, dir: &TempDir) -> PathBuf {
    let file_path = dir.path().join("netlist.cir");
    let mut file = File::create(&file_path).expect("Failed to create file");
    write!(file, "{}", netlist).expect("Failed to write to file");
    file_path
}
fn run_ngspice(file_path: &Path) {
    let output = Command::new("ngspice")
        .arg("-b")
        .arg(file_path)
        .output()
        .expect("Failed to execute ngspice");

    assert!(output.status.success(), "ngspice returned an error");
}
