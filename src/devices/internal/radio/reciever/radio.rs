use crate::utils::serial::Serial;

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
    ) -> crate::devices::internal::radio::reciever::radio::Radio<'a> {

        // write!(serial, "initiating radio... \r\n").unwrap();
        
        Serial::write(serial,"Initializing Radio", crate::utils::serial::MessageSeverity::INFORMATIVE);
        

        let mut radio: microbit::hal::ieee802154::Radio<'a> = microbit::hal::ieee802154::Radio::init(board_radio, &clocks);
        
        radio.set_channel(channel);

        Serial::write(serial,"Successfully Initialized Radio", crate::utils::serial::MessageSeverity::OK);

        
        return Radio { controller:  radio, timeout, password: None};
    }


    /// tries to read a radio packet and writes it to the serial monitor 
    /// 
    /// Slave only action
    /// 
    /// returns true if successful, none if it timeouts 
    pub fn read_to_serial(&mut self,timer: &mut Timer<TIMER0>,serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>) -> Result<(),RadioError> {

        // let _ = write!(serial,"Attempting to read Packet to serial monitor");
        Serial::write(serial,"Attempting to read a broadcasted message to serial monitor", crate::utils::serial::MessageSeverity::INFORMATIVE);
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
                        Serial::write(serial,"Radio Timeout", crate::utils::serial::MessageSeverity::Warning);
                        return Err(RadioError::Timeout)
                    },
                }
            },
        }

        if wrong_crc {
            Serial::write(serial,"Radio Packet may have been modified", crate::utils::serial::MessageSeverity::Warning);
            Serial::write(serial,"Reading Radio Packet to Serial", crate::utils::serial::MessageSeverity::INFORMATIVE);
            // write!(serial, "! Packet may have been modified. Packet:").unwrap();
            for i in packet.iter() {
                write!(serial, " {:?} ",i ).unwrap();
            }
        }
        else {
            // write!(serial, "Packet:").unwrap();
            Serial::write(serial,"Reading Radio Packet to Serial", crate::utils::serial::MessageSeverity::INFORMATIVE);
            for i in packet.iter() {
                write!(serial, " {:?} ",i ).unwrap();
            }
        }
        
        return Ok(())

    }

    pub fn read(&mut self,timer: &mut Timer<TIMER0>,serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>) -> Result<Packet, RadioError> {
        Serial::write(serial,"Attempting to read a broadcasted message to serial monitor", crate::utils::serial::MessageSeverity::INFORMATIVE);
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
                        Serial::write(serial,"Radio Timeout", crate::utils::serial::MessageSeverity::Warning);
                        return Err(RadioError::Timeout)
                    },
                }
            },
        }

        if wrong_crc {
            Serial::write(serial,"Radio Packet may have been modified", crate::utils::serial::MessageSeverity::Warning);
            Serial::write(serial,"Reading Radio Packet to Serial", crate::utils::serial::MessageSeverity::INFORMATIVE);
            // write!(serial, "! Packet may have been modified. Packet:").unwrap();

            match self.password {
                Some(password) => {
                    let p = 0_usize;
                    for i in packet.iter() {
                        if p != 3 && i == &password[p] {

                        }  
                    }
                },
                None => {

                },
            }
        }
        else {
            // write!(serial, "Packet:").unwrap();
            Serial::write(serial,"Reading Radio Packet to Serial", crate::utils::serial::MessageSeverity::INFORMATIVE);
            for i in packet.iter() {
                write!(serial, " {:?} ",i ).unwrap();
            }
        }


        Ok(todo!())
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

    pub fn disable_password(&mut self) {
        self.password = None;
    }

    pub fn set_timeout(&mut self, new_timeout:u32) {
        self.timeout = new_timeout;
    }

}