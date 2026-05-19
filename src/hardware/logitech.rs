use std::fs;
use std::path::PathBuf;
use super::RaceWheel;

pub struct LogitechWheel {
    pub name: String,
    pub sysfs_path: PathBuf,
}

impl RaceWheel for LogitechWheel {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_rotation(&self, degrees: u32) -> Result<(), std::io::Error> {
        let path = self.sysfs_path.join("range");
        fs::write(path, degrees.to_string())
    }
}

pub fn detect_g923() -> Option<LogitechWheel> {
    let input_path = "/sys/class/input";
    if let Ok(entries) = fs::read_dir(input_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name_path = path.join("device/name");
            if name_path.exists() {
                if let Ok(mut name) = fs::read_to_string(&name_path) {
                    name = name.trim().to_string();

                    if name.contains("G923") || name.contains("Logitech G923") {
                        return Some(LogitechWheel {
                            name,
                            sysfs_path: path.join("device"),
                        });
                    }
                }
            }
        }
    }
    None
}