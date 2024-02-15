use dotenv::dotenv;
use std::env;
use std::io;
use sqlx::postgres::PgPool;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect(
        "DATABASE_URL is not set in .env file"
    );
    let db_pool = PgPool::connect(&db_url).await.unwrap();
    let course_rows = sqlx::query!(
        r#"SELECT * FROM ezy_course_c4 WHERE course_id = $1"#,
        1
    )
        .fetch_all(&db_pool)
        .await
        .unwrap();
    let mut course_list = vec![];
    for course_row in course_rows {
        course_list.push(
            Course{
                course_id: course_row.course_id,
                tutor_id: course_row.tutor_id,
                course_name: course_row.course_name,
                posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
            }
        );
    }
    println!("Courses = {:?}", course_list);
    Ok(())
}