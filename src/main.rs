mod data;
mod scene;
use nannou::prelude::*;

const BPM: f32 = 360.0; // beat => 16th beat
const NOTE_INTERVAL: f32 = 20.0;

enum BallDirection {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

struct Map {
    w: f32,
    h: f32,
}

struct Ball {
    radius: f32,
    pos: Vec<f32>,
    dir: BallDirection,
}

struct Course {
    map: Map,
    ball: Ball,
    turn_len: u32,
    turn_cnt: u32
}

struct Model {
    course: Course,
}

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1024, 764)
        .build()
        .unwrap();

    let map_1 = Map {
        w: NOTE_INTERVAL * 24.0,
        h: NOTE_INTERVAL * 12.0,
    };
    let ball_1 = Ball {
        radius: 5.0,
        pos: vec![
            -map_1.w/2.0,
            map_1.h/2.0
        ],
        dir: BallDirection::RIGHT,
    };
    let course_1 = Course {
        map: map_1,
        ball: ball_1,
        turn_len: 5,
        turn_cnt: 0,
    };

    Model {
        course: course_1,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let millis_per_frame = app.duration.since_prev_update.as_millis() as f32;
    let speed = NOTE_INTERVAL * BPM * millis_per_frame / 60_000.0;
    match model.course.ball.dir {
        BallDirection::RIGHT => {
            model.course.ball.pos[0] += speed as f32;
            if model.course.ball.pos[0] >= model.course.map.w/2.0 {
                model.course.ball.pos[0] = model.course.map.w/2.0;
                model.course.ball.dir = BallDirection::DOWN;
            }
        },
        BallDirection::DOWN => {
            model.course.ball.pos[1] -= speed as f32;
            if model.course.ball.pos[1] <= -model.course.map.h/2.0 {
                model.course.ball.pos[1] = -model.course.map.h/2.0;
                model.course.ball.dir = BallDirection::LEFT;
            }
        },
        BallDirection::LEFT => {
            model.course.ball.pos[0] -= speed as f32;
            if model.course.ball.pos[0] <= -model.course.map.w/2.0 {
                model.course.ball.pos[0] = -model.course.map.w/2.0;
                model.course.ball.dir = BallDirection::UP;
            }
        },
        BallDirection::UP => {
            model.course.ball.pos[1] += speed as f32;
            if model.course.ball.pos[1] >= model.course.map.h/2.0 {
                model.course.ball.pos[1] = model.course.map.h/2.0;
                if model.course.turn_cnt >= model.course.turn_len {
                    // TODO => change course
                    model.course.turn_cnt = 0; // KARI
                    model.course.ball.dir = BallDirection::RIGHT; // KARI
                } else {
                    model.course.turn_cnt += 1;
                    model.course.ball.dir = BallDirection::RIGHT;
                }
            }
        },
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(GRAY);

    let course = &model.course;
    let count_text = format!("{} / {}", model.course.turn_cnt, model.course.turn_len);
    let millis_text = format!("mills_per_frame : {}", &app.duration.since_prev_update.as_millis().to_string());
    draw.rect().w_h(course.map.w , course.map.h).x_y(0.0, 0.0).no_fill().stroke_color(WHITE).stroke_weight(2.0);
    draw.ellipse().radius(course.ball.radius).x_y(course.ball.pos[0], course.ball.pos[1]);
    draw.text(&count_text).font_size(24).y(50.0);
    draw.text(&millis_text).font_size(24).y(-50.0);

    let mut i = -course.map.w/2.0;
    while i < course.map.w/2.0 {
        let p1 = Point2 {x:i , y:course.map.h/2.0 + 5.0};
        draw.line()
        .start(p1)
        .end(Point2 {x:i , y:course.map.h/2.0 + NOTE_INTERVAL});
        //.rotate(deg_to_rad(90.0));
        draw.line()
        .start(Point2 {x:i , y:course.map.h/2.0 + NOTE_INTERVAL})
        .end(Point2 {x:i+NOTE_INTERVAL-5.0 , y:course.map.h/2.0 + NOTE_INTERVAL});
        i += NOTE_INTERVAL;
    }


    draw.to_frame(app, &frame).unwrap();
}
