use std::{collections::HashMap};

// No necesita recibir parámetros, ya que la fn crea el HashMap desde cero con las categorías predefinidas y lo devuelve.
pub fn obtener_categorias() -> HashMap<&'static str, &'static str> {

    // Creamos el HashMap nuevo que recibira las categoras ya predifinidas.
    let mut categorias: HashMap<&str, &str> = HashMap::new();

    //Categorias para la Carpeta Imágenes.
    categorias.insert("jpg", "Imágenes");
    categorias.insert("jpeg", "Imágenes");
    categorias.insert("png", "Imágenes");
    categorias.insert("gif", "Imágenes");
    categorias.insert("svg", "Imágenes");
    categorias.insert("webp", "Imágenes");
    categorias.insert("bmp", "Imágenes");
    categorias.insert("ico", "Imágenes");
    categorias.insert("tiff", "Imágenes");

    //Categorias para la Carpeta Documentos.
    categorias.insert("pdf", "Documentos");
    categorias.insert("docx", "Documentos");
    categorias.insert("doc", "Documentos");
    categorias.insert("txt", "Documentos");
    categorias.insert("xlsx", "Documentos");
    categorias.insert("pptx", "Documentos");
    categorias.insert("odt", "Documentos");
    categorias.insert("epub", "Documentos");
    categorias.insert("csv", "Documentos");

    // Categorias para la Carpeta Videos.
    categorias.insert("mp4", "Videos");
    categorias.insert("mkv", "Videos");
    categorias.insert("avi", "Videos");
    categorias.insert("mov", "Videos");
    categorias.insert("flv", "Videos");
    categorias.insert("wmv", "Videos");
    categorias.insert("webm", "Videos");

    // Categorias para la Carpeta Música.
    categorias.insert("mp3", "Música");
    categorias.insert("wab", "Música");
    categorias.insert("flac", "Música");
    categorias.insert("ogg", "Música");
    categorias.insert("m4a", "Música");
    categorias.insert("aac", "Música");

    // Categorias para la Carpeta Software.
    categorias.insert("exe", "Software");
    categorias.insert("msi", "Software");
    categorias.insert("appimage", "Software");
    categorias.insert("deb", "Software");
    categorias.insert("rpm", "Software");
    categorias.insert("bin", "Software");
    categorias.insert("sh", "Software");

    // Categorias para la Carpeta Comprimidos.
    categorias.insert("rar", "Comprimidos");
    categorias.insert("7z", "Comprimidos");
    categorias.insert("tar", "Comprimidos");
    categorias.insert("gz", "Comprimidos");
    categorias.insert("xz", "Comprimidos");
    categorias.insert("bz2", "Comprimidos");

    // Categorias para la Carpeta Código.
    categorias.insert("js", "Código");
    categorias.insert("html", "Código");
    categorias.insert("css", "Código");
    categorias.insert("scss", "Código");
    categorias.insert("rs", "Código");
    categorias.insert("py", "Código");
    categorias.insert("c", "Código");
    categorias.insert("cpp", "Código");
    categorias.insert("gd", "Código");
    categorias.insert("json", "Código");

    // Categorias para la Carpeta Otros.
    categorias.insert("iso", "Otros");
    categorias.insert("db", "Otros");
    categorias.insert("bak", "Otros");
    
categorias
}