//! Module for controlling the motor that controls turning the autonomous vehicle
//! servo documentation: <https://d3if9wubzr0anm.cloudfront.net/pds/2213210-1.pdf>


use pwm_pca9685::{Channel};

use crate::utils::ports::*;
use crate::devices::external::{ServoErrors, ServoMotorController};
// use crate::error_handler::ServoErrors;





#[derive(Debug)]
/// Interface for External Turning Servos 
/// 
/// 0 deg at pwm_on 200
/// 
/// max at 600
///  
/// min at 150
pub struct TurningMotor {
    // no generics since this is probably only ever gonna be used with the microbit
    // unless if some mad man wants to step up above this and use an esp32 or something
    channel:Channel,
    neutral_direction:u16
}


impl TurningMotor {

    /// Only accepts ports labeled SX
    /// 
    /// Trying to pass a MX port will cause this function to return an Error
    pub fn new(channel:MicrotbitDriverPorts,neutral_direction:u16) -> Result<TurningMotor,ServoErrors>{

        match channel {
            MicrotbitDriverPorts::M1 => {
                return Err(ServoErrors::DisallowedPort)
            },
            MicrotbitDriverPorts::M2 => {
                return Err(ServoErrors::DisallowedPort)
            },
            MicrotbitDriverPorts::M3 => {
                return Err(ServoErrors::DisallowedPort)
            },
            MicrotbitDriverPorts::M4 => {
                return Err(ServoErrors::DisallowedPort)
            },
            _ => {
                
            }
        }

        return Ok(TurningMotor {
            channel: channel.servo_get(),
            neutral_direction
        })
    }

    /// change a servo angle when you have a specific PWM value you want to set it to
    /// 
    /// Read the documentation about the motor here: <https://d3if9wubzr0anm.cloudfront.net/pds/2213210-1.pdf>
    pub fn raw_change_servo_angle(&mut self, servo_motor_controller: &mut ServoMotorController,value:u16) {
        servo_motor_controller.controller.set_channel_on_off(self.channel, 0, value).unwrap();
    }

    /// resets the servo to a neutral direction
    pub fn reset_direction(&mut self, servo_motor_controller: &mut ServoMotorController) {
        servo_motor_controller.controller.set_channel_on_off(self.channel, 0, self.neutral_direction).unwrap();
    }   
}