use actix_files::NamedFile;
use actix_web::{get, Error, HttpRequest, Result};
use std::path::PathBuf;

#[get("/static/{filename:.*}")]
async fn leer_archivo_estatico(req: HttpRequest) -> Result<NamedFile, Error> {
    let ruta: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut ruta_string = ruta.into_os_string().into_string().unwrap();
    ruta_string = format!("./static/{}", ruta_string);
    let archivo = NamedFile::open(ruta_string)?;

    Ok(archivo.use_last_modified(true))
}
