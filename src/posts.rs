use serde::Serialize;

#[derive(Serialize)]
pub struct Post {
    pub id: String,
    pub titulo: String,
    pub description_corta: String,
    pub autor: String,
    pub avatar: String,
    pub imagen_encabezado: String,
    pub contenido: String,
    pub publicado: bool,
    pub fecha_publicacion: String,
}

pub fn obtener_todos_los_posts() -> Vec<Post> {
    vec![
        Post {
            id: String::from("mi-post"),
            titulo: String::from("Mi Post"),
            description_corta: String::from("Este es mi <b>primer post</b>"),
            autor: String::from("Rusty Full Stack"),
            avatar: String::from("/static/img/avatar.jpg"),
            imagen_encabezado: String::from("/static/img/post1.jpg"),
            contenido: String::from(
                r#"<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit.</p> Proin sit amet dictum ipsum, eu volutpat nulla. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Nam porttitor felis non fringilla fringilla. Sed vitae ultrices eros. Suspendisse quis nibh vel sapien volutpat venenatis sit amet ut eros. Aliquam sit amet tellus non tortor dapibus scelerisque. Etiam finibus hendrerit nibh, nec vestibulum nisl porta et. Nullam imperdiet est sit amet scelerisque tempus. Ut nibh ipsum, fringilla vitae nisi ut, gravida blandit erat. Praesent venenatis ante eu venenatis pretium. Morbi mollis arcu sapien, id condimentum eros porta ac.
    Aliquam feugiat eros vitae nisi ultrices, dictum tincidunt risus volutpat. Ut eleifend turpis eget fringilla congue. Aenean in tortor lobortis, vehicula eros ac, fringilla arcu. Fusce volutpat justo ut turpis convallis facilisis. Cras et blandit purus. Nunc convallis consequat libero. Donec mattis tortor sit amet vestibulum gravida. Donec nec egestas quam, quis pharetra est. Praesent id ante sed mauris auctor malesuada. Donec lobortis ultricies feugiat. Nullam vestibulum feugiat porta."#,
            ),
            publicado: true,
            fecha_publicacion: String::from("2024-02-09"),
        },
        Post {
            id: String::from("creado-templates-html-con-actix-web"),
            titulo: String::from("Creado Templates HTML con actix-web"),
            description_corta: String::from("Aprendiendo actix-web con <a href='https://rustyfullstack.com'>rustyfullstack.com!</a>"),
            autor: String::from("Rusty Full Stack"),
            imagen_encabezado: String::from("/static/img/post2.jpg"),
            avatar: String::from("/static/img/avatar.jpg"),
            contenido: String::from(
                r#"<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin sit amet dictum ipsum, eu volutpat nulla.</p> Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Nam porttitor felis non fringilla fringilla. Sed vitae ultrices eros. Suspendisse quis nibh vel sapien volutpat venenatis sit amet ut eros. Aliquam sit amet tellus non tortor dapibus scelerisque. Etiam finibus hendrerit nibh, nec vestibulum nisl porta et. Nullam imperdiet est sit amet scelerisque tempus. Ut nibh ipsum, fringilla vitae nisi ut, gravida blandit erat. Praesent venenatis ante eu venenatis pretium. Morbi mollis arcu sapien, id condimentum eros porta ac.
    Aliquam feugiat eros vitae nisi ultrices, dictum tincidunt risus volutpat. Ut eleifend turpis eget fringilla congue. Aenean in tortor lobortis, vehicula eros ac, fringilla arcu. Fusce volutpat justo ut turpis convallis facilisis. Cras et blandit purus. Nunc convallis consequat libero. Donec mattis tortor sit amet vestibulum gravida. Donec nec egestas quam, quis pharetra est. Praesent id ante sed mauris auctor malesuada. Donec lobortis ultricies feugiat. Nullam vestibulum feugiat porta."#,
            ),
            publicado: true,
            fecha_publicacion: String::from("2024-02-09"),
        },
    ]
}

pub fn obtener_post(id: &str) -> Option<Post> {
    let posts: Vec<Post> = obtener_todos_los_posts();
    if let Some(post_encontrado) = posts.into_iter().find(|post| post.id == id) {
        Some(post_encontrado)
    } else {
        None
    }
}
