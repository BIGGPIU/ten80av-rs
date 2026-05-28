use embedded_hal::delay::DelayNs;
use microbit::hal::Timer;
use microbit::hal::{Clocks, clocks::ExternalOscillator};
use microbit::hal::ieee802154::{self, Packet};
use microbit::pac::TIMER0;
use core::fmt::Write;

use crate::utils::MicrobitMessageFormat;
use crate::utils::serial::Serial;


pub enum RadioError {
    /// I'm not gonna write anything for this one you're not stupid
    Timeout,
    /// If you're a slave/master and you try to do something only a master/slave can do
    InvalidStatus
}

/// Sender Radio For the Micro:bit V2
/// 
/// This one sends information over the radio using the protocol developed by the Calvert Cruisers. This is only one standard for 
/// sending and recieving data over radio. If you do not wish to use it and would like to make your own then please refer to [INSERT RADIO.RS LINK]
/// 
/// 
/// 
/// # Key Differences between Radio and CalvertCruisersStandardRadio (CCSR)
/// * CCSR Requires a password.
/// * CCSR Passwords are 4 bytes rather than 3 bytes 
/// * CCSR can only send messages if they come from a source that implements [INSERT MESSAGEFORMAT TRAIT]
/// * CCSR sends messages in 20 byte messages rather versus the 16 byte long messages sent with Radio
pub struct CalverCruisersStandardRadio<'a> {
    controller:microbit::hal::ieee802154::Radio<'a>,
    password:[u8;4],
    timeout:u32,
}

impl CalverCruisersStandardRadio<'_> {


    /// Initialize the radio.
    /// 
    /// The arguments are as follows: (
    ///     
    /// board_radio: From Board.RADIO,
    ///     
    /// clocks: Reference from Board.CLOCKS,
    ///     
    /// channel: What channel you want the radio to listen in on
    ///     
    /// timeout: How long IN MICROSECONDS do you want to wait to recieve something 
    /// 
    /// )
    pub fn new<'a>(
        board_radio:microbit::pac::RADIO,
        clocks:&'a Clocks<ExternalOscillator, microbit::hal::clocks::Internal, microbit::hal::clocks::LfOscStopped>,
        channel:ieee802154::Channel,
        password:[u8;4],
        timeout:u32,
        serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>
    ) -> CalverCruisersStandardRadio<'a> {

        Serial::write(serial, "Starting Radio Configured As A Sender", crate::utils::serial::MessageSeverity::INFORMATIVE);
        

        let mut radio: microbit::hal::ieee802154::Radio<'a> = microbit::hal::ieee802154::Radio::init(board_radio, &clocks);
        
        radio.set_channel(channel);

        
        
        return CalverCruisersStandardRadio { controller:  radio, timeout,password};
    }

    /// Initialize the radio.
    /// 
    /// The arguments are as follows: (
    ///     
    /// board_radio: From Board.RADIO,
    ///     
    /// clocks: Reference from Board.CLOCKS,
    ///     
    /// channel: What channel you want the radio to listen in on
    ///     
    /// timeout: How long IN MICROSECONDS do you want to wait to recieve something 
    /// 
    /// )
    pub fn new_nolog<'a>(
        board_radio:microbit::pac::RADIO,
        clocks:&'a Clocks<ExternalOscillator, microbit::hal::clocks::Internal, microbit::hal::clocks::LfOscStopped>,
        channel:ieee802154::Channel,
        password:[u8;4],
        timeout:u32
    ) -> CalverCruisersStandardRadio<'a> {

        let mut radio: microbit::hal::ieee802154::Radio<'a> = microbit::hal::ieee802154::Radio::init(board_radio, &clocks);
        
        radio.set_channel(channel);


        
        return CalverCruisersStandardRadio { controller:  radio, timeout,password};
    }

    /// tries to write bytes to the Radio
    /// 
    /// Example:
    /// 
    /// ```rust
    /// todo!(":3")
    /// ```
    pub fn write(&mut self, message:&impl MicrobitMessageFormat, timer: &mut Timer<TIMER0>,serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>) -> Result<(),RadioError> {
        
        Serial::write(serial, "Attempting to broadcast a message...",crate::utils::serial::MessageSeverity::INFORMATIVE);

        let mut packet = Packet::new();

        let message_buf = message.create_message_slice();

        let buf = [
            self.password[0],
            self.password[1],
            self.password[2],
            self.password[3],
            message_buf[0],
            message_buf[1],
            message_buf[2],
            message_buf[3],
            message_buf[4],
            message_buf[5],
            message_buf[6],
            message_buf[7],
            message_buf[8],
            message_buf[9],
            message_buf[10],
            message_buf[11],
            message_buf[12],
            message_buf[13],
            message_buf[14],
            message_buf[16],
        ];

        packet.copy_from_slice(&buf);

      
        let mut cycles_passed = 0;

        while self.timeout >= cycles_passed {

            match self.controller.try_send(&mut packet) {
                Ok(_) => break,
                Err(_) => {
                    
                },
            }
            
            timer.delay_us(1);
            cycles_passed += 1;
        }


        if self.timeout == cycles_passed {
            Serial::write(serial,"Radio Timeout", crate::utils::serial::MessageSeverity::Warning);
            return Err(RadioError::Timeout)
        }
        else {
            return Ok(())
        }

        
    }

    /// set the amount of time to wait while sending a message before giving up.
    pub fn set_timeout(&mut self, new_timeout:u32) {
        self.timeout = new_timeout;
    }

}