use actix_web::{App, HttpServer};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use rust_pastebin::{db::DatabasePool, env::Env, handlers::paste::paste_router};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_vars = Env::init();

    let pool = DatabasePool::init(env_vars.database_url).await;

    let server = HttpServer::new(move || {
        let (app, openapi) = App::new()
            .into_utoipa_app()
            .configure(|cfg| paste_router(cfg, pool.clone()))
            .split_for_parts();

        // Connect Swagger
        app.service(SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", openapi))
    })
    .bind((env_vars.host.clone(), env_vars.port))?;

    println!(
        "Server is running on http://{}:{}",
        env_vars.host, env_vars.port
    );

    server.run().await
}
