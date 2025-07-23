use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::JwtConfig,
    error::{AppError, AppResult},
    models::{
        dtos::{LoginRequest, RegisterRequest},
        user::{self, Entity as User},
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // 用户ID
    pub email: String,
    pub exp: usize, // 过期时间
    pub iat: usize, // 签发时间
}

/// 用户注册
pub async fn register(
    db: &DatabaseConnection,
    req: RegisterRequest,
) -> AppResult<user::Model> {
    // 检查邮箱是否已存在
    let existing_user = User::find()
        .filter(user::Column::Email.eq(&req.email))
        .one(db)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::Conflict("Email already exists".to_string()));
    }

    // 加密密码
    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)?;

    // 创建新用户
    let new_user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(req.username),
        email: Set(req.email),
        password_hash: Set(password_hash),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };

    let user = new_user.insert(db).await?;
    Ok(user)
}

/// 用户登录
pub async fn login(
    db: &DatabaseConnection,
    req: LoginRequest,
) -> AppResult<user::Model> {
    // 查找用户
    let user = User::find()
        .filter(user::Column::Email.eq(&req.email))
        .one(db)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // 验证密码
    let is_valid = bcrypt::verify(&req.password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::Unauthorized);
    }

    Ok(user)
}

/// 生成JWT令牌
pub fn generate_token(user: &user::Model, config: &JwtConfig) -> AppResult<String> {
    let now = Utc::now();
    let exp = now + Duration::seconds(config.expires_in);

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_ref()),
    )?;

    Ok(token)
}

/// 验证JWT令牌
pub fn verify_token(token: &str, config: &JwtConfig) -> AppResult<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

/// 根据用户ID获取用户
pub async fn get_user_by_id(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> AppResult<user::Model> {
    let user = User::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(user)
}
