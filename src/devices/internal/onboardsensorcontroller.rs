use lsm303agr::{AccelOutputDataRate, Lsm303agr};
use microbit::{hal::{Timer, Twim}, pac::{TIMER0, TWIM0}};

use crate::utils::serial::Serial;



pub struct OnboardSensorController {
    pub controller:Lsm303agr<lsm303agr::interface::I2cInterface<Twim<TWIM0>>, lsm303agr::mode::MagOneShot>
}


impl OnboardSensorController {

    /// (if you're confused just call .into on the on board_internal_i2c_pins)
    pub fn new(
        board_twim0:microbit::pac::TWIM0,
        i2c_internal_pins:microbit::hal::twim::Pins,
        serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>,
        timer: &mut Timer<TIMER0>
    ) -> Self {
        Serial::write(serial, "Initiating Magnometer/Accelerometer", crate::utils::serial::MessageSeverity::INFORMATIVE);

        let i2c_twim = microbit::hal::Twim::new(
            board_twim0,
            i2c_internal_pins.into(),
            microbit::hal::twim::Frequency::K100
        );

        

        let mut sensor: Lsm303agr<lsm303agr::interface::I2cInterface<Twim<TWIM0>>, lsm303agr::mode::MagOneShot> = Lsm303agr::new_with_i2c(i2c_twim);
        sensor.init().unwrap();
        
        sensor.set_accel_mode_and_odr(timer, lsm303agr::AccelMode::Normal, AccelOutputDataRate::Hz50 ).unwrap();
        sensor.set_mag_mode_and_odr(timer, lsm303agr::MagMode::HighResolution, lsm303agr::MagOutputDataRate::Hz50).unwrap();


        Serial::write(serial,"Successfully Started Magnometer/Accelerometer", crate::utils::serial::MessageSeverity::OK);

        Self { controller: sensor }
    }


    #[inline]
    /// frees the underlying TWIM interface for later use 
    fn free(self) -> (microbit::pac::TWIM0,microbit::hal::twim::Pins) {
        self.controller.destroy().free()
    }

    
    /// this one is a weird one, it exists mainly due to rusts ownership rules.
    /// 
    /// This first frees the pins up from the MagnometerAccelerometerController instance (meaning that you 
    /// wont be able to use the Magnometer/Accelerometer until you use ServoMotorController::into_magnometer_accelerometer)
    /// 
    /// Then it uses those pins to create a new ServoMotorController instance.
    /// 
    /// Finally, it returns a tuple that contains the (old i2c external pins, new Servo Motor Controller)
    pub fn into_servo_motor_controller(self,i2c_external_pins:microbit::hal::twim::Pins,serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>) -> (microbit::hal::twim::Pins,crate::devices::external::ServoMotorController) {
        let (twim,pins) = self.free();

        let x = crate::devices::external::ServoMotorController::new(
            twim,
            i2c_external_pins,
            serial
        );


        return (pins,x);
    }
}