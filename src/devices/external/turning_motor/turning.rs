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
    channel:Channel
}


impl TurningMotor {

    /// Only accepts ports labeled SX
    /// 
    /// Trying to pass a MX port will cause this function to return an Error
    pub fn new(channel:MicrotbitDriverPorts) -> Result<TurningMotor,ServoErrors>{

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
            channel: channel.servo_get()
        })
    }

    /// takes a value between -45 and 45  
    /// then tells the servo to go to that angle 
    // pub fn change_servo_angle(&mut self,servo_motor_controller: &mut ServoMotorController, deg:i8) -> Result<(),ServoErrors> {
    //     if deg.abs() > 45 {
    //         return Err(ServoErrors::OutOfBoundsAngle)
    //     }

    //     let new_pwm_value = (deg as i16).map_range(-45..45, 150..600) as u16;

    //     servo_motor_controller.controller.set_channel_on_off(self.channel, 0, new_pwm_value).unwrap();

    //     Ok(())
    // }

    /// change a servo angle when you have a specific PWM value you want to set it to
    /// 
    /// Neutral value: 200
    /// 
    /// Read the documentation about the motor here: <https://d3if9wubzr0anm.cloudfront.net/pds/2213210-1.pdf>
    pub fn raw_change_servo_angle(&mut self, servo_motor_controller: &mut ServoMotorController,value:u16) {
        servo_motor_controller.controller.set_channel_on_off(self.channel, 0, value).unwrap();
    }

    /// resets the servo to a neutral direction
    pub fn reset_direction(&mut self, servo_motor_controller: &mut ServoMotorController) {
        servo_motor_controller.controller.set_channel_on_off(self.channel, 0, 200).unwrap();
    }   
}