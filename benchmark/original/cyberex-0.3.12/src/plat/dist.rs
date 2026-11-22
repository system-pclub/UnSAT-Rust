use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub enum PlatDist {
    Debian,
    Rh,
    Other,
}
fn get_plat_id_like() -> String {
    let mut id_like = "".to_string();
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("ID_LIKE=") && line.contains("debian") {
                    id_like = "debian".to_string();
                    break;
                }
            } else {
                continue;
            }
        }
    }
    id_like
}

pub fn plat_dist() -> PlatDist {
    match get_plat_id_like().as_str() {
        "debian" => PlatDist::Debian,
        "fedora" => PlatDist::Rh,
        _ => PlatDist::Other,
    }
}
