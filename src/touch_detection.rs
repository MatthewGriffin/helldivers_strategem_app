use bevy::prelude::{EventReader, EventWriter, ResMut, TouchInput};

use crate::{process_touch_event::ProcessTouchEvent, touch_resource::TouchResource};

pub fn touch_detection(
    mut touch_evr: EventReader<TouchInput>,
    mut current_touch: ResMut<TouchResource>,
    mut process_touch_event_sender: EventWriter<ProcessTouchEvent>,
) {
    use bevy::input::touch::TouchPhase;
    for ev in touch_evr.read() {
        // in real apps you probably want to store and track touch ids somewhere
        match ev.phase {
            TouchPhase::Started => {
                println!("Touch {} started at: {:?}", ev.id, ev.position);
                current_touch.touch_start = ev.position;
            }
            TouchPhase::Moved => {
                //println!("Touch {} moved to: {:?}", ev.id, ev.position);
            }
            TouchPhase::Ended => {
                println!("Touch {} ended at: {:?}", ev.id, ev.position);
                current_touch.touch_end = ev.position;

                let diff = current_touch.touch_start - current_touch.touch_end;
                println!("diff is {}", diff);
                process_touch_event_sender.send(ProcessTouchEvent {
                    start_end_diff: diff,
                });
            }
            TouchPhase::Canceled => {
                println!("Touch {} cancelled at: {:?}", ev.id, ev.position);
            }
        }
    }
}
