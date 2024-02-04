use async_trait::async_trait;
use rocket::futures::TryFutureExt;
use sqlx::{types::Uuid, PgPool};

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self{db}
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        // Use the `sqlx::types::Uuid::parse_str` method to parse the `question_uuid` field
        // in `Answer` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid = Uuid::parse_str(&answer.question_uuid)
        .map_err(|e| DBError::InvalidUUID(answer.question_uuid))?;
        let mut pool = self.db.acquire().map_err(|e| DBError::Other(Box::new(e))).await?;
        let record = sqlx::query!(
            "INSERT INTO answers(question_uuid, content)
            VALUES ($1, $2)
            RETURNING answer_uuid, created_at",
            uuid,
            answer.content)
            .fetch_one(&mut pool)
            .await
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;
        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: uuid.to_string(),
            content: answer.content,
            created_at: record.created_at.to_string()
            })
}


    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
         let uuid = Uuid::parse_str(&answer_uuid)
        .map_err(|e| DBError::InvalidUUID(answer_uuid))?;

        let mut pool = self.db.acquire()
        .map_err(|e| DBError::Other(Box::new(e))).await?;

        sqlx::query!("DELETE FROM answers WHERE answer_uuid = $1", uuid)
            .execute(&mut pool)
            .map_err(|e| DBError::Other(Box::new(e)))
            .await?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let mut pool = self.db.acquire()
        .map_err(|e| DBError::Other(Box::new(e))).await?;

        let uuid = Uuid::parse_str(&question_uuid)
        .map_err(|e| DBError::InvalidUUID(question_uuid))?;

        let records = sqlx::query!(
            "SELECT * FROM answers WHERE question_uuid = $1",
            uuid)
        .fetch_all(&mut pool)
        .map_err(|e| DBError::Other(Box::new(e)))
        .await?;

        let answers: Vec<AnswerDetail> = records.iter()
        .map(|r| {
            AnswerDetail{
                answer_uuid: r.answer_uuid.to_string(),
                question_uuid: r.question_uuid.to_string(),
                content: r.content.clone(),
                created_at: r.created_at.to_string()
            }
        }).collect();

        Ok(answers)
    }
}
