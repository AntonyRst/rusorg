mod categorias;
mod organizador;

use categorias::obtener_categorias;
use organizador::organizar_archivo;
use notify::{Watcher, RecursiveMode, recommended_watcher};
use std::path::Path;

fn main() {
    let categorias = obtener_categorias();
    let home = std::env::var("HOME").unwrap();
    let ruta_descargas = format!("{}/Descargas", home);
    
    println!("Vigilando: {}", ruta_descargas);

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = recommended_watcher(tx).unwrap();
    watcher.watch(Path::new(&ruta_descargas), RecursiveMode::NonRecursive).unwrap();

    println!("Organizador activo. Presiona Ctrl+C para detener.");

    for evento in rx {
        println!("--- Evento detectado ---");
        match evento {
            Ok(e) => {
                for ruta in e.paths {
                    if ruta.is_file() {
                        if let Err(err) = organizar_archivo(&ruta, &categorias) {
                            println!("Error al organizar: {}", err);
                        }
                    }
                }
            }
            Err(e) => println!("Error en el watcher: {}", e),
        }
    }
}
