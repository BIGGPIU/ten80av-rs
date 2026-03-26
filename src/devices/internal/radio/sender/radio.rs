use embedded_hal::delay::DelayNs;
use microbit::hal::Timer;
use microbit::hal::{Clocks, clocks::ExternalOscillator};
use microbit::hal::ieee802154::{self, Packet};
use microbit::pac::TIMER0;
use core::fmt::Write;


pub enum RadioError {
    Timeout,
    /// If you're a slave/master and you try to do something only a master/slave can do
    InvalidStatus
}

pub struct Radio<'a> {
    controller:microbit::hal::ieee802154::Radio<'a>,
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


        
        return Radio { controller:  radio, timeout};
    }

    /// tries to write bytes to the Radio
    /// 
    /// Master only action
    pub fn write(&mut self, message:&[u8], timer: &mut Timer<TIMER0>) -> Result<(),RadioError> {
        
        let mut packet = Packet::new();

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
            return Err(RadioError::Timeout)
        }
        else {
            return Ok(())
        }

        
    }


    pub fn set_timeout(&mut self, new_timeout:u32) {
        self.timeout = new_timeout;
    }

}