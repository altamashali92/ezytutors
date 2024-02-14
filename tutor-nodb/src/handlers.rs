use super::state::AppState;
use actix_web::{web, HttpResponse};
use super::models::Course;
use chrono::Utc;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>
) -> HttpResponse {
    println!("Received new course");
    let course_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .count();
    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some(course_count_for_user + 1),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Successfully added course")
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>,
) -> HttpResponse {
    let tutor_id = params.0;
    let filtered_courses: Vec<Course> = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == tutor_id)
        .collect();
    if filtered_courses.is_empty() {
        HttpResponse::NotFound().json("No courses found for tutor")
    } else {
        HttpResponse::Ok().json(filtered_courses)
    }
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let tutor_id = params.0;
    let course_id = params.1;
    let course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|course| course.tutor_id == tutor_id && course.course_id == Some(course_id));
    match course {
        Some(course) => HttpResponse::Ok().json(course),
        None => HttpResponse::NotFound().json("Course not found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, web::Path};
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
                tutor_id: 1,
                course_name: "test course".into(),
                course_id: None,
                posted_time: None,
        });
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![
                Course {
                    tutor_id: 1,
                    course_name: "test course".into(),
                    course_id: Some(1),
                    posted_time: Some(Utc::now().naive_utc()),
                },
                Course {
                    tutor_id: 1,
                    course_name: "test course 2".into(),
                    course_id: Some(2),
                    posted_time: Some(Utc::now().naive_utc()),
                },
            ]),
        });
        let tutor_id: web::Path<(usize,)> = web::Path::from((1,));
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_failure() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let tutor_id: web::Path<(usize,)> = web::Path::from((1,));
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
    #[actix_rt::test]
    async fn get_one_course_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {   
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let params: web::Path<(usize, usize)> = web::Path::from((1, 1));     
        let resp = get_course_detail(app_state, params).await;          
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);                       
    }

}