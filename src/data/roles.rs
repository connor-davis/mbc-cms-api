use anyhow::Error;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct RoleManager {
    pub database_pool: Pool<Postgres>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RolePermission {
    pub id: Uuid,
    pub role_id: Uuid,
    pub permission_name: String,
    pub permission_level: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl RoleManager {
    pub async fn init() -> Self {
        let config: Config = Config::init();

        let database_pool = match PgPoolOptions::new()
            .max_connections(32)
            .connect(&config.database_url)
            .await
        {
            Ok(database_pool) => {
                tracing::info!("✅ RoleManager connected to database.");
                database_pool
            }
            Err(error) => {
                tracing::error!("🔥 RoleManager failed to connect to database: {}", error);
                std::process::exit(1);
            }
        };

        Self { database_pool }
    }

    pub async fn create_role_with_permissions(
        &self,
        name: String,
        permissions: Vec<(String, i64)>,
    ) -> Result<(), Error> {
        let inserted_role = sqlx::query!(
            r#"
                INSERT INTO roles (name)
                VALUES ($1)
                RETURNING id;
            "#,
            name
        )
        .fetch_one(&self.database_pool)
        .await
        .map_err(|error| {
            tracing::error!("🔥 Failed to insert new role: {}", error);
            error
        })?;

        for permission in permissions {
            sqlx::query!(
                r#"
                    INSERT INTO roles_permissions (role_id, permission_name, permission_level)
                    VALUES ($1, $2, $3)
                "#,
                inserted_role.id,
                permission.0,
                permission.1
            )
            .execute(&self.database_pool)
            .await
            .map_err(|error| {
                tracing::error!("🔥 Failed to insert new role permission: {}", error);
                error
            })?;
        }

        Ok(())
    }

    pub async fn create_role_permissions(
        &self,
        role_id: Uuid,
        permissions: Vec<(String, i64)>,
    ) -> Result<(), Error> {
        for permission in permissions {
            sqlx::query!(
                r#"
                    INSERT INTO roles_permissions (role_id, permission_name, permission_level)
                    VALUES ($1, $2, $3)
                "#,
                role_id,
                permission.0,
                permission.1
            )
            .execute(&self.database_pool)
            .await
            .map_err(|error| {
                tracing::error!("🔥 Failed to insert new role permisson: {}", error);
                error
            })?;
        }

        Ok(())
    }

    pub async fn remove_role_permissions(
        &self,
        role_id: Uuid,
        permissions: Vec<String>,
    ) -> Result<(), Error> {
        for permission_name in permissions {
            sqlx::query!(
                r#"
                    DELETE FROM roles_permissions
                    WHERE role_id = $1 AND permission_name = $2
                "#,
                role_id,
                permission_name,
            )
            .execute(&self.database_pool)
            .await
            .map_err(|error| {
                tracing::error!("🔥 Failed to delete role permission: {}", error);
                error
            })?;
        }

        Ok(())
    }

    pub async fn get_role(&self, role_id: Uuid) -> Result<Option<Role>, Error> {
        let role = sqlx::query_as!(
            Role,
            r#"
                SELECT * FROM roles
                WHERE id = $1
            "#,
            role_id
        )
        .fetch_optional(&self.database_pool)
        .await
        .map_err(|error| {
            tracing::error!("🔥 Failed to get role: {}", error);
            error
        })?;

        Ok(role)
    }

    pub async fn get_role_permissions(&self, role_id: Uuid) -> Result<Vec<RolePermission>, Error> {
        let role_permissions = sqlx::query_as!(
            RolePermission,
            r#"
                SELECT * FROM roles_permissions
                WHERE role_id = $1
            "#,
            role_id
        )
        .fetch_all(&self.database_pool)
        .await
        .map_err(|error| {
            tracing::error!("🔥 Failed to get role permissions: {}", error);
            error
        })?;

        Ok(role_permissions)
    }

    pub async fn has_permissions(
        &self,
        role_id: Uuid,
        permission_name: String,
        permission_level: i64,
    ) -> Result<bool, Error> {
        let permissions = &self.get_role_permissions(role_id).await?;
        let permission = permissions
            .into_iter()
            .find(|permission| permission.permission_name == permission_name);

        match permission {
            Some(permission) => Ok(permission.permission_level == permission_level),
            None => Ok(false),
        }
    }
}
