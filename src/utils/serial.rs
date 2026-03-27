use core::ptr::addr_of_mut;
use core::fmt;
use microbit::hal::uarte::{Error, Instance, Uarte, UarteRx, UarteTx};
use core::fmt::Write;

static mut TX_BUF:[u8;1] = [0;1];
static mut RX_BUF:[u8;1] = [0;1];

pub struct UartePort<T:Instance>(UarteTx<T>,UarteRx<T>);

impl<T:Instance> UartePort<T> {
    /// Initiate logging
    pub fn new(serial:Uarte<T>) -> UartePort<T> {
        
        // this is kinda like a broadcast
        let (tx,rx) = serial
            .split(
                unsafe {
                    addr_of_mut!(TX_BUF).as_mut().unwrap()
                },
                unsafe {
                    addr_of_mut!(RX_BUF).as_mut().unwrap()
            }).unwrap();
        
        UartePort(tx, rx)
        
    }
} 


impl<T: Instance> fmt::Write for UartePort<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}

impl<T: Instance> embedded_io::ErrorType for UartePort<T> {
    type Error = Error;
}

impl<T: Instance> embedded_io::Write for UartePort<T> {
    /// write to the stream
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> {
        self.0.write(buffer)
    }

    /// flush the stream
    fn flush(&mut self) -> Result<(), Self::Error> {
        self.0.flush()
    }
}



pub struct Serial {

}


pub enum MessageSeverity {
    /// for when you're writing to the console just to write to the console
    INFORMATIVE,
    /// for when you're writing to the console to say something went well
    OK,
    /// for when you're writing to the console to say something kinda messed up
    Warning,
    /// for hwen you're writing to the console to say something is really messed up
    Error
}

impl Serial {
    /// Write a message to the Serial monitor
    pub fn write(serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>,message:&str,severity:MessageSeverity) {
        match severity {
            MessageSeverity::OK => {
                write!(serial, "[OK] {message:?} \r\n").unwrap();
            },
            MessageSeverity::Warning => {
                write!(serial, "[WARN] {message:?}\r\n").unwrap();
            },
            MessageSeverity::Error => {
                write!(serial, "[ERROR] {message:?}\r\n").unwrap();
            },
            MessageSeverity::INFORMATIVE => {
                write!(serial, "[INFO] {message:?}\r\n").unwrap();
            },
        }
    }

    /// clears the terminal by sending a ton of newlines
    pub fn clear_terminal(serial:&mut crate::utils::serial::UartePort<microbit::pac::UARTE0>) {
        write!(serial,"\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n").unwrap();
    }
}