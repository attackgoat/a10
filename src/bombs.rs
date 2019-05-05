use futures::{Async, Stream};

use super::receiver::Receiver;
use super::servo::Servo;

const SAFE: u8 = 0x00;
const DROP: u8 = 0xff;

// The number of RX channel frames we wait between bomb drops
const DELAY_FRAMES: usize = 250;

pub struct Bombs {
    frame_count: usize,
    receiver: Receiver,
    servos: [Servo; 5],
}

impl Bombs {
    pub fn update(&mut self) {
        // See if we can drop the bombs!
        match self.switch_pos() {
            SwitchPosition::On1 => {
                // At the first detent we drop the center bomb
                self.servos[2].set_value(DROP);
            },
            SwitchPosition::On2 => {
                // At the second detent we drop the remaining 4 bombs in sequence
                if self.frame_count > DELAY_FRAMES << 2 {
                    self.servos[4].set_value(DROP);
                } else if self.frame_count > DELAY_FRAMES * 3 {
                    self.servos[0].set_value(DROP);
                    self.frame_count += 1;
                } else if self.frame_count > DELAY_FRAMES << 1 {
                    self.servos[3].set_value(DROP);
                    self.frame_count += 1;
                } else if self.frame_count > DELAY_FRAMES {
                    self.servos[1].set_value(DROP);
                    self.frame_count += 1;
                }
            },
            _ => {
                // Set all bombs to safety mode
                self.frame_count = 0;
                for servo in &mut self.servos {
                    servo.set_value(SAFE);
                }
            },
        }

        // Let all the servos do their magic
        for servo in &mut self.servos {
            servo.update();
        }
    }

    fn switch_pos(&mut self) -> SwitchPosition {
        // Get the RX channel signal value
        let rx = match self.receiver.poll() {
            Ok(Async::Ready(Some(sig))) => sig,
            _ => return SwitchPosition::Off,
        };

        // Get the switch position from that
        return match rx {
            sig if sig > 0xc0 => SwitchPosition::On2,
            sig if sig > 0x40 => SwitchPosition::On1,
            _ => SwitchPosition::Off,
        };
    }
}

impl Default for Bombs {
    fn default() -> Self {
        Self {
            frame_count: 0,
            receiver: Receiver::new(),
            servos: Default::default(),
        }
    }
}

enum SwitchPosition {
    Off,
    On1,
    On2,
}
