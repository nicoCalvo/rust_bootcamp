use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Question {
    title: String,
    description: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    question_uuid: String,
    title: String,
    description: String
    created_at: String
}

#[derive(Serialize, Deserialize)]
pub struct QuestionId{
    question_uuid: String
}

#[derive(Serialize, Deserialize)]
pub struct Answer{
    question_uuid: String,
    content: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerDetail{
    answer_uuid: String,
    question_uuid: String,
    content: String,
    created_at: String
}

#[derive(Serialize, Deserialize)]
pub struct AnswerId{
    answer_uuid: String
}

