use async_trait::async_trait;
use uuid::Uuid;

use crate::models::{Section, Thread, ChatPost, Post, UserRole};

#[async_trait]
pub trait ForumExt {
    async fn create_thread(&self, user: Uuid, section: i64, title: &str, content: &str, hash_tags: &Vec<String>) -> Result<(), sqlx::Error>;
    async fn delete_thread(&self, thread_id: i64) -> Result<(), sqlx::Error>;
    async fn update_thread(&self, thread_id: i64, title: &str, content: &str) -> Result<(), sqlx::Error>;
    async fn lock_thread(&self, thread_id: i64, locked: bool) -> Result<(), sqlx::Error>;

    async fn create_section(&self, name: &str, description: &str, allowed_for: &[UserRole]) -> Result<(), sqlx::Error>;
    async fn get_sections(&self, user: Uuid) -> Result<Vec<Section>, sqlx::Error>;
    async fn delete_section(&self, s_id: i64) -> Result<(), sqlx::Error>;

    async fn get_chat(&self, limit: usize) -> Result<Vec<ChatPost>, sqlx::Error>;
    async fn post_chat(&self, u_id: Uuid, content: &str) -> Result<(), sqlx::Error>;
    async fn delete_chat(&self, post_id: i32) -> Result<(), sqlx::Error>;

    async fn get_section(&self, s_id: i64, page: i32, limit: usize) -> Result<Vec<Thread>, sqlx::Error>;
    async fn get_thread(&self, t_id: i64, page: i32, limit: usize) -> Result<Vec<Post>, sqlx::Error>;
    async fn get_thread_info(&self, t_id: i64) -> Result<Thread, sqlx::Error>;
    async fn get_thread_author(&self, t_id: i64) -> Result<Uuid, sqlx::Error>;
    async fn get_thread_reply_count(&self, t_id: i64) -> Result<i64, sqlx::Error>;

    async fn update_post(&self, p_id: i64, content: &str) -> Result<(), sqlx::Error>;
    async fn delete_post(&self, post_id: i64) -> Result<(), sqlx::Error>;
    async fn get_post_author(&self, t_id: i64) -> Result<Option<Uuid>, sqlx::Error>;
    async fn posts_since(&self, post_id: i64) -> Result<i64, sqlx::Error>;
}

#[async_trait]
impl ForumExt for crate::db::DBClient {
    async fn create_thread(&self, user: Uuid, section: i64, title: &str, content: &str, hash_tags: &Vec<String>) -> Result<(), sqlx::Error> {
        struct ParsingHelper {
            id: i64,
        }

        let r = sqlx::query_as!(ParsingHelper, r#"INSERT INTO forum.threads(title,created_at,content,author,section_id,locked)
            VALUES ($1,LOCALTIMESTAMP,$2,$3,$4,false)
            RETURNING id"#,
            title, content, user, section)
            .fetch_one(&self.pool)
            .await?;
            let _ = hash_tags.iter().map(async |t| {
                sqlx::query!(r#"INSERT INTO forum.hashtags(tag, topic) VALUES($1, $2)"#, t, r.id)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| e)
            });
        Ok(())
    }

    async fn delete_thread(&self, thread_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM forum.threads WHERE id = $1"#, thread_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn update_thread(&self, thread_id: i64, title: &str, content: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"UPDATE forum.threads
            SET
                title = $2,
                content = $3
            WHERE id = $1"#, thread_id, title, content)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn lock_thread(&self, thread_id: i64, locked: bool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE forum.threads
            SET locked = $2
            WHERE id = $1
            "#,
            thread_id, locked)
            .fetch_one(&self.pool)
            .await?;
        Ok(())
    }

    async fn create_section(&self, name: &str, description: &str, allowed_for: &[UserRole]) -> Result<(), sqlx::Error> {
        struct Helper {
            id: i64,
        }

        let r = sqlx::query_as!(Helper,
            r#" INSERT INTO forum.sections
                    (name, description)
                VALUES($1, $2)
                RETURNING id"#, name, description)
            .fetch_one(&self.pool)
            .await?;

        let _ = allowed_for.iter().map(async |rl| {
            sqlx::query!(r#"INSERT INTO forum.sections_allowed
                                (section_id, role)
                            VALUES($1, $2)"#, r.id, *rl as UserRole)
                .execute(&self.pool)
                .await
                .map_err(|e| e)
        });

        Ok(())
    }

    async fn get_sections(&self, user: Uuid) -> Result<Vec<Section>, sqlx::Error> {
         sqlx::query_as!(Section,
            r#"SELECT s.id, s.name, s.description FROM forum.sections_allowed sa
               JOIN(SELECT role FROM forum.users WHERE id = $1) ur ON sa.role = ur.role
               JOIN forum.sections s ON section_id = sa.section_id"#, user)
            .fetch_all(&self.pool)
            .await
    }

    async fn delete_section(&self, s_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM forum.sections
               WHERE id = $1"#, s_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_chat(&self, limit: usize) -> Result<Vec<ChatPost>, sqlx::Error> {
        let limit = limit as i64;
        sqlx::query_as!(ChatPost,
            r#" SELECT p.id,added,author,u.name as author_name,content FROM forum.chat_posts p
                INNER JOIN forum.users u ON author = u.id
                ORDER BY added DESC
                LIMIT $1"#, limit)
            .fetch_all(&self.pool)
            .await
    }

    async fn post_chat(&self, u_id: Uuid, content: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#" INSERT INTO forum.chat_posts(added, author, content)
                VALUES (LOCALTIMESTAMP, $1, $2)"#, u_id, content)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_chat(&self, post_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"DELETE FROM forum.chat_posts WHERE id = $1"#, post_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_section(&self, s_id: i64, page: i32, limit: usize) -> Result<Vec<Thread>, sqlx::Error> {
        let offset = (page - 1) as usize * limit;
        let limit = limit as i64;
        let offset = offset as i64;

        sqlx::query_as!(Thread,
            r#" SELECT * FROM forum.threads WHERE section_id = $1
                LIMIT $2 OFFSET $3"#, s_id, limit, offset)
            .fetch_all(&self.pool)
            .await
    }

    async fn get_thread(&self, t_id: i64, page: i32, limit: usize) -> Result<Vec<Post>, sqlx::Error> {
        let offset = (page - 1) as usize * limit;
        let limit = limit as i64;
        let offset = offset as i64;
        sqlx::query_as!(Post,
            r#" SELECT * FROM forum.posts WHERE topic = $1
                LIMIT $2 OFFSET $3"#, t_id, limit, offset)
            .fetch_all(&self.pool)
            .await
    }

    async fn get_thread_info(&self, t_id: i64) -> Result<Thread, sqlx::Error> {
        sqlx::query_as!(Thread,
            r#" SELECT * FROM forum.threads WHERE id = $1"#, t_id)
            .fetch_one(&self.pool)
            .await
    }

    async fn get_thread_author(&self, t_id: i64) -> Result<Uuid, sqlx::Error> {
        struct Helper {
            author: Uuid,
        }

        let res = sqlx::query_as!(Helper,
            r#"SELECT author FROM forum.threads WHERE id = $1"#, t_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(res.author)
    }

    async fn get_post_author(&self, t_id: i64) -> Result<Option<Uuid>, sqlx::Error> {
        struct Helper {
            author: Option<Uuid>,
        }

        let res = sqlx::query_as!(Helper,
            r#"SELECT author FROM forum.posts WHERE id = $1"#, t_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(res.author)
    }


    async fn get_thread_reply_count(&self, t_id: i64) -> Result<i64, sqlx::Error> {
        struct Helper {
            cnt: Option<i64>,
        }

        let res = sqlx::query_as!(Helper,
            r#"SELECT COUNT(*) cnt FROM forum.threads WHERE id = $1"#, t_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(res.cnt.unwrap_or(-1))
    }

    async fn update_post(&self, p_id: i64, content: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#" UPDATE forum.posts
                SET content = $1
                WHERE id = $2"#, content, p_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_post(&self, post_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#" DELETE FROM forum.posts
                WHERE id = $1"#, post_id)
            .execute(&self.pool)
            .await?;

        Ok(())

    }

    async fn posts_since(&self, post_id: i64) -> Result<i64, sqlx::Error> {
        struct Helper {
            count: Option<i64>,
        }

        let res = sqlx::query_as!(Helper,
            r#" SELECT COUNT(*) 
                FROM forum.posts 
                WHERE created_at > (
                    SELECT created_at 
                    FROM forum.posts 
                    WHERE id = $1)"#, post_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(res.count.unwrap_or(-1))
    }
}
