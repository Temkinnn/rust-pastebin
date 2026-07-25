use actix_web::{App, HttpServer};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use rust_pastebin::{db::DatabasePool, env::Env};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_vars = Env::init();

    let _pool = DatabasePool::init(env_vars.database_url);

    let server = HttpServer::new(|| {
        let (app, openapi) = App::new().into_utoipa_app().split_for_parts();

        // Connect Swagger
        app.service(SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", openapi))
    })
    .bind((env_vars.host, env_vars.port))?;

    println!();
    
    server.run().await
}
