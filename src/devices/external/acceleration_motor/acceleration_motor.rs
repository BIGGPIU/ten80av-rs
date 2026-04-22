//! Module for controlling the motor that controls Acceleration the autonomous vehicle


use core::fmt::Write;
// use crate::{error_handler::ServoErrors, port_translations::{self}, servomotorcontroller::ServoMotorController};
// use map_range::MapRange;
use crate::utils::ports::*;

pub enum MotorState {
    Coast,
    Reverse,
    Forward,
    Brake
}

/// Interface for External Acceleration Motors
/// 
/// Motor Max: 4095
/// Motor min: 0
/// Motor Center: 4095/2
pub struct AccelerationMotor {
    // no generics since this is probably only ever gonna be used with the microbit
    // unless if some mad man wants to step up above this and use an esp32 or something
    channel:MotorPort,
    speed:u16,
    state:MotorState
}

#[derive(Debug)]
pub enum ServoErrors {
    DisallowedPort,
    OutOfBoundsAngle,
}

impl AccelerationMotor {
    pub fn new(channel:MicrotbitDriverPorts) -> Result<AccelerationMotor,ServoErrors>{

        match channel {
            MicrotbitDriverPorts::M1 => {
                
            },
            MicrotbitDriverPorts::M2 => {
                
            },
            MicrotbitDriverPorts::M3 => {
                
            },
            MicrotbitDriverPorts::M4 => {
                
            },
            _ => {
                return Err(ServoErrors::DisallowedPort)
            }
        }

        return Ok(AccelerationMotor {
            channel: channel.motor_get(),
            speed:0_u16,
            state:MotorState::Brake
        })
    }



    /// change the state of the motor
    /// 
    /// This function works by changing the state of the motor depending on which combination of ports are enabled.
    /// 
    /// The documentation describes it as such:
    /// 
    /// ## Moving Forward:
    /// Forward Port: Enabled ✅
    /// 
    /// Reverse Port: Disabled ❎
    /// 
    /// ## Reversing:
    /// Forward Port: Disabled ❎
    /// 
    /// Reverse Port: Enabled ✅
    /// 
    /// ## Coasting / Fast Decay
    /// Forward Port: Disabled ❎
    /// 
    /// Reverse Port: Disabled ❎
    /// 
    /// ## Brake / Slow Decay
    /// Forward Port: Enabled ✅
    /// 
    /// Reverse Port: Enabled ✅
    /// 
    /// 
    /// Read the fucking manual for more info: <https://dfimg.dfrobot.com/wiki/17542/DFR0548_gravity-hr8833-motor-and-servo-driver-expansion-board_schematics_v1.zip>
    pub fn change_state(&mut self, servo_motor_controller: &mut crate::devices::external::servomotorcontroller::ServoMotorController,new_state:MotorState) {
        self.state = new_state;
        self.write_to_motor_chip(servo_motor_controller);
    }


    /// logging version of change_state
    pub fn paranoid_change_state(&mut self, servo_motor_controller: &mut crate::devices::external::servomotorcontroller::ServoMotorController,new_state:MotorState, serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>) {
        self.state = new_state;
        self.write_to_motor_chip_logging(servo_motor_controller,serial);
    }

    /// change the speed of the motor
    /// then updates the motor with the speed you set
    /// the speed changes the pwm calculation
    /// 
    /// the calculation looks like this:
    /// 
    /// i = pwm_off_value
    pub fn change_speed(&mut self, servo_motor_controller: &mut crate::devices::external::servomotorcontroller::ServoMotorController,new_speed:u16) {
        self.speed = new_speed;
        self.write_to_motor_chip(servo_motor_controller);
    }

    /// Same as 
    /// ```rust
    /// &self.change_state(servo_motor_controller,MotorState::Brake);
    /// ```
    pub fn brake(&mut self, servo_motor_controller: &mut crate::devices::external::servomotorcontroller::ServoMotorController) {
        self.state = MotorState::Brake;
        self.write_to_motor_chip(servo_motor_controller);
    }

    /// Same as 
    /// ```rust
    /// &self.change_state(servo_motor_controller,MotorState::Forward);
    /// ```
    pub fn forward(&mut self, servo_motor_controller: &mut crate::devices::external::servomotorcontroller::ServoMotorController) {
        self.state = MotorState::Forward;
        self.write_to_motor_chip(servo_motor_controller);
    }
    


    fn write_to_motor_chip(&self, servo_motor_controller: &mut crate::devices::external::servomotorcontroller::ServoMotorController) {
        

        match self.state {
            MotorState::Coast => {
                servo_motor_controller.controller.set_channel_full_off(self.channel.forward_port).unwrap();
                servo_motor_controller.controller.set_channel_full_off(self.channel.reverse_port).unwrap();
                
            },
            MotorState::Reverse => {
                servo_motor_controller.controller.set_channel_full_off(self.channel.forward_port).unwrap();
                servo_motor_controller.controller.set_channel_on_off(self.channel.reverse_port, 0, self.speed).unwrap();
            },
            MotorState::Forward => {
                servo_motor_controller.controller.set_channel_on_off(self.channel.forward_port, 0,self.speed).unwrap();
                servo_motor_controller.controller.set_channel_full_off(self.channel.reverse_port).unwrap();
            },
            MotorState::Brake => {
                servo_motor_controller.controller.set_channel_on_off(self.channel.forward_port, 0,self.speed).unwrap();
                servo_motor_controller.controller.set_channel_on_off(self.channel.reverse_port, 0,self.speed).unwrap();
                
            },
        }
    }

    fn write_to_motor_chip_logging(&self, servo_motor_controller: &mut crate::devices::external::servomotorcontroller::ServoMotorController,serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>) {
        

        match self.state {
            MotorState::Coast => {
                servo_motor_controller.controller.set_channel_full_off(self.channel.forward_port).unwrap();
                servo_motor_controller.controller.set_channel_full_off(self.channel.reverse_port).unwrap();
                
            },
            MotorState::Reverse => {
                servo_motor_controller.controller.set_channel_full_off(self.channel.forward_port).unwrap();
                servo_motor_controller.controller.set_channel_on_off(self.channel.reverse_port, 0, self.speed).unwrap();
            },
            MotorState::Forward => {
                servo_motor_controller.controller.set_channel_on_off(self.channel.forward_port, 0,self.speed).unwrap();
                servo_motor_controller.controller.set_channel_full_off(self.channel.reverse_port).unwrap();
            },
            MotorState::Brake => {
                match servo_motor_controller.controller.set_channel_on_off(self.channel.forward_port, 0,self.speed)  {
                    Ok(_) => {},
                    Err(e) => {
                        write!(serial,"{e:?}").unwrap();
                    },
                };
                match servo_motor_controller.controller.set_channel_on_off(self.channel.reverse_port, 0,self.speed) {
                    Ok(_) => {},
                    Err(e) => {
                        write!(serial,"{e:?}").unwrap();
                    },
                }
                
            },
        }
    }
}