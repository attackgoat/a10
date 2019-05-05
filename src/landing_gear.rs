use futures::{Async, Stream};

use super::receiver::Receiver;
use super::servo::Servo;

// The number of RX channel frames we wait after opening the door before dropping the gear
const DELAY_DOOR_FRAMES: usize = 500;

// The number of RX channel frames we wait after dropping the gear before closing the door
const DELAY_GEAR_FRAMES: usize = 1650;

pub struct LandingGear {
    door_servo: Servo,
    frame_count: usize,
    gear_servo: Servo,
    receiver: Receiver,
}

impl LandingGear {
    pub fn update(&mut self) {
        match self.switch_pos() {
            SwitchPosition::On => {
            },
            _ => {
            },
        }

        // Let the servos do their magic
        self.door_servo.update();
        self.gear_servo.update();
    }

    fn switch_pos(&mut self) -> SwitchPosition {
        // Get the RX channel signal value
        let rx = match self.receiver.poll() {
            Ok(Async::Ready(Some(sig))) => sig,
            _ => return SwitchPosition::Off,
        };

        // Get the switch position from that
        return match rx {
            sig if sig > 0x80 => SwitchPosition::On,
            _ => SwitchPosition::Off,
        };
    }
}

impl Default for LandingGear {
    fn default() -> Self {
        Self {
            door_servo: Default::default(),
            frame_count: 0,
            gear_servo: Default::default(),
            receiver: Receiver::new(),
        }
    }
}

enum SwitchPosition {
    Off,
    On,
}
