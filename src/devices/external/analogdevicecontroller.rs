use microbit::hal::Saadc;
use crate::utils::serial::Serial;




/// Controller for external Analog Devices
pub struct AnalogDeviceController
{
    pub controller:Saadc
}

impl AnalogDeviceController {



    /// Create a new Analog Device Controller.
    /// 
    /// This requires the on board SAADC peripheral which can be gotten with 
    /// ```rust
    /// let saadc_config = SaadcConfig::default();
    /// let mut saadc = Saadc::new(board.ADC, saadc_config);
    /// ```
    pub fn new(
        saadc:Saadc,
        serial: &mut crate::utils::serial::UartePort<microbit::pac::UARTE0>,
    ) -> AnalogDeviceController {
        Serial::write(serial, "Starting Analog Device Controller", crate::utils::serial::MessageSeverity::INFORMATIVE);

        return Self {
            controller: saadc,
        }
    }


    /// Create a new Analog Device Controller.
    /// 
    /// This requires the on board SAADC peripheral which can be gotten with 
    /// ```rust
    /// let saadc_config = SaadcConfig::default();
    /// let mut saadc = Saadc::new(board.ADC, saadc_config);
    /// ```
    pub fn new_nolog(
        saadc:Saadc,
    ) -> AnalogDeviceController {
     
        return Self {
            controller: saadc,
        }
    }
}