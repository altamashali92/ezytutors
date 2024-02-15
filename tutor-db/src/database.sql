drop table if exists ezy_course_c4;

create table ezy_course_c4 (
    course_id serial primary key,
    tutor_id int not null,
    course_name varchar(140) not null,
    posted_time timestamp default now()
);

INSERT INTO ezy_course_c4
    (course_id, tutor_id, course_name, posted_time)
VALUES (1,1,'First Course', '2024-02-15 08:29:43');

INSERT INTO ezy_course_c4
    (course_id, tutor_id, course_name, posted_time)
VALUES (2,1,'second Course', '2024-02-15 08:29:43');