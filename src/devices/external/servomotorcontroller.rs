use microbit::hal::twim::Frequency;
use microbit::pac::TIMER0;
use pwm_pca9685::{Address, Pca9685};
use microbit::hal::{Timer, Twim};




/// Controller for External Motors 
pub struct ServoMotorController {
    pub(crate) controller: Pca9685<Twim<microbit::pac::TWIM0>>
}



impl ServoMotorController {


    /// takes the microbits I2C interface in exchange for the Motors 
    /// 
    /// (if you're confused on how to get the board external pins just call .into on the on board_external_i2c_pins)
    /// 
    /// Returns the motor controller struct 
    pub fn new(
        board_twim0:microbit::pac::TWIM0,
        board_external_i2c_pins:microbit::hal::twim::Pins,
        serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>,
    )-> ServoMotorController
    {
        crate::utils::serial::Serial::write(serial, "Starting Servo Driver", crate::utils::serial::MessageSeverity::INFORMATIVE);

        let address = Address::default();

        let i2c_twim = microbit::hal::Twim::new(
            board_twim0,
            board_external_i2c_pins.into(),
            Frequency::K250
        );     

        let mut pwm: Pca9685<Twim<microbit::pac::TWIM0>> = match pwm_pca9685::Pca9685::new(i2c_twim, address) {
            Ok(x) => x,
            Err(e) => {
                // let _ = write!(serial,"unable to access Servo controller: \r\n \r\n Error Code:  {e:?} \r\n");
                match e {
                    pwm_pca9685::Error::I2C(_) => {
                        crate::utils::serial::Serial::write(serial, "Unable To Start Servo Driver: I2c Error", crate::utils::serial::MessageSeverity::Error);
                    },
                    pwm_pca9685::Error::InvalidInputData => {
                        crate::utils::serial::Serial::write(serial, "Unable to start Servo Driver: Invalid Input Data", crate::utils::serial::MessageSeverity::Error);
                    },
                }
                panic!();
            },
        };
        
        // let _ = write!(serial,"setting prescale \r\n");


    // prescale_value = round(osc_value / (4096 * update_rate)) - 1
    match pwm.set_prescale(100) {
            Ok(_) => {
                // do nothing
            },
            Err(_e) => {
                crate::utils::serial::Serial::write(serial, "Unable to set prescale.", crate::utils::serial::MessageSeverity::Error);
            },
        }

        match pwm.enable() {
            Ok(_) => {
                crate::utils::serial::Serial::write(serial, "Successfully started Servo Driver", crate::utils::serial::MessageSeverity::OK);
                return ServoMotorController {
                    controller: pwm 
                }
            },
            Err(_e) => {
                    // let _ = write!(serial,"unable to enable pwm device: \n \n Erorr Code:  {e:?} \r\n");
                    crate::utils::serial::Serial::write(serial,"unable to enable Servo PWM Device. Aborting.", crate::utils::serial::MessageSeverity::Error);
                    panic!()
            },
        };
        // pwm.set_channel_on(pwm_pca9685::Channel::All, 0).unwrap();
        // pwm.set_channel_off(pwm_pca9685::Channel::All, 2047).unwrap();

        // return Servos { turning_motor, acceleration_motor }


    }

    /// takes the microbits I2C interface in exchange for the Motors 
    /// 
    /// (if you're confused on how to get the board external pins just call .into on the on board_external_i2c_pins)
    /// 
    /// Returns the motor controller struct 
    pub fn new_nolog(
        board_twim0:microbit::pac::TWIM0,
        board_external_i2c_pins:microbit::hal::twim::Pins,
    )-> ServoMotorController
    {
        let address = Address::default();

        let i2c_twim = microbit::hal::Twim::new(
            board_twim0,
            board_external_i2c_pins.into(),
            Frequency::K250
        );     

        let mut pwm: Pca9685<Twim<microbit::pac::TWIM0>> = match pwm_pca9685::Pca9685::new(i2c_twim, address) {
            Ok(x) => x,
            Err(e) => {
                // let _ = write!(serial,"unable to access Servo controller: \r\n \r\n Error Code:  {e:?} \r\n");
                match e {
                    pwm_pca9685::Error::I2C(_) => {
                        
                    },
                    pwm_pca9685::Error::InvalidInputData => {
                        
                    },
                }
                panic!();
            },
        };
        
        // let _ = write!(serial,"setting prescale \r\n");


    // prescale_value = round(osc_value / (4096 * update_rate)) - 1
    match pwm.set_prescale(100) {
            Ok(_) => {
                // do nothing
            },
            Err(_e) => {
                
            },
        }

        match pwm.enable() {
            Ok(_) => {
                
                return ServoMotorController {
                    controller: pwm 
                }
            },
            Err(_e) => {
                    // let _ = write!(serial,"unable to enable pwm device: \n \n Erorr Code:  {e:?} \r\n");
                    
                    panic!()
            },
        };
        // pwm.set_channel_on(pwm_pca9685::Channel::All, 0).unwrap();
        // pwm.set_channel_off(pwm_pca9685::Channel::All, 2047).unwrap();

        // return Servos { turning_motor, acceleration_motor }


    }

    #[inline]
    /// frees the underlying TWIM interface for later use 
    fn free(self) -> (microbit::pac::TWIM0,microbit::hal::twim::Pins){
        self.controller.destroy().free()
    }
    
    /// this one is a weird one, it exists mainly due to rusts ownership rules.
    /// 
    /// This first frees the pins up from the servoController instance (meaning that you 
    /// wont be able to use the servocontroller until you use MagnometerAccelerometerController::into_servomotor)
    /// 
    /// Then it uses those pins to create a new MagnometerAccelerometerController instancec.
    /// 
    /// Finally, it returns a tuple that contains the (old i2c external pins, new Magnometer)
    pub fn into_magnometer_accelerometer(self, i2c_internal_pins:microbit::hal::twim::Pins,serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>,timer: &mut Timer<TIMER0>) -> (microbit::hal::twim::Pins,crate::devices::internal::OnboardSensorController) {
        let (twim,pins) = self.free();

        crate::utils::serial::Serial::write(serial, "Switching TWIM priority to Magnometer/Accelerometer", crate::utils::serial::MessageSeverity::INFORMATIVE);

        let x =  crate::devices::internal::OnboardSensorController::new(
            twim, 
            i2c_internal_pins, 
            serial,
            timer, 
        );


        return (microbit::hal::twim::Pins { scl: pins.scl, sda: pins.sda },x)

    }
    

}