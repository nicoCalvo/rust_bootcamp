use async_trait::async_trait;
use rocket::futures::TryFutureExt;
use sqlx::{types::Uuid, PgPool};

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self{db}
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let mut pool = self.db.acquire()
        .map_err(|e| DBError::Other(Box::new(e))).await?;
        let record = sqlx::query!(
            "INSERT INTO questions(title, description)
            VALUES ($1, $2)
            RETURNING question_uuid, created_at",
            question.title,
            question.description )
            .fetch_one(&mut pool)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))?;
        Ok(QuestionDetail {
            question_uuid: record.question_uuid.to_string(),
            title: question.title,
            description: question.description,
            created_at: record.created_at.to_string()
            })
}
       

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {

        let uuid = Uuid::parse_str(&question_uuid).map_err(|e| DBError::InvalidUUID(question_uuid))?;

        let mut pool = self.db.acquire()
            .map_err(|e| DBError::Other(Box::new(e))).await?;
    
        sqlx::query!("DELETE FROM questions WHERE question_uuid = $1", uuid)
            .execute(&mut pool)
            .map_err(|e| DBError::Other(Box::new(e)))
            .await?;

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let mut pool = self.db.acquire()
        .map_err(|e| DBError::Other(Box::new(e))).await?;

        let records = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&mut pool)
        .map_err(|e| DBError::Other(Box::new(e)))
        .await?;

        let questions: Vec<QuestionDetail> = records.iter()
        .map(|r| {
            QuestionDetail{
                question_uuid: r.question_uuid.to_string(),
                title: r.title.clone(),
                description: r.description.clone(),
                created_at: r.created_at.to_string()
            }
        }).collect();

        Ok(questions)
    }
}
