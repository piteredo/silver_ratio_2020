pub mod title;
pub mod course;

pub enum Content {
    TITLE(title::Title),
    COURSE1(course::Course),
    COURSE2(course::Course),
    COURSE3(course::Course),
    COURSE4(course::Course),
    COURSE5(course::Course),
    COURSE6(course::Course),
}

pub struct Scene {
    content: Content,
    next_scene: Content
}
