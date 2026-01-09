mod boveda;
use boveda::{Boveda, Entrada};
use std::io::{self, Write};
use arboard::Clipboard; // Importamos la herramienta del portapapeles

fn pedir_input(mensaje: &str) -> String {
    print!("{}", mensaje);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

// --- MAIN ---
fn main() {
    let nombre_archivo = "mis_claves.db";

    println!("--- 🔒 GESTOR DE CLAVES SEGURO (RUST) ---");
    print!("🔑 Introduce tu Contraseña Maestra: ");
    io::stdout().flush().unwrap();
    let password = rpassword::read_password().unwrap();

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

    loop {
        println!("\n--- MENÚ PRINCIPAL ---");
        println!("1. Agregar nueva contraseña");
        println!("2. Ver todas las contraseñas");
        println!("3. Buscar y Copiar"); // <-- Actualizado
        println!("4. Guardar y Salir");
        println!("5. Eliminar contraseña y guardar");
        println!("6. Modificar contraseña");
        
        let opcion = pedir_input("Elige una opción: ");

        match opcion.as_str() {
            "1" => {
                let servicio = pedir_input("Servicio: ");
                let usuario = pedir_input("Usuario: ");
                let clave = pedir_input("Contraseña: ");
                mi_boveda.agregar(Entrada { servicio, usuario, clave });
                println!("✅ Entrada agregada.");
            },
            "2" => {
                println!("\n--- TUS CLAVES ---");
                for (i, entrada) in mi_boveda.entradas.iter().enumerate() {
                    // Ocultamos la clave visualmente por seguridad
                    println!("{}. [{}] Usuario: {} | Clave: ****", 
                        i + 1, entrada.servicio, entrada.usuario);
                }
            },
            "3" => {
                let busqueda = pedir_input("¿Qué servicio buscas?: ").to_lowercase();
                
                // Filtramos y guardamos referencia al original
                let encontrados: Vec<&Entrada> = mi_boveda.entradas.iter()
                    .filter(|e| e.servicio.to_lowercase().contains(&busqueda))
                    .collect();

                if encontrados.is_empty() {
                    println!("❌ No se encontró nada.");
                } else {
                    println!("🔎 Resultados encontrados:");
                    // Mostramos índice local (1, 2, 3...)
                    for (i, entrada) in encontrados.iter().enumerate() {
                        println!("{}. [{}] Usuario: {}", i + 1, entrada.servicio, entrada.usuario);
                    }

                    // PREGUNTAR SI QUIERE COPIAR
                    println!("---");
                    let seleccion = pedir_input("Escribe el número para COPIAR la clave (o 0 para cancelar): ");
                    
                    // Convertimos el texto a número (usize)
                    if let Ok(indice) = seleccion.parse::<usize>() {
                        if indice > 0 && indice <= encontrados.len() {
                            let entrada_elegida = encontrados[indice - 1];
                            
                            // INTENTAMOS COPIAR AL PORTAPAPELES
                            match Clipboard::new() {
                                Ok(mut clipboard) => {
                                    // setText pone el texto en el portapapeles
                                    if let Err(e) = clipboard.set_text(&entrada_elegida.clave) {
                                        println!("❌ Error al copiar: {}", e);
                                    } else {
                                        println!("✨ ¡Clave de {} copiada al portapapeles! (Ya puedes hacer Ctrl+V)", entrada_elegida.servicio);
                                    }
                                },
                                Err(e) => println!("❌ No pude acceder al portapapeles: {}", e),
                            }
                        } else if indice != 0 {
                            println!("❌ Número inválido.");
                        }
                    }
                }
            },
            "4" => {
                match mi_boveda.guardar(nombre_archivo, &password) {
                    Ok(_) => println!("💾 Guardado. ¡Hasta luego!"),
                    Err(e) => println!("❌ Error al guardar: {}", e),
                }
                break;
            },
            "5" => {
                println!("\n--- ELIMINAR CONTRASEÑA ---");
                for (i, entrada) in mi_boveda.entradas.iter().enumerate() {
                    // Ocultamos la clave visualmente por seguridad
                    println!("{}. [{}] Usuario: {} | Clave: ****", 
                        i + 1, entrada.servicio, entrada.usuario);
                }

                // PREGUNTAR SI QUIERE COPIAR
                println!("---");
                let seleccion = pedir_input("Escribe el número de contraseña a borrar (0 para cancelar): ");
                // Convertimos el texto a número (usize)
                if let Ok(indice) = seleccion.parse::<usize>() {
                    if indice > 0 {
                        match mi_boveda.eliminar(indice - 1) {
                            Ok(_) => {
                                println!("Contraseña eliminada exitosamente!");
                                match mi_boveda.guardar(nombre_archivo, &password) {
                                    Ok(_) => println!("💾 Guardado. ¡Hasta luego!"),
                                    Err(e) => println!("❌ Error al guardar: {}", e),
                                }
                            },
                            Err(e) => println!("❌ No pude borrar la contraseña: {}", e),
                        }
                    } else {
                        println!("Operación cancelada.");
                    } 
                } else  {
                    println!("❌ Eso no es un número válido.");
                }

            },
            "6" => {
                println!("\n--- MODIFICAR CONTRASEÑA ---");
                // Listamos...
                for (i, entrada) in mi_boveda.entradas.iter().enumerate() {
                    println!("{}. [{}] Usuario: {} | Clave: ****", 
                        i + 1, entrada.servicio, entrada.usuario);
                }
                
                println!("---");
                let seleccion = pedir_input("Escribe el número del servicio a actualizar (0 cancelar): ");
                
                if let Ok(indice) = seleccion.parse::<usize>() {
                    if indice > 0 && indice <= mi_boveda.entradas.len() {
                        
                        // MEJORA 1: Pedir password oculto (como la clave maestra)
                        print!("Escribe la NUEVA contraseña (no se verá): ");
                        io::stdout().flush().unwrap();
                        let nuevo_password = rpassword::read_password().unwrap();

                        // MEJORA 2: Usar is_empty() que es más rápido y eficiente que chars().count()
                        if nuevo_password.trim().is_empty() {
                            println!("❌ Error: La contraseña no puede estar vacía. Operación cancelada.");
                        } else {
                            // Solo entramos aquí si hay contraseña real
                            match mi_boveda.editar(indice - 1, nuevo_password) {
                                Ok(_) => {
                                    println!("✨ ¡La clave se ha modificado exitosamente!");
                                    match mi_boveda.guardar(nombre_archivo, &password) {
                                        Ok(_) => println!("💾 Cambios guardados en disco."),
                                        Err(e) => println!("❌ Error al guardar: {}", e),
                                    }
                                },
                                Err(e) => println!("❌ Error al modificar: {}", e),
                            }
                        }

                    } else if indice != 0 {
                        println!("❌ Número inválido.");
                    }
                } else {
                    println!("❌ Eso no es un número.");
                }
            },
            _ => println!("❌ Opción no válida."),
        }
    }
}