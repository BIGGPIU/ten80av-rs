//! external and internal devices on the Micro:Bit and the Driver Expansion Board.
//! 
//! # Overview
//! 
//! * [`internal`] has internal sensors like: [`internal::radio`], [`internal::Magnometer`]
//! * [`external`] has external sensors and motors, like your [`external::TurningMotor`], [`external::AccelerationMotor`], [`external::UltraSonicDistanceSensor`], and your [`external::IRSensor`]

pub mod external;
pub mod internal;
