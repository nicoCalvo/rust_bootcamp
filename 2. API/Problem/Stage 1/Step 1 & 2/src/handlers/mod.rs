use std::{ops::Deref, sync::Arc};

use chrono::Utc;
use rocket::{serde::json::Json};
use uuid::Uuid;

use crate::models::*;

// ---- CRUD for Questions ----

static mut QUESTIONS: Vec<QuestionDetail> =Vec::new();
static mut ANSWERS: Vec<AnswerDetail> =Vec::new();

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
) -> Json<QuestionDetail> {
    let now = Utc::now();
    let question_detail = QuestionDetail{
        question_uuid: Uuid::new_v4().to_string(),
        title: question.title.clone(),
        description: question.description.clone(),
        created_at: now.to_string()
    };
    unsafe {QUESTIONS.push(question_detail.clone())}
    
    Json(question_detail)

}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    let q = unsafe {
        QUESTIONS.clone()
    };
    Json(q)
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>
) {
    unsafe{
    QUESTIONS.retain_mut(|q| q.question_uuid == question_uuid.question_uuid)
    }
}

// ---- CRUD for Answers ----


#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
) -> Json<AnswerDetail> {
    let now = Utc::now();
    let answer_detail = AnswerDetail{
        answer_uuid: Uuid::new_v4().to_string(),
        question_uuid: answer.question_uuid.clone(),
        content: answer.content.clone(),
        created_at: now.to_string()
    };
    unsafe {ANSWERS.push(answer_detail.clone())}
    Json(answer_detail)

}

#[get("/answers")]
pub async fn read_answers() -> Json<Vec<AnswerDetail>> {
    let a = unsafe {
        ANSWERS.clone()
    };
    Json(a)
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>
) {
    unsafe{
        ANSWERS.retain_mut(|q| q.question_uuid == answer_uuid.answer_uuid)
    }
}


#[cfg(test)]
mod test {
    use rocket::http::ContentType;
    use rocket::local::blocking::Client;
    use rocket::serde::json::json;
    use rocket::Build;
    use super::*;

    fn rocket() -> rocket::Rocket<Build> {
        rocket::build()
            .mount("/", routes![
                read_questions,
                create_question,
                delete_question,
                read_answers,
                create_answer,
                delete_answer
                ])

    }

    #[test]
    fn test_questions() {
        // empty questions
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/questions").dispatch();
        let status = response.status();
        assert_eq!(status.code, 200);
        let content = response.into_json::<Vec<QuestionDetail>>().unwrap();
        assert!(content.is_empty());

        // post question
        let response = client
            .post("/question")
            .header(ContentType::JSON)
            .body(
                r#"
                {   
                    "title": "some_title",
                    "description": "some description"
                }
                "#, )
            .dispatch();
        let status = response.status();
        assert_eq!(status.code, 200);
        let detail = response.into_json::<QuestionDetail>().unwrap();
        assert_eq!(detail.title, "some_title".to_owned());
        assert_eq!(detail.title, "some_title".to_owned());



        // delete question
        let q_id = json!({"question_uuid": detail.question_uuid.to_string()});
        
        let response = client
        .delete("/question")
        .header(ContentType::JSON)
        .body(q_id.to_string())
        .dispatch();
        let status = response.status();
        assert_eq!(status.code, 200);

    }

    #[test]
    fn test_answers() {
        // empty answers
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/answers").dispatch();
        let status = response.status();
        assert_eq!(status.code, 200);
        let content = response.into_json::<Vec<AnswerDetail>>().unwrap();
        assert!(content.is_empty());

        // post question
        let response = client
            .post("/answer")
            .header(ContentType::JSON)
            .body(
                r#"
                {   
                    "question_uuid": "some_uuid",
                    "content": "some answer"
                }
                "#, )
            .dispatch();
        let status = response.status();
        assert_eq!(status.code, 200);
        let detail = response.into_json::<AnswerDetail>().unwrap();
        assert_eq!(detail.question_uuid, "some_uuid".to_owned());
        assert_eq!(detail.content, "some answer".to_owned());



        // delete question
        let answer_id = json!({"answer_uuid": detail.answer_uuid.to_string()});
        
        let response = client
        .delete("/answer")
        .header(ContentType::JSON)
        .body(answer_id.to_string())
        .dispatch();
        let status = response.status();
        assert_eq!(status.code, 200);

    }
}