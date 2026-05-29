//! Module for recieving and graping information recieved from the micro:bit
//! 
//! 

pub(crate) const IR_SENSOR_IDENTIFIER:u8 = 0;
pub(crate) const ULTRASONIC_DISTANCE_SENSOR_IDENTIFIER:u8 = 1;
pub(crate) const RADIO_MESSAGE_IDENTIFIER:u8 = 2;
pub(crate) const MAGNOMETER_MESSAGE_IDENTIFIER:u8 = 3;

mod serial_message_structures;
mod graphs;

pub use graphs::{GrapingWindow,ItemQueue,Graphing};
#[cfg(not(feature = "calcru-serial-standard"))]
pub use serial_message_structures::SerialMessage;
#[cfg(feature = "calcru-serial-standard")]
pub use serial_message_structures::*;


// todo: create plotters live plotting interface. have this assume that the user is using the stuff thats included in the library but also 
// give people the option to make their own communication protocolmod serial_message_structures;
