use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use tracing::{info, instrument};
use utoipa_actix_web::{scope, service_config::ServiceConfig};

use crate::{
    error::AppError,
    models::paste::{CreatePasteDto, GetPastesQuery, Paste, PasteResponse, UpdatePasteDto},
    repositories::paste::PasteRepository,
    services::paste::PasteService,
    types::{AppResult, Database, Id},
};

#[utoipa::path(
    tag = "Paste",
    request_body = CreatePasteDto,
    description = "Create paste",
    responses(
        (status = 201, body = PasteResponse),
        (status = 500, body = AppError, description = "Internal or Database error",
            example = json!({
                "error": "Database Error"
            })),
    )
)]
#[post("")]
#[instrument(name = "Create paste handler", skip(service, body))]
async fn create_paste(
    service: web::Data<PasteService>,
    body: web::Json<CreatePasteDto>,
) -> AppResult<impl Responder> {
    let paste = service.create_paste(body.into_inner()).await?;
    Ok(HttpResponse::Created().json(paste))
}

#[utoipa::path(
    tag = "Paste",
    description = "Get pastes",
    params(GetPastesQuery),
    responses(
        (status = 200, body = [PasteResponse]),
        (status = 500, body = AppError, description = "Internal or Database error",
            example = json!({
                "error": "Database Error"
            })),
    )
)]
#[get("")]
#[instrument(name = "Get pastes handler", skip(service, query))]
async fn get_pastes(
    service: web::Data<PasteService>,
    query: web::Query<GetPastesQuery>,
) -> AppResult<impl Responder> {
    let pastes = service.get_pastes(query.limit, query.offset).await?;
    info!(query.limit, query.offset);
    Ok(HttpResponse::Ok().json(pastes))
}

#[utoipa::path(
    tag = "Paste",
    description = "Get paste by id",
    params(
        ("id", Path, description = "Id")
    ),
    responses(
        (status = 200, body = Paste),
        (status = 404, body = AppError, description = "Paste with such id does not exist",
            example = json!({
                "error": "Resource not found"
            })),
        (status = 500, body = AppError, description = "Internal or Database error",
            example = json!({
                "error": "Database Error"
            })),
    )
)]
#[get("/{id}")]
#[instrument(name = "Get paste by id handler", skip(service, path))]
async fn get_paste_by_id(
    service: web::Data<PasteService>,
    path: web::Path<Id>,
) -> AppResult<impl Responder> {
    let paste = service.get_paste_by_id(path.into_inner()).await?;
    info!(paste.id);
    Ok(HttpResponse::Ok().json(paste))
}

#[utoipa::path(
    tag = "Paste",
    description = "Update paste data by id",
    params(
        ("id", Path, description = "Id")
    ),
    request_body = UpdatePasteDto,
    responses(
        (status = 200, body = Paste),
        (status = 404, body = AppError, description = "Paste with such id does not exist",
            example = json!({
                "error": "Resource not found"
            })),
        (status = 500, body = AppError, description = "Internal or Database error",
            example = json!({
                "error": "Database Error"
            })),
    )
)]
#[patch("/{id}")]
#[instrument(name = "Update paste handler", skip(service, path, body))]
async fn update_paste_data(
    service: web::Data<PasteService>,
    path: web::Path<Id>,
    body: web::Json<UpdatePasteDto>,
) -> AppResult<impl Responder> {
    let paste = service
        .update_paste(path.into_inner(), body.into_inner())
        .await?;
    info!(paste.id);
    Ok(HttpResponse::Ok().json(paste))
}

#[utoipa::path(
    tag = "Paste",
    description = "Delete paste by id",
    params(
        ("id", Path, description = "Id")
    ),
    responses(
        (status = 200, body = Paste),
        (status = 404, body = AppError, description = "Paste with such id does not exist",
            example = json!({
                "error": "Resource not found"
            })),
        (status = 500, body = AppError, description = "Internal or Database error",
            example = json!({
                "error": "Database Error"
            })),
    )
)]
#[delete("/{id}")]
#[instrument(name = "Delete paste handler", skip(service, path))]
async fn delete_paste_by_id(
    service: web::Data<PasteService>,
    path: web::Path<Id>,
) -> AppResult<impl Responder> {
    let paste = service.delete_paste_by_id(path.into_inner()).await?;
    info!(paste.id);
    Ok(HttpResponse::Ok().json(paste))
}

pub fn paste_router(cfg: &mut ServiceConfig, pool: Database) {
    let repo = PasteRepository::new(pool);
    let service = PasteService::new(repo);

    cfg.app_data(web::Data::new(service)).service(
        scope("/paste")
            .service(create_paste)
            .service(get_pastes)
            .service(get_paste_by_id)
            .service(update_paste_data)
            .service(delete_paste_by_id),
    );
}
