pub mod title;
pub mod course;

pub enum Scenes {
    TITLE(title::Title),
    COURSE1,
    COURSE2,
    COURSE3,
    COURSE4,
    COURSE5,
    COURSE6,
}

pub struct Scene {
    content: Scenes,
    next_scene: Scenes
}
