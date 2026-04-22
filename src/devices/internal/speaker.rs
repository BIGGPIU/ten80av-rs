use embedded_hal::{delay::DelayNs, digital::{OutputPin, StatefulOutputPin}};
use microbit::{hal::Timer, pac::TIMER0};
use micromath::F32Ext;

/// Interface for Speakers/Buzzers, Compatible with the speaker onboard the Micro:Bit
pub struct Speaker {
    speaker_pin:microbit::hal::gpio::Pin<microbit::hal::gpio::Output<microbit::hal::gpio::PushPull>>,
    
    /// should this speaker even be on in the first place
    pub(crate) on_flag:bool,
    /// what is the maximum amount of time you want time_until_switch_us
    /// to be set to?  
    pub(crate) current_max_time_on:u32,
    /// constantly mutating time saying how much time you have left until you toggle the switch
    pub(crate) time_until_switch_us:u32,

}


impl Speaker {



    pub fn new(
        speaker_pin:microbit::hal::gpio::Pin<microbit::hal::gpio::Output<microbit::hal::gpio::PushPull>>
    ) -> Speaker {
        return Speaker { speaker_pin, on_flag: false, current_max_time_on: 0, time_until_switch_us: 0  }
    }


    // useful resource I think: https://www.phys.unsw.edu.au/jw/notes.html
    pub fn play_frequency(&mut self, timer: &mut Timer<TIMER0>,frequency_hz:f64, duration_us:u32) {
        let period_ms:f64 = (1.0/frequency_hz)*1000.0;

        let microsecond_period = period_ms * 1000.0;

        let u32microsecond_period = microsecond_period as u32;

        let mut time_elapsed_us = 0;
        while duration_us > time_elapsed_us {
            self.speaker_pin.set_high();
            timer.delay_us(u32microsecond_period);
            self.speaker_pin.set_low();
            timer.delay_us(u32microsecond_period);
            time_elapsed_us += u32microsecond_period;
        }
    }


    pub fn play_midi_note(&mut self, timer: &mut Timer<TIMER0>,midi_note:u8,duration_us:u32) {
        
        let tuning = 440.0/32.0;
        
        let note_eq = ((midi_note as f32)-9.0)/12.0;

        let frequency = tuning * (2_f32.powf(note_eq)) as f64;

        let period_ms:f64 = (1.0/frequency)*1000.0;

        let microsecond_period = period_ms * 1000.0;

        let u32microsecond_period = microsecond_period as u32;

        let mut time_elapsed_us = 0;
        while duration_us > time_elapsed_us {
            self.speaker_pin.set_high();
            timer.delay_us(u32microsecond_period);
            self.speaker_pin.set_low();
            timer.delay_us(u32microsecond_period);
            time_elapsed_us += u32microsecond_period;
        }
    }

    pub (crate) fn inverse_speaker(&mut self) {
        self.speaker_pin.toggle();
    }

    pub (crate) fn disable_speaker(&mut self) {
        self.speaker_pin.set_low();
    }
}


