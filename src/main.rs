mod archivos_estaticos;
mod manejo_errores;
mod posts;

use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use archivos_estaticos::leer_archivo_estatico;
use manejo_errores::MiError;
use posts::{obtener_post, obtener_todos_los_posts, Post};
use tera::{Context, Tera};

// Es importante notar que para acceder al State que se guardo
// con .app_data, se coloca como parametro en la definicion del metodo
// el web::Data<T> donde T es el tipo de dato que pasamos en .app_data
// Lo genial es que si no necesitamos utilizar Tera, simplemente podemos
// quitarlo de la definicion, es decir, no es obligatorio poner en las
// definicionnes de las funciones todo lo que se ecuentre en el state
#[get("/")]
async fn index(tera: web::Data<Tera>) -> Result<HttpResponse, MiError> {
    // Ahora creamos el Context para pasarle valores a nuestro template HTML
    // es importante notar que le agregamos un "mut" porque vamos a modificar
    // sus valores
    let mut context = Context::new();
    // Ahora agregamos el {{nombre}} que espera nuestro template
    // para  ello el key debe coincidir con lo especificado en el template
    // en este caso el valor que se pasa a "posts" son todos los Posts que seran serializados por
    // Serde
    let posts: Vec<Post> = obtener_todos_los_posts();
    if posts.len() == 0 {
        return Err(MiError::ErrorGeneral);
    }
    context.insert("posts", &posts);
    // Ahora hacemos el render de nuestro template, pasando los valores
    // del contexto para ser reemplazados al momento del render.
    let respuesta_html = tera.render("blog/posts.html", &context).unwrap();
    Ok(HttpResponse::Ok().body(respuesta_html))
}

// Esta es la forma de pasar un parametro por la ruta del url, en la definicion
// de la funcion, es importante poder definir el tipo de los parametros
#[get("/{id}")]
async fn post(tera: web::Data<Tera>, path: web::Path<String>) -> Result<HttpResponse, MiError> {
    let mut context = Context::new();

    //  el parametro "path" contiene todos los parametros pasados por url, en
    //  este especifico caso solo es uno de tipo String, si hubierann mas, entonces
    //  habria que especificar cada tipo o crear una estructura que contenga esos
    //  parametros, puedes aprender mas de parametros de ruta en la documentacion:
    //  https://actix.rs/docs/extractors#path

    let id = path.into_inner(); // obteniendo el id
    if let Some(post) = obtener_post(&id) {
        // Si encuentra un post verificamos si tiene publicado = true
        if post.publicado {
            context.insert("post", &post);
            let respuesta_html = tera.render("blog/post.html", &context).unwrap();
            Ok(HttpResponse::Ok().body(respuesta_html))
        } else {
            // Si no esta publicado todavia, entonces regresaremos un error 404,
            Err(MiError::PaginaNoEncontrada)
        }
    } else {
        // En este caso si o encuentra el post, retornaremos un error 404 (Not Found)
        Err(MiError::PaginaNoEncontrada)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // Creamos una configuracion de Tera, en la cual se especificamos
        // que todos los  templates se encuetran en la carpeta "templates"
        // en la raiz del proyecto y que ademas dicha carpeta puede contener sub carpetas
        let tera: Tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .service(leer_archivo_estatico)
            // Aca agregamos al "State" nuestra configuracio de tera
            // para agregar al State, utilizamos el metodo .app_data
            // al cual le pasamos de parametro un web::Data para poder
            // manejar de forma segura la memoria junto con la configuracion
            // de tera
            .app_data(web::Data::new(tera))
            .service(index)
            .service(post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
