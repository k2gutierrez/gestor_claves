mod boveda;
use boveda::{Boveda, Entrada};
use inquire::{Select, Text, Password, Confirm};
use arboard::Clipboard; // Importamos la herramienta del portapapeles

// --- MAIN ---
fn main() {
    let nombre_archivo = "mis_claves.db";

    println!("--- 🔒 GESTOR DE CLAVES SEGURO (RUST) ---");
    let password = Password::new("🔑 Introduce tu Contraseña Maestra:")
        .without_confirmation()
        .prompt();

    let password = match password {
        Ok(pass) => pass,
        Err(_) => {
            print!("Operación cancelada.");
            return;
        }
    };

    let mut mi_boveda = match Boveda::cargar(nombre_archivo, &password) {
        Ok(boveda) => {
            println!("✅ Bóveda desencriptada con éxito.");
            boveda
        },
        Err(_) => {
            println!("⚠️ Creando una bóveda nueva.");
            Boveda::nueva()
        }
    };

    // Definimos las opciones del menú como un vector de texto
    let opciones_menu = vec![
        "1. Agregar nueva contraseña",
        "2. Ver todas las contraseñas",
        "3. Buscar y Copiar",
        "4. Modificar contraseña",
        "5. Eliminar contraseña",
        "6. Guardar y Salir",
    ];

    loop {
        println!("\n--------------------------------");
        
        // 2. El Menú con Flechas
        let seleccion = Select::new("¿Qué deseas hacer?", opciones_menu.clone())
            .prompt(); // Muestra el menú interactivo

        match seleccion {
            Ok(opcion) => {
                // inquire nos devuelve el texto completo (ej: "1. Agregar..."), 
                // así que verificamos con cuál empieza.
                if opcion.starts_with("1") {
                    // --- AGREGAR ---
                    // Usamos Text::new para pedir datos limpios
                    let servicio = Text::new("Servicio (ej. Facebook):").prompt().unwrap();
                    let usuario = Text::new("Usuario/Email:").prompt().unwrap();
                    let clave = Text::new("Contraseña:").prompt().unwrap();

                    mi_boveda.agregar(Entrada { servicio, usuario, clave });
                    println!("✅ Entrada agregada.");
                } else if opcion.starts_with("2") {
                    // --- VER TODAS ---
                    println!("--- TUS CLAVES ---");
                    for (i, entrada) in mi_boveda.entradas.iter().enumerate() {
                        println!("{}. [{}] Usuario: {} | Clave: *****", i + 1, entrada.servicio, entrada.usuario);
                    }
                } else if opcion.starts_with("3") {
                    // --- BUSCAR Y COPIAR ---
                    let busqueda = Text::new("Buscar servicio:").prompt().unwrap().to_lowercase();
                    let encontrados: Vec<&Entrada> = mi_boveda.entradas.iter()
                        .filter(|e| e.servicio.to_lowercase().contains(&busqueda))
                        .collect();
                    
                    if encontrados.is_empty() {
                        println!("❌ No se encontró nada.");
                    } else {
                        // Creamos una lista de strings para el menú de selección
                        // Format! nos ayuda a crear textos dinámicos
                        let opciones_busqueda: Vec<String> = encontrados.iter()
                            .map(|e| format!("[{}] {}", e.servicio, e.usuario))
                            .collect();
                        
                        // Mostramos un sub-menú para elegir cuál copiar
                        let eleccion = Select::new("Selecciona para copiar:", opciones_busqueda).prompt();

                        if let Ok(seleccion_texto) = eleccion {
                            // Buscamos cuál eligió el usuario en base al texto
                            if let Some(entrada_elegida) = encontrados.iter().find(|e| format!("[{}] {}", e.servicio, e.usuario) == seleccion_texto) {
                                
                                // CASO A: NO estamos en Android (PC, Mac, Linux Desktop)
                                #[cfg(not(target_os = "android"))]
                                {
                                    match Clipboard::new() {
                                        Ok(mut clipboard) => {
                                            if let Err(e) = clipboard.set_text(&entrada_elegida.clave) {
                                                println!("❌ Error al copiar: {}", e);
                                            } else {
                                                println!("✨ ¡Clave de {} copiada! (Ctrl+V)", entrada_elegida.servicio);
                                            }
                                        },
                                        Err(_) => println!("❌ No pude acceder al portapapeles en este sistema."),
                                    }
                                }

                                // CASO B: SÍ estamos en Android
                                #[cfg(target_os = "android")]
                                {
                                    println!("📱 Modo Android detectado: El copiado automático está desactivado por seguridad/compatibilidad.");
                                    println!("🔑 Tu clave es: {}", entrada_elegida.clave);
                                    println!("(Puedes seleccionarla y copiarla manualmente)");
                                }

                            }
                        }
                    }
                } else if opcion.starts_with("4") {
                    // --- MODIFICAR ---
                    // Usamos Select para elegir qué modificar, en lugar de escribir índice
                    let opciones_editar: Vec<String> = mi_boveda.entradas.iter()
                        .enumerate()
                        .map(|(i, e)| format!("{}. [{}] {}", i + 1, e.servicio, e.usuario))
                        .collect();

                    let seleccion_editar = Select::new("Elige cuál modificar:", opciones_editar).prompt();

                    if let Ok(texto) = seleccion_editar {
                        // Extraemos el número del principio del string "1. [Facebook]..."
                        let partes: Vec<&str> = texto.split('.').collect();
                        if let Ok(indice) = partes[0].parse::<usize>() {
                            
                            // Pedimos la nueva clave oculta
                            let nueva_clave = Password::new("Nueva contraseña:")
                                .with_display_mode(inquire::PasswordDisplayMode::Masked) // Muestra * en vez de nada
                                .without_confirmation()
                                .prompt()
                                .unwrap();

                            if !nueva_clave.trim().is_empty() {
                                let _ = mi_boveda.editar(indice - 1, nueva_clave);
                                println!("✨ Modificada y guardada en memoria.");
                                // Podrías guardar auto aquí si quieres
                                let _ = mi_boveda.guardar(nombre_archivo, &password);
                            }
                        }
                    }
                } else if opcion.starts_with("5") {
                    // --- ELIMINAR ---
                    let opciones_borrar: Vec<String> = mi_boveda.entradas.iter()
                        .enumerate()
                        .map(|(i, e)| format!("{}. [{}] {}", i + 1, e.servicio, e.usuario))
                        .collect();

                    let seleccion_borrar = Select::new("❌ ELIMINAR: Elige cuál borrar:", opciones_borrar).prompt();

                    if let Ok(texto) = seleccion_borrar {
                        // Confirmación de seguridad
                        let seguro = Confirm::new("¿Estás seguro de que quieres borrarla para siempre?")
                            .with_default(false)
                            .prompt();

                        if let Ok(true) = seguro {
                            let partes: Vec<&str> = texto.split('.').collect();
                            if let Ok(indice) = partes[0].parse::<usize>() {
                                let _ = mi_boveda.eliminar(indice - 1);
                                println!("🗑️ Eliminada.");
                                let _ = mi_boveda.guardar(nombre_archivo, &password);
                            }
                        } else {
                            println!("Operación cancelada.");
                        }
                    }
                } else if opcion.starts_with("6") {
                    // --- SALIR ---
                    match mi_boveda.guardar(nombre_archivo, &password) {
                        Ok(_) => println!("💾 Guardado. ¡Hasta luego!"),
                        Err(e) => println!("❌ Error al guardar: {}", e),
                    }
                    break;
                }
            },
            Err(_) => {
                println!("Error en el menú o cancelación.");
                break;
            }
        }
    }
}