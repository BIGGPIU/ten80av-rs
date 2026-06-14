//! Module for recieving and graphing information recieved from the micro:bit
//! 
//! todo: add examples

#[cfg(feature = "calcru-serial-standard")]
pub const IR_SENSOR_IDENTIFIER:u8 = 0;
#[cfg(feature = "calcru-serial-standard")]
pub const ULTRASONIC_DISTANCE_SENSOR_IDENTIFIER:u8 = 1;
#[cfg(feature = "calcru-serial-standard")]
pub const RADIO_MESSAGE_IDENTIFIER:u8 = 2;
#[cfg(feature = "calcru-serial-standard")]
pub const MAGNOMETER_MESSAGE_IDENTIFIER:u8 = 3;

mod serial_message_structures;
mod graphs;

#[cfg(feature = "graphing")]
pub use graphs::{ItemQueueReader,ItemQueue,Graphing,GraphSize};

#[cfg(not(feature = "calcru-serial-standard"))]
pub use serial_message_structures::SerialMessage;
#[cfg(feature = "calcru-serial-standard")]
pub use serial_message_structures::*;
