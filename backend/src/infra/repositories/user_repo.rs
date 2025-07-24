// 用户仓库层 - 数据访问接口
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    error::AppResult,
    models::user::{self, Entity as User},
};

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> AppResult<Option<user::Model>> {
        let user = User::find_by_id(id).one(db).await?;
        Ok(user)
    }

    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> AppResult<Option<user::Model>> {
        let user = User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await?;
        Ok(user)
    }

    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> AppResult<Option<user::Model>> {
        let user = User::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await?;
        Ok(user)
    }
}
