use crate::{AppError, User};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::PgPool;
use std::mem;

impl User {
    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user =
            sqlx::query_as("SELECT id, fullname, email, created_at FROM users WHERE email = $1")
                .bind(email)
                .fetch_optional(pool)
                .await?;
        Ok(user)
    }

    pub async fn create(
        email: &str,
        fullname: &str,
        password: &str,
        pool: &PgPool,
    ) -> Result<Self, AppError> {
        let password_hash = hash_password(password)?;
        let user = sqlx::query_as(
            r#"
            INSERT INTO users(email, fullname, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, fullname, email, password_hash, created_at
            "#,
        )
        .bind(email)
        .bind(fullname)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn verify(
        email: &str,
        password: &str,
        pool: &PgPool,
    ) -> Result<Option<Self>, AppError> {
        let user: Option<User> = sqlx::query_as("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await?;

        match user {
            Some(mut user) => {
                let password_hash = mem::take(&mut user.password_hash);
                let is_valid = verify_password(password, &password_hash.unwrap_or_default())?;
                if is_valid {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

// check the password whether same
fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(password_hash)?;

    let is_valid = argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use sqlx_db_tester::TestPg;
    use std::path::Path;

    #[test]
    fn it_work() {
        println!("yes it work");
    }

    #[test]
    fn password_hash_it_work() -> Result<()> {
        println!("password-hash test====");
        let password = "hhahha0";
        let password_hash = hash_password(password)?;
        println!("pwd_hash: {:?}", &password_hash);
        assert_eq!(password_hash.len(), 97);
        assert!(verify_password(password, &password_hash)?);
        Ok(())
    }

    #[tokio::test]
    async fn create_and_veiry_user_should_work() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://postgres:123456@localhost:5432".to_string(),
            Path::new("../migrations"),
        );

        let db_pool = tdb.get_pool().await;
        let email = "testmumm@test.org";
        let name = "Martin Z";
        let password = "hhahha0";
        let user = User::create(email, name, password, &db_pool).await?;
        assert_eq!(user.email, email);
        assert_eq!(user.fullname, name);
        assert!(user.id > 0);
        println!("user-created: {:?}", user);
        println!("test db test");

        let user = User::find_by_email(email, &db_pool).await?;
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.email, email);
        assert_eq!(user.fullname, name);

        let user = User::verify(email, password, &db_pool).await?;
        assert!(user.is_some());

        Ok(())
    }
}
