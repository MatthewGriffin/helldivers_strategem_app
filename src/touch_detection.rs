use bevy::prelude::{EventReader, TouchInput};

pub fn touch_detection(mut touch_evr: EventReader<TouchInput>) {
    use bevy::input::touch::TouchPhase;
    for ev in touch_evr.read() {
        // in real apps you probably want to store and track touch ids somewhere
        match ev.phase {
            TouchPhase::Started => {
                println!("Touch {} started at: {:?}", ev.id, ev.position);
            }
            TouchPhase::Moved => {
                println!("Touch {} moved to: {:?}", ev.id, ev.position);
            }
            TouchPhase::Ended => {
                println!("Touch {} ended at: {:?}", ev.id, ev.position);
            }
            TouchPhase::Canceled => {
                println!("Touch {} cancelled at: {:?}", ev.id, ev.position);
            }
        }
    }
}
