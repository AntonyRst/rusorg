use std::collections::HashMap;
use std::path::Path;
use std::thread;
use std::time::Duration;

// Usamos dos &str porque HashMap obtiene la llave y el valor.
// HashMap<llave, valor> --- llave: "jpg", "pdf", "mp3" --- valor: "Imágenes", "Documentos", "Música".
pub fn organizar_archivo(ruta: &Path, categorias: &HashMap<&str, &str>) -> Result<(), std::io::Error> {
    thread::sleep(Duration::from_millis(200));

    let nombre_completo = ruta.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    let extension = if let Some(pos) = nombre_completo.rfind('.') {
        &nombre_completo[pos + 1..]
    } else {
        nombre_completo
    }.to_lowercase();

    if extension.is_empty() || extension == "part" || extension == "crdownload" || extension == "tmp" {
        return Ok(());
    }

    if let Some(carpeta) = categorias.get(extension.as_str()) {
        let home = std::env::var("HOME").unwrap();
        let destino_dir = format!("{}/Descargas/{}", home, carpeta);
        std::fs::create_dir_all(&destino_dir)?;

        let destino_final = format!("{}/{}", destino_dir, nombre_completo);

        // Intentamos moverlo
        match std::fs::rename(ruta, &destino_final) {
            Ok(_) => println!("Movido: {} -> {}", nombre_completo, carpeta),
            Err(e) => println!("Error al mover {}: {}", nombre_completo, e),
        }
    } else {
        println!("Ignorado (sin categoría): {} (ext: {})", nombre_completo, extension);
    }
    Ok(())
}