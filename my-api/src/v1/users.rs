use actix_web::{
    error::PathError,
    web::{self, PathConfig, ServiceConfig},
    HttpRequest, HttpResponse,
};
use tracing::instrument;
use uuid::Uuid;

use crate::{repository::Repository, user::User};

const PATH: &str = "/user";

pub fn service<R: Repository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope(PATH)
            .app_data(PathConfig::default().error_handler(path_config_handler))
            .route("/{user_id}", web::get().to(get::<R>))
            .route("/", web::post().to(post::<R>))
            .route("/", web::put().to(put::<R>))
            .route("/{user_id}", web::delete().to(delete::<R>)),
    );
}

#[instrument(skip(repo))]
async fn get<R: Repository>(user_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    match repo.get_user(&user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("Not found"),
    }
}

#[instrument(skip(repo))]
async fn post<R: Repository>(user: web::Json<User>, repo: web::Data<R>) -> HttpResponse {
    match repo.create_user(&user).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("Something went wrong: {}", e)),
    }
}

#[instrument(skip(repo))]
async fn put<R: Repository>(user: web::Json<User>, repo: web::Data<R>) -> HttpResponse {
    match repo.update_user(&user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().body(format!("Something went wrong: {}", e)),
    }
}

#[instrument(skip(repo))]
async fn delete<R: Repository>(user_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    match repo.delete_user(&user_id).await {
        Ok(id) => HttpResponse::Ok().body(id.to_string()),
        Err(e) => HttpResponse::InternalServerError().body(format!("Something went wrong: {}", e)),
    }
}

fn path_config_handler(err: PathError, _req: &HttpRequest) -> actix_web::Error {
    actix_web::error::ErrorBadRequest(err)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        repository::MockRepository,
        user::{CustomData, User},
    };
    use chrono::{NaiveDate, Utc};

    pub fn create_test_user(id: Uuid, name: String, birth_date_ymd: (i32, u32, u32)) -> User {
        let (year, month, day) = birth_date_ymd;
        User {
            id,
            name,
            birth_date: NaiveDate::from_ymd(year, month, day),
            custom_data: CustomData { random: 1 },
            created_at: Some(Utc::now()),
            updated_at: None,
        }
    }

    #[actix_rt::test]
    async fn get_works() {
        let user_id = uuid::Uuid::new_v4();
        let user_name = "User Name";
        let mut repo = MockRepository::default();

        repo.expect_get_user().returning(move |id| {
            let user = create_test_user(*id, user_name.to_string(), (1977, 03, 10));
            Ok(user)
        });

        let mut result = get(web::Path::from(user_id), web::Data::new(repo)).await;

        let user = result
            .take_body()
            .as_ref()
            .map(|b| match b {
                actix_web::dev::Body::Bytes(x) => serde_json::from_slice::<'_, User>(x).ok(),
                _ => None,
            })
            .flatten()
            .unwrap();

        assert_eq!(user.id, user_id);
        assert_eq!(user.name, user_name);
    }

    #[actix_rt::test]
    async fn create_works() {
        let user_id = uuid::Uuid::new_v4();
        let user_name = "User Name";
        let new_user = create_test_user(user_id, user_name.to_string(), (1977, 03, 10));
        let mut repo = MockRepository::default();

        repo.expect_create_user()
            .returning(move |user| Ok(user.to_owned()));

        let mut result = post(web::Json(new_user), web::Data::new(repo)).await;

        let user = result
            .take_body()
            .as_ref()
            .map(|b| match b {
                actix_web::dev::Body::Bytes(x) => serde_json::from_slice::<'_, User>(x).ok(),
                _ => None,
            })
            .flatten()
            .unwrap();

        assert_eq!(user.id, user_id);
        assert_eq!(user.name, user_name);
    }

    #[actix_rt::test]
    async fn update_works() {
        let user_id = uuid::Uuid::new_v4();
        let user_name = "User Name";
        let new_user = create_test_user(user_id, user_name.to_string(), (1977, 03, 10));
        let mut repo = MockRepository::default();

        repo.expect_update_user()
            .returning(move |user| Ok(user.to_owned()));

        let mut result = put(web::Json(new_user), web::Data::new(repo)).await;

        let user = result
            .take_body()
            .as_ref()
            .map(|b| match b {
                actix_web::dev::Body::Bytes(x) => serde_json::from_slice::<'_, User>(x).ok(),
                _ => None,
            })
            .flatten()
            .unwrap();

        assert_eq!(user.id, user_id);
        assert_eq!(user.name, user_name);
    }

    #[actix_rt::test]
    async fn delete_works() {
        let user_id = uuid::Uuid::new_v4();
        let mut repo = MockRepository::default();

        repo.expect_delete_user()
            .returning(move |id| Ok(id.to_owned()));

        let mut result = delete(web::Path::from(user_id), web::Data::new(repo)).await;
        let result = result.take_body();

        let id = result
            .as_ref()
            .map(|b| match b {
                actix_web::dev::Body::Bytes(x) => std::str::from_utf8(x).ok(),
                _ => None,
            })
            .flatten()
            .unwrap();

        assert_eq!(id, user_id.to_string());
    }
}
