
use pwm_pca9685::Channel;

pub enum MicrotbitDriverPorts {
    // servo ports according to the documentation
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,

    // motor ports according to le documentation
    /// This device works by changing the state of the motor depending on which combination of ports are enabled.
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
    /// Read the fucking manual for more info: https://dfimg.dfrobot.com/wiki/17542/DFR0548_gravity-hr8833-motor-and-servo-driver-expansion-board_schematics_v1.zip
    M1,
    /// This device works by changing the state of the motor depending on which combination of ports are enabled.
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
    /// Read the fucking manual for more info: https://dfimg.dfrobot.com/wiki/17542/DFR0548_gravity-hr8833-motor-and-servo-driver-expansion-board_schematics_v1.zip
    M2,
    /// This device works by changing the state of the motor depending on which combination of ports are enabled.
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
    /// Read the fucking manual for more info: https://dfimg.dfrobot.com/wiki/17542/DFR0548_gravity-hr8833-motor-and-servo-driver-expansion-board_schematics_v1.zip
    M3,
    /// This device works by changing the state of the motor depending on which combination of ports are enabled.
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
    /// Read the fucking manual for more info: https://dfimg.dfrobot.com/wiki/17542/DFR0548_gravity-hr8833-motor-and-servo-driver-expansion-board_schematics_v1.zip
    M4,

    // I'm documentation-sexual
}


impl MicrotbitDriverPorts {
    /// Get the ports for S1 through S8
    /// 
    /// Will panic if given a Motor port (M1 - M4)
    pub fn servo_get(self) -> Channel {

        match self {
            Self::S1 => Channel::C15,
            Self::S2=> Channel::C14,
            Self::S3=> Channel::C13,
            Self::S4=> Channel::C12,
            Self::S5=> Channel::C11,
            Self::S6=> Channel::C10,
            Self::S7=> Channel::C9,
            Self::S8=> Channel::C8,
            _ => panic!("invalid channel.")
        }
    }


    /// Get the ports for M1 through M4
    /// 
    /// Will panic if given a Servo Port (S1 - S8)
    pub fn motor_get(self) -> MotorPort {
        match self {
            Self::M1 =>  MotorPort { forward_port: Channel::C7, reverse_port: Channel::C6},
            Self::M2 => MotorPort { forward_port: Channel::C5, reverse_port: Channel::C4 },
            Self::M3 =>  MotorPort { forward_port: Channel::C3, reverse_port: Channel::C2},
            Self::M4 =>  MotorPort { forward_port: Channel::C1, reverse_port: Channel::C0},
            _ => {
                panic!("invalid channel.")
            }
        }
    }

}


/// This device works by changing the state of the motor depending on which combination of ports are enabled.
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
/// Read the fucking manual for more info: https://dfimg.dfrobot.com/wiki/17542/DFR0548_gravity-hr8833-motor-and-servo-driver-expansion-board_schematics_v1.zip
pub struct MotorPort {
    pub forward_port: Channel,
    pub reverse_port: Channel
}


pub enum MicrobitDriverIOPorts {
    Pin0,
    Pin1,
    Pin2,
    Pin8,
    Pin12,
    Pin13,
    Pin14,
    Pin15,
    Pin16
}