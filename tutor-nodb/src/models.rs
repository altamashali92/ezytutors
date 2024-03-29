use actix_web::web;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Course {
    pub tutor_id: usize,
    pub course_id: Option<usize>,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

// convert data from incoming request to Course struct
impl From <web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}