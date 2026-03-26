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
    ) -> crate::devices::internal::radio::reciever::radio::Radio<'a> {

        write!(serial, "initiating radio... \r\n").unwrap();
        

        let mut radio: microbit::hal::ieee802154::Radio<'a> = microbit::hal::ieee802154::Radio::init(board_radio, &clocks);
        
        radio.set_channel(channel);


        
        return Radio { controller:  radio, timeout};
    }


    /// tries to read a radio packet and writes it to the serial monitor 
    /// 
    /// Slave only action
    /// 
    /// returns true if successful, none if it timeouts 
    pub fn read_to_serial(&mut self,timer: &mut Timer<TIMER0>,serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>) -> Result<(),RadioError> {

        let _ = write!(serial,"Attempting to read Packet to serial monitor");
        let mut packet = Packet::new();
        let wrong_crc:bool;


        match self.controller.recv_timeout(&mut packet, timer, self.timeout) {
            Ok(_x) => {
                wrong_crc = false;  
            },
            Err(e) => {
                match e {
                    ieee802154::Error::Crc(_) => {
                        wrong_crc = true;
                    },
                    ieee802154::Error::Timeout => {
                        return Err(RadioError::Timeout)
                    },
                }
            },
        }

        if wrong_crc {
            
            write!(serial, "! Packet may have been modified. Packet:").unwrap();
            for i in packet.iter() {
                write!(serial, " {:?} ",i ).unwrap();
            }
        }
        else {
            write!(serial, "Packet:").unwrap();
            for i in packet.iter() {
                write!(serial, " {:?} ",i ).unwrap();
            }
        }
        
        return Ok(())

    }

    pub fn set_timeout(&mut self, new_timeout:u32) {
        self.timeout = new_timeout;
    }

}