use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use tera::{Context, Tera};

#[derive(Debug, Display, Error)]
pub enum MiError {
    #[display(fmt = "No encontrado")]
    PaginaNoEncontrada,

    #[display(fmt = "Ha ocurrido un error, estamos investigando la causa")]
    ErrorGeneral,
}

impl error::ResponseError for MiError {
    fn error_response(&self) -> HttpResponse {
        // Aca creamos la nueva instancia de tera solo para manejo de errores
        let mut context = Context::new();
        let tera: Tera = Tera::new("templates/**/*").unwrap();

        // Agregamos el status code y el mensaje
        context.insert("status_code", &self.status_code().to_string());
        context.insert("mensaje", &self.to_string());

        let render = tera.render("errores/error.html", &context).unwrap();

        // Ahora mostramos el template
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(render)
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MiError::PaginaNoEncontrada => StatusCode::NOT_FOUND,
            MiError::ErrorGeneral => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
