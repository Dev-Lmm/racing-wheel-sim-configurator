use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::io::ErrorKind;
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
        let val_str = degrees.to_string();

        match fs::write(&path, &val_str) {
            Ok(_) => Ok(()), // ¡Éxito a la primera!
            Err(e) if e.kind() == ErrorKind::PermissionDenied => {
                println!("Faltan permisos. Solicitando acceso de administrador por GUI...");

                // 2. Si falla por permisos, invocamos la ventana gráfica de Linux (pkexec)
                // Le pedimos que ejecute 'chmod a+w' específicamente en este archivo
                let status = Command::new("pkexec")
                    .arg("chmod")
                    .arg("a+w")
                    .arg(&path)
                    .status()?;

                // 3. Verificamos si el usuario puso la contraseña correcta o si le dio a "Cancelar"
                if status.success() {
                    println!("Permisos concedidos por el usuario. Aplicando cambios...");
                    // Reintentamos la escritura ahora que el archivo está desbloqueado
                    fs::write(&path, &val_str)
                } else {
                    // El usuario canceló la ventana o puso mal la contraseña
                    Err(std::io::Error::new(
                        ErrorKind::PermissionDenied,
                        "Operación cancelada en la ventana de autenticación."
                    ))
                }
            }
            Err(e) => Err(e), // Si es un error distinto a permisos, lo devolvemos tal cual
        }
    }
}
pub fn detect_g923() -> Option<LogitechWheel> {
    // Apuntamos directamente a los cerebros HID en lugar de las entradas genéricas
    let hid_path = "/sys/bus/hid/devices";

    if let Ok(entries) = fs::read_dir(hid_path) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Obtenemos el nombre de la carpeta (ej. "0003:046D:C26E.000B")
            let dir_name = path.file_name().unwrap_or_default().to_string_lossy().to_uppercase();

            // Buscamos específicamente el Vendor 046D (Logitech) y Product C26E (G923 Modo PC)
            if dir_name.contains("046D:C26E") {
                let range_path = path.join("range");

                // Verificamos de forma estricta que el archivo de rotación exista
                if range_path.exists() {
                    return Some(LogitechWheel {
                        name: "Logitech G923".to_string(),
                        sysfs_path: path, // Guardamos la ruta base exacta
                    });
                }
            }
        }
    }
    None
}