use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
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
    description = "Создать запись",
    responses(
        (status = 201, body = PasteResponse),
        (status = 500, body = AppError, description = "Внутренняя ошибка сервера или БД",
            example = json!({
                "error": "Database Error"
            })),
    )
)]
#[post("")]
async fn create_paste(
    service: web::Data<PasteService>,
    body: web::Json<CreatePasteDto>,
) -> AppResult<impl Responder> {
    let data = body.into_inner();

    let paste = service.create_paste(data).await?;
    Ok(HttpResponse::Created().json(paste))
}

#[utoipa::path(
    tag = "Paste",
    description = "Получить список записей",
    params(GetPastesQuery),
    responses(
        (status = 200, body = [PasteResponse]),
        (status = 500, body = AppError, description = "Внутренняя ошибка сервера или БД",
            example = json!({
                "error": "Database Error"
            })),
    )
)]
#[get("")]
async fn get_pastes(
    service: web::Data<PasteService>,
    query: web::Query<GetPastesQuery>,
) -> AppResult<impl Responder> {
    let pastes = service.get_pastes(query.limit, query.offset).await?;
    Ok(HttpResponse::Ok().json(pastes))
}

#[utoipa::path(
    tag = "Paste",
    description = "Получить запись по ее Id",
    params(
        ("id", Path, description = "Id записи")
    ),
    responses(
        (status = 200, body = Paste),
        (status = 404, body = AppError, description = "Запись с таким Id не существует",
            example = json!({
                "error": "Resource not found"
            })),
        (status = 500, body = AppError, description = "Внутренняя ошибка сервера или БД",
            example = json!({
                "error": "Database Error"
            })),
    )
)]
#[get("/{id}")]
async fn get_paste_by_id(
    service: web::Data<PasteService>,
    path: web::Path<Id>,
) -> AppResult<impl Responder> {
    let id = path.into_inner();

    let paste = service.get_paste_by_id(id).await?;
    Ok(HttpResponse::Ok().json(paste))
}

#[utoipa::path(
    tag = "Paste",
    description = "Изменить данные записи по Id",
    params(
        ("id", Path, description = "Id записи")
    ),
    request_body = UpdatePasteDto,
    responses(
        (status = 200, body = Paste),
        (status = 404, body = AppError, description = "Запись с таким Id не существует",
            example = json!({
                "error": "Resource not found"
            })),
        (status = 500, body = AppError, description = "Внутренняя ошибка сервера или БД",
            example = json!({
                "error": "Database Error"
            })),
    )
)]
#[patch("/{id}")]
async fn update_paste_data(
    service: web::Data<PasteService>,
    path: web::Path<Id>,
    body: web::Json<UpdatePasteDto>,
) -> AppResult<impl Responder> {
    let id = path.into_inner();
    let data = body.into_inner();

    let pastes = service.update_paste_data(id, data).await?;
    Ok(HttpResponse::Ok().json(pastes))
}

#[utoipa::path(
    tag = "Paste",
    description = "Удалить запись по Id",
    params(
        ("id", Path, description = "Id записи")
    ),
    responses(
        (status = 200, body = Paste),
        (status = 404, body = AppError, description = "Запись с таким Id не существует",
            example = json!({
                "error": "Resource not found"
            })),
        (status = 500, body = AppError, description = "Внутренняя ошибка сервера или БД",
            example = json!({
                "error": "Database Error"
            })),
    )
)]
#[delete("/{id}")]
async fn delete_paste_by_id(
    service: web::Data<PasteService>,
    path: web::Path<Id>,
) -> AppResult<impl Responder> {
    let id = path.into_inner();

    let paste = service.delete_paste_by_id(id).await?;
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
