use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use std::fs;

pub fn organizar_archivo(ruta: &Path, categorias: &HashMap<String, String>) -> Result<(), std::io::Error> {
    // 1. Pausa de seguridad para asegurar que el archivo terminó de escribirse
    thread::sleep(Duration::from_millis(500));

    // 2. Obtener el nombre completo del archivo (ej: "foto.jpg" o "jpg")
    let nombre_completo = ruta.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    // 3. Intentar obtener la extensión real (.jpg, .pdf)
    let mut extension = ruta.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    // 4. LÓGICA ESPECIAL: Si no hay extensión, usamos el nombre como extensión
    // Esto permite que archivos llamados literalmente "jpg" sean procesados
    if extension.is_empty() {
        extension = nombre_completo.to_lowercase();
    }

    // Limpiamos la extensión de posibles paréntesis como "jpg(1)" para la búsqueda
    let extension_busqueda = extension.split('(').next().unwrap_or(&extension).trim();

    // 5. Filtro de archivos temporales
    if ["part", "crdownload", "tmp"].contains(&extension_busqueda) {
        return Ok(());
    }

    // 6. Si la extensión (o nombre) coincide con una categoría de tu config.toml
    if let Some(carpeta) = categorias.get(extension_busqueda) {
        let home = std::env::var("HOME").expect("Variable HOME no definida");
        let destino_dir = PathBuf::from(home).join("Descargas").join(carpeta);
        
        // Crear la carpeta si no existe (ej: Descargas/Imagenes)
        fs::create_dir_all(&destino_dir)?;

        // Construir la ruta de destino original
        let ruta_tentativa = destino_dir.join(nombre_completo);

        // APLICAR LÓGICA DE DUPLICADOS (genera nombre_1.jpg si ya existe)
        let ruta_final = generar_ruta_unica(ruta_tentativa);

        match fs::rename(ruta, &ruta_final) {
            Ok(_) => println!("✓ Movido: {} -> {:?}", nombre_completo, ruta_final.file_name().unwrap()),
            Err(e) => eprintln!("x Error al mover {}: {}", nombre_completo, e),
        }
    }
    
    Ok(())
}

/// Función interna para encontrar un nombre de archivo disponible
fn generar_ruta_unica(ruta_inicial: PathBuf) -> PathBuf {
    let mut ruta = ruta_inicial.clone();
    
    if !ruta.exists() {
        return ruta;
    }

    let carpeta = ruta.parent().unwrap().to_path_buf();
    let nombre_base = ruta.file_stem().unwrap().to_string_lossy().to_string();
    let extension = ruta.extension().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();

    let mut contador = 1;
    while ruta.exists() {
        let nuevo_nombre = if extension.is_empty() {
            format!("{}_{}", nombre_base, contador)
        } else {
            format!("{}_{}.{}", nombre_base, contador, extension)
        };
        ruta = carpeta.join(nuevo_nombre);
        contador += 1;
    }

    ruta
}