use bevy::{
    math::{Quat, Vec2, Vec3},
    prelude::{EventReader, TouchInput},
};

pub fn touch_detection(mut touch_evr: EventReader<TouchInput>) {
    use bevy::input::touch::TouchPhase;
    for ev in touch_evr.read() {
        let mut start = Vec3::new(0.0, 0.0, 1.0);
        let mut end = Vec3::new(0.0, 0.0, 1.0);
        // in real apps you probably want to store and track touch ids somewhere
        match ev.phase {
            TouchPhase::Started => {
                println!("Touch {} started at: {:?}", ev.id, ev.position);
                start.x = ev.position.x;
                start.y = ev.position.y;
            }
            TouchPhase::Moved => {
                println!("Touch {} moved to: {:?}", ev.id, ev.position);
            }
            TouchPhase::Ended => {
                println!("Touch {} ended at: {:?}", ev.id, ev.position);
                end.x = ev.position.x;
                end.y = ev.position.y;
            }
            TouchPhase::Canceled => {
                println!("Touch {} cancelled at: {:?}", ev.id, ev.position);
            }
        }

        let angle = start.angle_between(end);
        println!("final angle is {}", angle);
    }
}
