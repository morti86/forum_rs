use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::{User, UserRole, Thread, Post, UserWarning, PrivateMessage};

#[async_trait]
pub trait UserExt {
    async fn get_user(&self, user_id: Option<Uuid>, name: Option<&str>, email: Option<&str>, token: Option<&str>) -> Result<Option<User>, sqlx::Error>;
    async fn get_users(&self, page: u32, limit: usize) -> Result<Vec<User>, sqlx::Error>;
    async fn recently_online(&self, since: DateTime<Utc>, page: u32, limit: usize) -> Result<Vec<User>, sqlx::Error>;
    async fn add_user(&self, name: &str, email: &str, password: &str, verification_token: &str, token_expires_at: DateTime<Utc>) -> Result<(), sqlx::Error>;
    async fn save_user(&self, name: &str, email: &str, password: &str,  
        description: Option<&str>, facebook: Option<&str>, x_id: Option<&str>) -> Result<(), sqlx::Error>;
    async fn update_user_avatar(&self, id: Uuid, avatar: Option<&str>) -> Result<(), sqlx::Error>;
    async fn get_user_count(&self) -> Result<i64, sqlx::Error>;
    async fn update_user_name(&self, user_id: Uuid, name: &str) -> Result<(), sqlx::Error>;
    async fn update_user_role(&self, user_id: Uuid, role: UserRole) -> Result<(), sqlx::Error>;
    async fn update_user_password(&self, user_id: Uuid, password: &str) -> Result<(), sqlx::Error>;
    async fn warn_user(&self, user_id: Uuid, comment: Option<&str>, warned_by: Uuid, ban: Option<i32>) -> Result<(), sqlx::Error>;
    async fn unban_user(&self, user_id: Uuid) -> Result<(), sqlx::Error>;
    async fn verifed_token(&self, token: &str) -> Result<(), sqlx::Error>;
    async fn add_verifed_token(&self, user_id: Uuid, token: &str, expires_at: DateTime<Utc>) -> Result<(), sqlx::Error>;
    async fn get_user_posts(&self, user_id: Option<Uuid>, user_name: Option<&str>) -> Result<Vec<Post>, sqlx::Error>;
    async fn get_user_threads(&self, user_id: Option<Uuid>, user_name: Option<&str>) -> Result<Vec<Thread>, sqlx::Error>;
    async fn get_user_warnings(&self, user_id: Uuid, since: Option<DateTime<Utc>>) -> Result<Vec<UserWarning>, sqlx::Error>;
    async fn send_pm(&self, user_id: Uuid, send_to: Uuid, content: &str) -> Result<(), sqlx::Error>;
    async fn get_pms(&self, user: Uuid, page: u32, limit: usize) -> Result<Vec<PrivateMessage>, sqlx::Error>;
}

#[async_trait]
impl UserExt for crate::db::DBClient {
    async fn get_user(&self, user_id: Option<Uuid>, name: Option<&str>, email: Option<&str>, token: Option<&str>) -> Result<Option<User>, sqlx::Error> {
        let mut user: Option<User> = None;

        if let Some(user_id) = user_id {
            user = sqlx::query_as!(
                User,
                r#"SELECT id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, 
                 role as "role: UserRole", description, avatar, facebook, x_id, banned_until, last_online 
                FROM forum.users WHERE id = $1"#,
                user_id
            ).fetch_optional(&self.pool).await?;
        } else if let Some(name) = name {
            user = sqlx::query_as!(
                User,
                r#"SELECT id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, 
                 role as "role: UserRole", description, avatar, facebook, x_id, banned_until, last_online 
                FROM forum.users WHERE name = $1"#,
                name
            ).fetch_optional(&self.pool).await?;
        } else if let Some(email) = email {
            user = sqlx::query_as!(
                User,
                r#"SELECT id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, 
                 role as "role: UserRole", description, avatar, facebook, x_id, banned_until, last_online 
                FROM forum.users WHERE email = $1"#,
                email
            ).fetch_optional(&self.pool).await?;
        } else if let Some(token) = token {
            user = sqlx::query_as!(
                User,
                r#"
                SELECT id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, 
                 role as "role: UserRole" , description, avatar, facebook, x_id, banned_until, last_online
                FROM forum.users 
                WHERE verification_token = $1"#,
                token
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(user)
    }

    async fn get_users(&self, page: u32, limit: usize) -> Result<Vec<User>, sqlx::Error> {
       let offset = (page - 1) as usize * limit;
       let limit = limit as i64;
       let offset = offset as i64;

       sqlx::query_as!(
           User,
           r#"
           SELECT id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, 
                  role as "role: UserRole", description, avatar, facebook, x_id, banned_until, last_online 
           FROM forum.users 
           ORDER BY id 
           LIMIT $1 
           OFFSET $2
           "#,
           limit,
           offset
       )
       .fetch_all(&self.pool)
       .await
   }

    async fn recently_online(&self, since: DateTime<Utc>, page: u32, limit: usize) -> Result<Vec<User>, sqlx::Error> {
        let offset = (page as i64 - 1) * (limit as i64);

        sqlx::query_as!(
            User,
            r#"
            SELECT id, name, email, password, verified, created_at, updated_at, verification_token, token_expires_at, 
                   role as "role: UserRole", description, avatar, facebook, x_id, banned_until, last_online 
            FROM forum.users 
            WHERE last_online > $1
            ORDER BY id 
            LIMIT $2 
            OFFSET $3
            "#,
            since,
            limit as i64,
            offset
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn save_user(&self, name: &str, email: &str, password: &str,  
        description: Option<&str>, facebook: Option<&str>, x_id: Option<&str>) -> Result<(), sqlx::Error> {
        // Example implementation (adjust SQL to your schema):
        sqlx::query!(
            r#"
            UPDATE forum.users
            SET 
                email = $2,
                password = $3,
                description = $4,
                facebook = $5,
                x_id = $6,
                updated_at = LOCALTIMESTAMP
            WHERE name = $1
            "#,
            name,
            email,
            password,
            description,
            facebook,
            x_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_user_avatar(&self, id: Uuid, avatar: Option<&str>) -> Result<(), sqlx::Error> { 
        sqlx::query_as!(
            User,
            r#"
            UPDATE forum.users
            SET 
                avatar = $2,
                updated_at = LOCALTIMESTAMP
            WHERE id = $1
            "#,
            id,
            avatar)
        .fetch_one(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_user_count(&self) -> Result<i64, sqlx::Error> {
        struct Record {
            count: Option<i64>,
        }

        let r = sqlx::query_as!(Record, r#"SELECT COUNT(*) FROM forum.users"#)
            .fetch_one(&self.pool)
            .await?;

        Ok(r.count.unwrap_or(-1))
    }
    
    async fn add_user(&self, name: &str, email: &str, password: &str, verification_token: &str, token_expires_at: DateTime<Utc>) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO forum.users 
            (name, email, password, verification_token, token_expires_at,role)
            VALUES ($1,$2,$3,$4,$5,$6)
            "#,
            name,
            email,
            password,
            verification_token,
            token_expires_at,
            UserRole::User as UserRole)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_user_name(&self, id: Uuid, name: &str) -> Result<(), sqlx::Error> { 
        sqlx::query_as!(
            User,
            r#"
            UPDATE forum.users
            SET 
                name = $2,
                updated_at = LOCALTIMESTAMP
            WHERE id = $1
            "#,
            id,
            name)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_user_role(&self, user_id: Uuid, role: UserRole) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE forum.users
            SET 
                role = $2,
                updated_at = LOCALTIMESTAMP
            WHERE id = $1
            "#,
            user_id,
            role as UserRole)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_user_password(&self, user_id: Uuid, password: &str) -> Result<(), sqlx::Error> { 
        sqlx::query_as!(
            User,
            r#"
            UPDATE forum.users
            SET 
                password = $2,
                updated_at = LOCALTIMESTAMP
            WHERE id = $1
            "#,
            user_id,
            password)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn warn_user(&self, user_id: Uuid, comment: Option<&str>, warned_by: Uuid, ban: Option<i32>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO forum.user_warning(user_id,warn_time,comment,warned_by) VALUES($1,LOCALTIMESTAMP,$2,$3)"#,
            user_id, comment, warned_by)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(days) = ban {
            let days = sqlx::postgres::types::PgInterval { months: 0, days, microseconds: 0 };
            sqlx::query!(
                r#"UPDATE forum.users SET banned_until = LOCALTIMESTAMP + $2::interval
                WHERE id = $1"#,
                user_id, days)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }
    
    async fn unban_user(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE forum.users SET banned_until = NULL
            WHERE id = $1"#,
            user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn verifed_token(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE forum.users
            SET verified = true, 
                updated_at = LOCALTIMESTAMP,
                verification_token = NULL,
                token_expires_at = NULL
            WHERE verification_token = $1
            "#,
            token
        ).execute(&self.pool)
       .await?;

        Ok(())
    }

    async fn add_verifed_token(&self, user_id: Uuid, token: &str, expires_at: DateTime<Utc>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE forum.users
            SET
                updated_at = LOCALTIMESTAMP,
                verification_token = $2,
                token_expires_at = $3
            WHERE id = $1
            "#,
            user_id, token, expires_at
        ).execute(&self.pool)
       .await?;

        Ok(())
    }

    async fn get_user_posts(&self, user_id: Option<Uuid>, user_name: Option<&str>) -> Result<Vec<Post>, sqlx::Error> {
        if let Some(id) = user_id {
            return sqlx::query_as!(
                Post,
                r#"SELECT * FROM forum.posts WHERE author = $1"#, id)
                .fetch_all(&self.pool)
                .await;
        } else {
            let name = user_name.unwrap();
            return sqlx::query_as!(
                Post,
                r#" SELECT forum.posts.id,content,author,topic,comments,forum.posts.created_at,modified_at,likes 
                    FROM forum.posts INNER JOIN forum.users ON forum.users.id = author
                    WHERE forum.users.name = $1"#, name)
                .fetch_all(&self.pool)
                .await;
        }
    }

    async fn get_user_threads(&self, user_id: Option<Uuid>, user_name: Option<&str>) -> Result<Vec<Thread>, sqlx::Error> {
      if let Some(id) = user_id {
            return sqlx::query_as!(
                Thread,
                r#"SELECT * FROM forum.threads WHERE author = $1"#, id)
                .fetch_all(&self.pool)
                .await;
        } else {
            let name = user_name.unwrap();
            return sqlx::query_as!(
                Thread,
                r#" SELECT forum.threads.id, title, forum.threads.created_at, content, author, section_id, locked, sticky
                    FROM forum.threads INNER JOIN forum.users ON forum.users.id = author
                    WHERE forum.users.name = $1"#, name)
                .fetch_all(&self.pool)
                .await;
        }
    }

    async fn get_user_warnings(&self, user_id: Uuid, since: Option<DateTime<Utc>>) -> Result<Vec<UserWarning>, sqlx::Error> {
        let since = since.unwrap_or(DateTime::<Utc>::MIN_UTC);
        return sqlx::query_as!(UserWarning,
            r#" SELECT forum.user_warning.id,user_id as user,warn_time,comment,users.name as warned_by,banned
                FROM forum.user_warning INNER JOIN forum.users ON forum.user_warning.warned_by = forum.users.id
                WHERE forum.user_warning.user_id = $1 AND warn_time > $2"#, user_id, since)
            .fetch_all(&self.pool)
            .await;
    }

    async fn send_pm(&self, user_id: Uuid, send_to: Uuid, content: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO forum.private_messages (author, receiver, content)
            VALUES($1, $2, $3)"#, user_id, send_to, content)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_pms(&self, user: Uuid, page: u32, limit: usize) -> Result<Vec<PrivateMessage>, sqlx::Error> {
       let offset = (page - 1) as usize * limit;
       let limit = limit as i64;
       let offset = offset as i64;

        return sqlx::query_as!(PrivateMessage,
            r#" SELECT id,author,receiver,content FROM forum.private_messages WHERE receiver = $1
                LIMIT $2 OFFSET $3"#,
            user, limit, offset)
            .fetch_all(&self.pool)
            .await;
    }
}
