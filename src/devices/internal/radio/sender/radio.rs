use embedded_hal::delay::DelayNs;
use microbit::hal::Timer;
use microbit::hal::{Clocks, clocks::ExternalOscillator};
use microbit::hal::ieee802154::{self, Packet};
use microbit::pac::TIMER0;
use core::fmt::Write;

use crate::utils::serial::Serial;


pub enum RadioError {
    /// I'm not gonna write anything for this one you're not stupid
    Timeout,
    /// If you're a slave/master and you try to do something only a master/slave can do
    InvalidStatus
}

/// Sender Radio For the Micro:bit V2
pub struct Radio<'a> {
    controller:microbit::hal::ieee802154::Radio<'a>,
    password:Option<[u8;3]>,
    timeout:u32,

}



impl Radio<'_> {


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
        timeout:u32,
        serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>
    ) -> crate::devices::internal::radio::sender::radio::Radio<'a> {

        write!(serial, "initiating radio... \r\n").unwrap();
        

        let mut radio: microbit::hal::ieee802154::Radio<'a> = microbit::hal::ieee802154::Radio::init(board_radio, &clocks);
        
        radio.set_channel(channel);


        
        return Radio { controller:  radio, timeout,password: None};
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
        timeout:u32
    ) -> crate::devices::internal::radio::sender::radio::Radio<'a> {

        let mut radio: microbit::hal::ieee802154::Radio<'a> = microbit::hal::ieee802154::Radio::init(board_radio, &clocks);
        
        radio.set_channel(channel);


        
        return Radio { controller:  radio, timeout,password: None};
    }

    /// tries to write bytes to the Radio
    /// 
    /// Example:
    /// 
    /// ```rust
    /// todo!(":3")
    /// ```
    pub fn write(&mut self, message:&[u8;16], timer: &mut Timer<TIMER0>,serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>) -> Result<(),RadioError> {
        
        Serial::write(serial, "Attempting to broadcast a message...",crate::utils::serial::MessageSeverity::INFORMATIVE);

        let mut packet = Packet::new();

        // add the password to the packet if it exists
        match self.password {
            Some(password) => {
                packet.copy_from_slice(&password);
            },
            None => {

            },
        }

        packet.copy_from_slice(message);
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

    /// # Preface
    /// Microbit radios communicate using the IEEE 802.15.4 standard they dont use any sort of identification when messages are sent
    /// So basically you have a whole bunch of devices communicating with eachother with no way to identify who the device is.
    /// This might be good if you're in an environment like your house where you would only have one or two Microbits communicating at the same time
    /// but this becomes a problem at competitions where there are 5+ different microbits sending their own signals.
    /// 
    /// The old solution would just have people using different wavelengths, which works completely fine. This system is here to cut the communication out.
    /// 
    /// ( still go talk to the other teams though, be nice )
    /// 
    /// # Usage
    /// 
    /// Setting the password enables the password across all commands. (except read_to_serial)'
    /// 
    /// The password is 3 bytes long and whatever message recieved will check if the first three bytes match the passwords three bytes.
    /// if they dont match eachother then immediately stop reading the message.
    /// otherwise continue to read the message and go through the function like usual.
    /// 
    /// **Note: Even though the word "password" is used, the code isnt really made to be secret. Identification bytes would probably be a more suitable name
    /// but I like password more**
    pub fn set_password(&mut self, pass:[u8;3]) {
        self.password = Some(pass);
    }

    /// Stop using the password you set
    pub fn disable_password(&mut self) {
        self.password = None;
    }


    /// set the amount of time to wait while sending a message before giving up.
    pub fn set_timeout(&mut self, new_timeout:u32) {
        self.timeout = new_timeout;
    }

}