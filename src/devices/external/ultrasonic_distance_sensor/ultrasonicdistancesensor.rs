


use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin}};
use microbit::{hal::{ Timer, gpio::{Input, Output, PullUp, PushPull}}, pac::{TIMER0}};
use core::fmt::Write;

use crate::utils::serial::Serial;

pub enum UltrasonicDistanceSensorError {
    Timeout,
    Unknown(&'static str),
}

/// Interface for an hcsr04 Ultrasonic Distance Sensor 
pub struct UltraSonicDistanceSensor {   
    echo_pin:microbit::hal::gpio::Pin<Input<PullUp>>,
    trigger_pin:microbit::hal::gpio::Pin<Output<PushPull>>,
    max_timeout:u32,
}



impl UltraSonicDistanceSensor {
    /// creates a new SensorController
    /// 
    /// The blocking_action_timeout argument specifies how long the controller should wait in microseconds
    /// before giving up on reading/writing data. This is to prevent random freezes
    pub fn new(
        echo_pin:microbit::hal::gpio::Pin<Input<PullUp>>,
        trigger_pin:microbit::hal::gpio::Pin<Output<PushPull>>,
        max_timeout:u32,
        serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>,
    ) -> UltraSonicDistanceSensor {

        Serial::write(serial,"Starting UltraSonic Distance Sensor", crate::utils::serial::MessageSeverity::INFORMATIVE);

        

        let x =  UltraSonicDistanceSensor {
            echo_pin,
            trigger_pin,
            max_timeout
        };

        return x;
    }

    /// creates a new SensorController
    /// 
    /// The blocking_action_timeout argument specifies how long the controller should wait in microseconds
    /// before giving up on reading/writing data. This is to prevent random freezes
    pub fn new_nolog(
        echo_pin:microbit::hal::gpio::Pin<Input<PullUp>>,
        trigger_pin:microbit::hal::gpio::Pin<Output<PushPull>>,
        max_timeout:u32,
    ) -> UltraSonicDistanceSensor {
        let x =  UltraSonicDistanceSensor {
            echo_pin,
            trigger_pin,
            max_timeout
        };

        return x;
    }    



    pub fn measure_raw(&mut self, timer:&mut Timer<TIMER0>) -> Result<u32,UltrasonicDistanceSensorError> {
        self.send_trigger_pulse(timer);

        let _echo_start = match self.wait_for_echo_start(timer) {
            Ok(x) => x,
            Err(_) => {
                // write!(serial,"error: {e:?} \r\n").unwrap();
                return Err(UltrasonicDistanceSensorError::Timeout)
            },
        };

        let pulse_width = match self.wait_for_echo_end(timer) {
            Ok(x) => x,
            Err(e) => { 
                return Err(UltrasonicDistanceSensorError::Timeout)
            },
        };

        return Ok(pulse_width)

    }

    pub fn measure(&mut self, timer:&mut Timer<TIMER0>) -> Result<f32,UltrasonicDistanceSensorError> {
        self.send_trigger_pulse(timer);

        let _echo_start = match self.wait_for_echo_start(timer) {
            Ok(x) => x,
            Err(e) => {
                return Err(UltrasonicDistanceSensorError::Timeout)
            },
        };

        let pulse_width = match self.wait_for_echo_end(timer) {
            Ok(x) => x,
            Err(_) => { 
                return Err(UltrasonicDistanceSensorError::Timeout)
            },
        };

        return Ok(self.pulse_width_to_cm(pulse_width))

    }

    /// referenced from: https://docs.rs/crate/hcsr04/0.1.3/source/src/hcsr04.rs
    fn send_trigger_pulse(&mut self,timer:&mut Timer<TIMER0>) {
        self.trigger_pin.set_low().unwrap();

        timer.delay_us(2);

        self.trigger_pin.set_high().unwrap();

        timer.delay_us(10);

        self.trigger_pin.set_low().unwrap();
    }

    /// referenced from: https://docs.rs/crate/hcsr04/0.1.3/source/src/hcsr04.rs
    fn wait_for_echo_start(&mut self, timer:&mut Timer<TIMER0>) -> Result<u32,UltrasonicDistanceSensorError> {
        let mut elapsed = 0_u32;

        while self.echo_pin.is_low().unwrap() {
            if elapsed >= self.max_timeout {
                return Err(UltrasonicDistanceSensorError::Timeout);
            }

            timer.delay_us(1);
            elapsed+=1;
        }


        return Ok(elapsed)
    }

    /// referenced from: https://docs.rs/crate/hcsr04/0.1.3/source/src/hcsr04.rs
    fn wait_for_echo_end(&mut self, timer:&mut Timer<TIMER0>) -> Result<u32,UltrasonicDistanceSensorError> {
        let mut elapsed = 0u32;

        while self.echo_pin.is_high().unwrap() {
            if elapsed >= self.max_timeout {
                return Err(UltrasonicDistanceSensorError::Timeout)
            }

            timer.delay_us(1);
            elapsed+=1;
        }

        Ok(elapsed)
    }

    // average room temperature: 70F
    // average room temperature: 21C

    fn pulse_width_to_cm(&self, pulse_width_us:u32) -> f32 {
        let speed_mps = 331.3 * (1.0 + 0.00183 * 21 as f32);
        let speed_cm_per_second = speed_mps/10000.0;

        let distance_cm = (pulse_width_us as f32 * speed_cm_per_second) / 2.0;

        return distance_cm

    }

}