use crate::devices::internal::onboardsensorcontroller::OnboardSensorController;

/// Interface for the Magnometer Onboard the Micro:Bit V2
pub struct Magnometer {

}


impl Magnometer {



    /// Read the raw input from the onboard Magnometer
    pub fn read_magnometer(controller:&mut  OnboardSensorController) -> Option<(i16, i16, i16)> {
        match controller.controller.magnetic_field() {
            Ok(x) => {
                return Some(x.xyz_unscaled());
            },
            Err(_) => {
                return None
            },
        }
    }
}