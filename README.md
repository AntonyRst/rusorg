# rusorg

Organizador automático de descargas escrito en Rust. Vigila la carpeta `~/Descargas` en tiempo real y mueve cada archivo a su subcarpeta correcta según su extensión — sin intervención manual y con consumo mínimo de recursos.

## ¿Cómo funciona?

Cuando detecta un archivo nuevo en `~/Descargas` lo analiza y lo mueve automáticamente:

```
~/Descargas/foto.jpg        → ~/Descargas/Imágenes/
~/Descargas/documento.pdf   → ~/Descargas/Documentos/
~/Descargas/cancion.mp3     → ~/Descargas/Música/
~/Descargas/video.mp4       → ~/Descargas/Videos/
~/Descargas/programa.deb    → ~/Descargas/Software/
~/Descargas/archivo.zip     → ~/Descargas/Comprimidos/
~/Descargas/main.rs         → ~/Descargas/Código/
```

Si la carpeta destino no existe la crea automáticamente.

## Categorías soportadas

| Carpeta | Extensiones |
|---------|-------------|
| Imágenes | jpg, jpeg, png, gif, svg, webp, bmp, ico, tiff |
| Documentos | pdf, doc, docx, txt, xlsx, pptx, odt, epub, csv |
| Videos | mp4, mkv, avi, mov, flv, wmv, webm |
| Música | mp3, wav, flac, ogg, m4a, aac |
| Software | exe, msi, appimage, deb, rpm, bin, sh |
| Comprimidos | zip, rar, 7z, tar, gz, xz, bz2 |
| Código | rs, py, js, html, css, scss, c, cpp, json, gd |
| Otros | iso, db, bak |

## Instalación

Necesitas tener [Rust](https://www.rust-lang.org/) instalado.

```bash
git clone https://github.com/AntonyRst/rusorg
cd rusorg
cargo build --release
sudo cp target/release/rusorg /usr/local/bin/rusorg
```

## Ejecutar como servicio (Linux)

Para que arranque automáticamente con el sistema:

```bash
sudo nano /etc/systemd/system/rusorg.service
```

Contenido del archivo:

```ini
[Unit]
Description=Organizador automático de Descargas
After=network.target

[Service]
Type=simple
User=tu_usuario
ExecStart=/usr/local/bin/rusorg
Restart=on-failure

[Install]
WantedBy=default.target
```

Activar el servicio:

```bash
sudo systemctl enable rusorg
sudo systemctl start rusorg
sudo systemctl status rusorg
```

## Autor

Jhon