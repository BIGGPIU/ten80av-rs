use crate::devices::internal::radio::reciever::calvert_cruisers_radio::RadioError::{IncorrectPassword, ModifiedMessageErr};
use crate::devices::internal::radio::reciever::calvert_cruisers_radio::RadioRecievedMessages::RecievedIrMessage;
use crate::utils::{IR_SENSOR_IDENTIFIER, MAGNOMETER_MESSAGE_IDENTIFIER, RADIO_MESSAGE_IDENTIFIER, ULTRASONIC_DISTANCE_SENSOR_IDENTIFIER};
use crate::utils::serial::Serial;

use microbit::hal::Timer;
use microbit::hal::{Clocks, clocks::ExternalOscillator};
use microbit::hal::ieee802154::{self, Packet};
use microbit::pac::TIMER0;

use core::fmt::Write;


pub enum RadioError {
    Timeout,
    /// If you're a slave/master and you try to do something only a master/slave can do
    InvalidStatus,
    /// an error you should expect honestly, just means that the first 3 bytes read are not the bytes that you set as your password
    IncorrectPassword,
    /// If the wrong CRC is found. This still returns the message in case you may want to read it 
    ModifiedMessage([u8;20]),
    /// If the wrong CRC is found. This does not return the possibly tampered message
    ModifiedMessageErr,
    /// If the message is shorter than expected
    BadMessageLength
}

pub struct LeftIRValue(i16);
pub struct RightIRValue(i16);
/// the x value returned from a magnometer
pub struct XValue(i16);
/// the y value returned from a magnometer
pub struct YValue(i16);
/// the z value returned from a magnometer
pub struct ZValue(i16);

pub enum RadioRecievedMessages {
    RecievedIrMessage(LeftIRValue,RightIRValue),
    RecievedUDSMessage(u32),
    RecievedRadioMessage([u8;11]),
    RecievedMagnometerMessage(XValue,YValue,ZValue),
    /// when we cant decipher what the message might be 
    Unknown([u8;16])
}

/// Reciever Radio For the Micro:bit V2
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

        // write!(serial, "initiating radio... \r\n").unwrap();
        
        Serial::write(serial,"Initializing Radio", crate::utils::serial::MessageSeverity::INFORMATIVE);
        

        let mut radio: microbit::hal::ieee802154::Radio<'a> = microbit::hal::ieee802154::Radio::init(board_radio, &clocks);
        
        radio.set_channel(channel);

        Serial::write(serial,"Successfully Initialized Radio", crate::utils::serial::MessageSeverity::OK);

        
        return CalverCruisersStandardRadio { controller:  radio, timeout, password};
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
  
        return CalverCruisersStandardRadio { controller:  radio, timeout, password};
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

            for i in packet.iter() {
                write!(serial, " {:?} ",i ).unwrap();
            }

            write!(serial,"\r\n").unwrap();
        }
        else {
            // write!(serial, "Packet:").unwrap();
            Serial::write(serial,"Reading Radio Packet to Serial", crate::utils::serial::MessageSeverity::INFORMATIVE);
            for i in packet.iter() {
                write!(serial, " {:?} ",i ).unwrap();
            }
            write!(serial,"\r\n").unwrap();
        }
        
        return Ok(())

    }

    /// Attempt to read 20 bytes from the radio. 
    /// 
    /// Heres an example of what a sample packret looks like for you visual learners out there
    /// 
    /// | 0             | 1             | 2             | 3..20   |
    /// | ------------- | ------------- | ------------- | ------- |
    /// | Password Byte | Password Byte | Password Byte | Message |
    pub fn read(&mut self,timer: &mut Timer<TIMER0>,serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>) -> Result<[u8;20], RadioError> {
        Serial::write(serial,"Attempting to read a broadcasted message to serial monitor", crate::utils::serial::MessageSeverity::INFORMATIVE);
        let mut packet: Packet = Packet::new();
        let wrong_crc:bool;


        // try to recieve a packet 
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
            Serial::write(serial,"Reading Radio Packet to Buffer", crate::utils::serial::MessageSeverity::INFORMATIVE);
            // write!(serial, "! Packet may have been modified. Packet:").unwrap();

            return CalverCruisersStandardRadio::read_message_with_password(self.password, packet,true);
            
        }
        else {
            // write!(serial, "Packet:").unwrap();
            Serial::write(serial,"Reading Radio Packet to Buffer", crate::utils::serial::MessageSeverity::INFORMATIVE);

            return CalverCruisersStandardRadio::read_message_with_password(self.password, packet, false);
        }


    }

    fn read_message_with_password(password:[u8;4],packet:Packet,bad_crc:bool) -> Result<[u8;20], RadioError>{
        let mut message_buf = [245;20];


        let mut packet_iter = packet.iter();
        
        // check to see if the password bytes are right
        for p in 0..4 {
            let m = match packet_iter.next() {
                Some(x) => x,
                None => {
                    return Err(RadioError::BadMessageLength)
                },
            };

            if m == &password[p] {
                continue;
            } 
            else {
                return Err(RadioError::IncorrectPassword)
            }
        }

        for i in 0..16 {
            let message = match packet_iter.next() {
                Some(x) => x,
                None => {
                    return Err(RadioError::BadMessageLength)
                },
            };

            message_buf[i] = *message;
        }

        if bad_crc {
            return Err(RadioError::ModifiedMessage(message_buf))
        }
        else {
            return Ok(message_buf)
        }
    }

    /// Read the packet and attempt to identify what the challenge is apart of 
    pub fn read_and_identify_package(&mut self,timer: &mut Timer<TIMER0>,serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>) -> Result<RadioRecievedMessages, RadioError>{
        Serial::write(serial,"Attempting to read a broadcasted message to serial monitor", crate::utils::serial::MessageSeverity::INFORMATIVE);
        let mut packet = Packet::new();
        let mut message_buf = [0_u8;20];

        match self.controller.recv_timeout(&mut packet, timer, self.timeout) {
            Ok(_) => {
                // do nothing
            },
            Err(e) => {
                match e {
                    ieee802154::Error::Crc(_) => {
                        return Err(ModifiedMessageErr)
                    },
                    ieee802154::Error::Timeout => {
                        return Err(RadioError::Timeout)
                    },
                }
            },
        }

        let mut message_ptr = 0;
        let mut packet_iter = packet.iter();

        // check to see if the password is right
        for _ in 0..4 {
            let m = match packet_iter.next() {
                Some(x) => x,
                None => {
                    return Err(RadioError::BadMessageLength)
                },
            };

            if m == &self.password[message_ptr] {
                message_buf[message_ptr] = *m;
                message_ptr += 1;
                continue;
            }
            else {
                return Err(IncorrectPassword)
            }

        }

        // fill the rest of the message in after we've checked for the password
        for _ in 0..16 {
            match packet_iter.next() {
                Some(x) => {
                    message_buf[message_ptr] = *x;
                    message_ptr += 1;
                },
                None => {
                    return Err(RadioError::BadMessageLength)
                },
            }
        }

        // try to identify what the message is and return stuff accordingly
        match message_buf[6] {
            IR_SENSOR_IDENTIFIER => {
                return Ok(RecievedIrMessage(
                    LeftIRValue(i16::from_le_bytes([message_buf[7],message_buf[8]])),
                    RightIRValue(i16::from_le_bytes([message_buf[9],message_buf[10]]))
                ))
            },
            RADIO_MESSAGE_IDENTIFIER => {
                let mut radio_message = [0_u8;11];
                radio_message.copy_from_slice(&message_buf[7..18]);
                return Ok(RadioRecievedMessages::RecievedRadioMessage(
                    radio_message
                ))
            },
            MAGNOMETER_MESSAGE_IDENTIFIER => {
                return Ok(RadioRecievedMessages::RecievedMagnometerMessage(
                    XValue(i16::from_le_bytes([message_buf[7],message_buf[8]])),
                    YValue(i16::from_le_bytes([message_buf[9],message_buf[10]])),
                    ZValue(i16::from_le_bytes([message_buf[11],message_buf[12]]))
                ))
            },
            ULTRASONIC_DISTANCE_SENSOR_IDENTIFIER => {
                return Ok(
                    RadioRecievedMessages::RecievedUDSMessage(
                        u32::from_le_bytes([
                            message_buf[7],
                            message_buf[8],
                            message_buf[9],
                            message_buf[10],
                        ])
                    )
                )
            }
            _ => {
                // unknown
                let mut unk_message = [0_u8;16];
                unk_message.copy_from_slice(&message_buf[4..20]);
                return Ok(
                    RadioRecievedMessages::Unknown(
                        unk_message
                    )
                )
            }
        }
    }

    /// Set the amount of time to wait before giving up
    pub fn set_timeout(&mut self, new_timeout:u32) {
        self.timeout = new_timeout;
    }

}