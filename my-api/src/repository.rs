use std::sync::{PoisonError, RwLock};

use crate::user::User;
use async_trait::async_trait;
use chrono::Utc;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("PoisonError: `{0}`")]
    LockError(String),
    #[error("This entity already exists")]
    AlreadyExists,
    #[error("This does not entity exists")]
    DoesNotExist,
    #[error("This id format is not valid")]
    InvalidId,
}

impl<T> From<PoisonError<T>> for RepositoryError {
    fn from(poison_error: PoisonError<T>) -> Self {
        RepositoryError::LockError(poison_error.to_string())
    }
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Repository: Send + Sync + 'static {
    async fn get_user(&self, user_id: &Uuid) -> RepositoryResult<User>;
    async fn create_user(&self, user: &User) -> RepositoryResult<User>;
    async fn update_user(&self, user: &User) -> RepositoryResult<User>;
    async fn delete_user(&self, user_id: &Uuid) -> RepositoryResult<Uuid>;
}

pub struct PostgresRepository {
    pool: sqlx::PgPool,
}

impl PostgresRepository {
    pub async fn from_env() -> sqlx::Result<Self> {
        let conn_str =
            std::env::var("DATABASE_URL").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
        let pool = sqlx::PgPool::connect(&conn_str).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl Repository for PostgresRepository {
    #[instrument(skip(self))]
    async fn get_user(&self, user_id: &uuid::Uuid) -> RepositoryResult<User> {
        let result = sqlx::query_as::<_, User>(
            "SELECT id, name, birth_date, custom_data, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await;

        result.map_err(|e| {
            tracing::error!("{:?}", e);
            RepositoryError::InvalidId
        })
    }

    #[instrument(skip(self))]
    async fn create_user(&self, user: &User) -> RepositoryResult<User> {
        let result = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, name, birth_date, custom_data)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, birth_date, created_at, updated_at
            "#,
        )
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.birth_date)
        .bind(&user.custom_data)
        .fetch_one(&self.pool)
        .await;

        result.map_err(|e| {
            tracing::error!("{:?}", e);
            RepositoryError::AlreadyExists
        })
    }

    #[instrument(skip(self))]
    async fn update_user(&self, user: &User) -> RepositoryResult<User> {
        let result = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET custom_data = $1, updated_at = $2
            WHERE id = $3
            RETURNING id, name, birth_date, created_at, updated_at
            "#,
        )
        .bind(&user.custom_data)
        .bind(Utc::now())
        .bind(&user.id)
        .fetch_one(&self.pool)
        .await;

        result.map_err(|e| {
            tracing::error!("{:?}", e);
            RepositoryError::DoesNotExist
        })
    }

    #[instrument(skip(self))]
    async fn delete_user(&self, user_id: &uuid::Uuid) -> RepositoryResult<Uuid> {
        let result = sqlx::query_as::<_, User>(
            r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING id, name, birth_date, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await;

        result.map(|e| e.id).map_err(|e| {
            tracing::error!("{:?}", e);
            RepositoryError::DoesNotExist
        })
    }
}

pub struct MemoryRepository {
    users: RwLock<Vec<User>>,
}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self {
            users: RwLock::new(vec![]),
        }
    }
}

#[async_trait]
impl Repository for MemoryRepository {
    #[instrument(skip(self))]
    async fn get_user(&self, user_id: &uuid::Uuid) -> RepositoryResult<User> {
        let users = self.users.read()?;
        users
            .iter()
            .find(|u| &u.id == user_id)
            .cloned()
            .ok_or_else(|| RepositoryError::InvalidId)
    }

    #[instrument(skip(self))]
    async fn create_user(&self, user: &User) -> RepositoryResult<User> {
        if self.get_user(&user.id).await.is_ok() {
            return Err(RepositoryError::AlreadyExists);
        }
        let mut new_user = user.to_owned();
        new_user.created_at = Some(Utc::now());
        let mut users = self.users.write()?;
        users.push(new_user.clone());
        Ok(new_user)
    }

    #[instrument(skip(self))]
    async fn update_user(&self, user: &User) -> RepositoryResult<User> {
        if let Ok(old_user) = self.get_user(&user.id).await {
            let mut updated_user = user.to_owned();
            updated_user.created_at = old_user.created_at;
            updated_user.updated_at = Some(Utc::now());
            let mut users = self.users.write()?;
            users.retain(|x| x.id != user.id);
            users.push(updated_user.clone());
            Ok(updated_user)
        } else {
            return Err(RepositoryError::DoesNotExist);
        }
    }

    #[instrument(skip(self))]
    async fn delete_user(&self, user_id: &uuid::Uuid) -> RepositoryResult<Uuid> {
        if self.get_user(&user_id).await.is_err() {
            return Err(RepositoryError::DoesNotExist);
        }
        let mut users = self.users.write()?;
        users.retain(|x| &x.id != user_id);
        Ok(user_id.to_owned())
    }
}
