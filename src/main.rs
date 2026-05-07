mod config;  
mod organizador;

use config::Config;
use organizador::organizar_archivo;
use notify::{Watcher, RecursiveMode, recommended_watcher};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    // 1. Cargamos la configuración inteligente
    let conf = Config::leer().expect("Error al cargar la configuración");

    // 2. Convertimos el formato del TOML al HashMap que usa tu organizador
    let mut categorias_map: HashMap<String, String> = HashMap::new();
    for (carpeta, extensiones) in &conf.organizacion {
        for ext in extensiones {
            categorias_map.insert(ext.clone(), carpeta.clone());
        }
    }
    
    // Usamos la ruta que viene del config (ya sea la del TOML o la automática)
    let ruta_descargas = &conf.ruta_descargas;
    
    println!("Vigilando: {}", ruta_descargas);

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = recommended_watcher(tx).unwrap();
    watcher.watch(Path::new(ruta_descargas), RecursiveMode::NonRecursive).unwrap();

    println!("Organizador activo. Presiona Ctrl+C para detener.");

    for evento in rx {
        match evento {
            Ok(e) => {
                for ruta in e.paths {
                    // Solo intentamos organizar si es un archivo que todavía existe
                    if ruta.is_file() {
                        // Pasamos el mapa de categorías generado desde el TOML
                        if let Err(err) = organizar_archivo(&ruta, &categorias_map) {
                            println!("Error al organizar: {}", err);
                        }
                    }
                }
            }
            Err(e) => println!("Error en el watcher: {}", e),
        }
    }
}