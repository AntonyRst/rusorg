use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    pub ruta_descargas: String,
    pub organizacion: HashMap<String, Vec<String>>,
}

// Aquí ocurre la magia: Rust mete el archivo dentro del binario al compilar
const DEFAULT_CONFIG_CONTENT: &str = include_str!("default_config.toml");

impl Config {
    pub fn leer() -> Self {
        let home = std::env::var("HOME").expect("No se encontró la variable HOME");
        let ruta_dir = format!("{}/.config/rusorg", home);
        let ruta_archivo = format!("{}/config.toml", ruta_dir);

        // Si el archivo no existe, lo creamos usando el contenido incrustado
        if !std::path::Path::new(&ruta_archivo).exists() {
            std::fs::create_dir_all(&ruta_dir).expect("No se pudo crear la carpeta ~/.config/rusorg");
            std::fs::write(&ruta_archivo, DEFAULT_CONFIG_CONTENT).expect("No se pudo crear el config.toml");
        }

        // Leemos el archivo (ya sea el nuevo o el que ya existía)
        let contenido = std::fs::read_to_string(&ruta_archivo).expect("No se pudo leer el config");
        
        let mut config: Config = toml::from_str(&contenido).expect("Formato TOML inválido");

        // Lógica de ruta automática
        if config.ruta_descargas.is_empty() {
            config.ruta_descargas = format!("{}/Descargas", home);
        }

        config
    }
}