use crate::utils::display::art::*;
use microbit::{display::blocking::Display, hal::Timer, pac::TIMER0};


pub struct DisplayFuncs;



impl DisplayFuncs {
    /// the same as display_message but not as cool
    pub fn display_message(message:&str, display: &mut Display, timer: &mut Timer<TIMER0>, display_time:usize) {
        for c in message.chars() {
            display.show(timer, get_display_letter_from_char(c), (display_time/message.len()) as u32);
        }
    } 
}