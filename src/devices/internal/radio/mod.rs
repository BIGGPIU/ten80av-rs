//! Module for using the Micro:bit as a radio master or radio reciever.
//! 
//! # Overview
//! * [RecieverRadio] - Module for using the Micro:Bit as a reciever
//! * [SenderRadio] - Module for using the Micro:bit as a sender
//! 
//! # Radio Passwords
//! 
//! ## Preface
//! Microbit radios communicate using the IEEE 802.15.4 standard they dont use any sort of identification when messages are sent
//! So basically you have a whole bunch of devices communicating with eachother with no way to identify who the device is.
//! This might be good if you're in an environment like your house where you would only have one or two Microbits communicating at the same time
//! but this becomes a problem at competitions where there are 5+ different microbits sending their own signals.
//! 
//! The old solution would just have people using different wavelengths, which works completely fine. This system is here to cut the communication out.
//! 
//! ( still go talk to the other teams though, be nice )
//! 
//! ## Usage
//! 
//! Setting the password enables the password across all commands. (except [`RecieverRadio::read_to_serial`])
//! 
//! The password is 3 bytes long and whatever message recieved will check if the first three bytes match the passwords three bytes.
//! if they dont match eachother then immediately stop reading the message.
//! otherwise continue to read the message and go through the function like usual.
//! 
//! **Note: Even though the word "password" is used, the code isnt really made to be secret. Identification bytes would probably be a more suitable name
//! but I like password more**
//! 
//! Heres an example of what a sample packet looks like for you visual learners out there
//! 
//! | 0             | 1             | 2             | 3..18   |
//! | ------------- | ------------- | ------------- | ------- |
//! | Password Byte | Password Byte | Password Byte | Message |


mod reciever;
mod sender;

pub use reciever::radio::Radio as RecieverRadio;
pub use sender::radio::Radio as SenderRadio;
pub use reciever::radio::RadioError as RecieverError;
pub use sender::radio::Radio as SenderError;