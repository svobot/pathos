// pathos
#![no_main]
#![no_std]
#![feature(panic_info_message, global_asm, asm)]

pub mod assembly;
pub mod uart;

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
    });
}

#[macro_export]
macro_rules! println
{
    () => ({
        print!("\r\n")
    });
    ($fmt:expr) => ({
        print!(concat!($fmt, "\r\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        print!(concat!($fmt, "\r\n"), $($args)+)
    });
}

#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Aborting :");
    if let Some(p) = info.location() {
        println!(
            "line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    } else {
        println!("no information available.");
    }
    abort();
}

#[no_mangle]
fn abort() -> ! {
    loop {
        unsafe {
            asm!("wfi", options(nomem, nostack));
        }
    }
}

#[no_mangle]
extern "C" fn kmain() {
    // Main should initialize all sub-systems and get ready to start
    // scheduling. The last thing this should do is to start the timer.
    let uart = uart::Uart::new(0x1000_0000);
    uart.init();

    println!("This is my operating system!");
    println!("I'm so awesome. If you start typing something, I'll show you what you typed!");
}
