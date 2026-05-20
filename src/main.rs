mod hardware;

use hardware::RaceWheel;
use std::sync::Arc;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // Intentamos detectar el volante al arrancar
    let volante = hardware::logitech::detect_g923();

    let volante_compartido: Option<Arc<dyn RaceWheel>> = match volante {
        Some(w) => {
            // CORRECCIÓN: Cambiar 'set_status-text' por 'set_status_text'
            ui.set_status_text(format!("Conectado: {}", w.name).into());
            Some(Arc::new(w))
        }
        None => {
            // CORRECCIÓN: Cambiar 'set_status-text' por 'set_status_text'
            ui.set_status_text("No se detectó ningún volante compatible por USB.".into());
            None
        }
    };

    // Manejamos el evento cuando el usuario da clic en "Aplicar Cambios"
    let volante_para_ui = volante_compartido.clone();
    ui.on_rotation_changed(move |angle| {
        if let Some(ref wheel) = volante_para_ui {
            match wheel.set_rotation(angle as u32) {
                Ok(_) => println!("¡Ángulo de {}° aplicado con éxito!", angle),
                Err(e) => eprintln!("Error al escribir en sysfs: {}. ¿Configuraste udev?", e),
            }
        } else {
            println!("Acción ignorada: No hay hardware conectado.");
        }
    });

    ui.run()
}