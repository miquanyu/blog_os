use uart_16550::SerialPort;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub fn print(arg: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(arg).expect("Printing to serial failed")
}

#[macro_export]
macro_rules! print_serial {
    ($($arg:tt)*) => ($crate::serial::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println_serial {
    () => (print_serial!("\n"));
    ($arg:expr) => (print_serial!(concat!($arg, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print_serial!(concat!($fmt, "\n"), $($arg)*));
}