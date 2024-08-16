use bevy::prelude::EventReader;

use crate::process_touch_event::ProcessTouchEvent;

pub fn process_input(mut input_to_process: EventReader<ProcessTouchEvent>) {
    for input in input_to_process.read() {
        let diff = input.start_end_diff;
        //if swipe upwards
        if (diff.x <= 10.0 || diff.x >= -10.0) && diff.y >= 100.0 {
            println!("Detected Up Swipe");
        }
        //if swipe down
        else if (diff.x <= 10.0 || diff.x >= -10.0) && diff.y <= -100.0 {
            println!("Detected Down Swipe");
        }
        //if swipe right
        else if (diff.y <= 10.0 || diff.y >= -10.0) && diff.x <= -100.0 {
            println!("Detected Right Swipe");
        }
        //if swipe left
        else if (diff.y <= 10.0 || diff.y >= -10.0) && diff.x >= 100.0 {
            println!("Detected Left Swipe");
        }
    }
}
