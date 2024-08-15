use bevy::{
    math::{Quat, Vec2, Vec3},
    prelude::{EventReader, TouchInput},
};

pub fn touch_detection(mut touch_evr: EventReader<TouchInput>) {
    use bevy::input::touch::TouchPhase;
    for ev in touch_evr.read() {
        let mut start = Vec2::new(0.0, 0.0);
        let mut end = Vec2::new(0.0, 0.0);
        // in real apps you probably want to store and track touch ids somewhere
        match ev.phase {
            TouchPhase::Started => {
                println!("Touch {} started at: {:?}", ev.id, ev.position);
                start = ev.position;
            }
            TouchPhase::Moved => {
                //println!("Touch {} moved to: {:?}", ev.id, ev.position);
            }
            TouchPhase::Ended => {
                println!("Touch {} ended at: {:?}", ev.id, ev.position);
                end = ev.position;

                let diff = start.dot(end); 
                println!("diff is {}",  diff);
            }
            TouchPhase::Canceled => {
                println!("Touch {} cancelled at: {:?}", ev.id, ev.position);
            }
        }
    }
}
