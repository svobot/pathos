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

    loop {
        if let Some(c) = uart.get() {
            match c {
                127 | 8 => {
                    print!("{}{}{}", 8 as char, ' ', 8 as char);
                }
                10 | 13 => {
                    println!();
                }
                27 => {
                    if let Some(91) = uart.get() {
                        if let Some(next_c) = uart.get() {
                            match next_c as char {
                                'A' => {
                                    println!("That's the up arrow!");
                                }
                                'B' => {
                                    println!("That's the down arrow!");
                                }
                                'C' => {
                                    println!("That's the right arrow!");
                                }
                                'D' => {
                                    println!("That's the left arrow!");
                                }
                                _ => {
                                    println!("That's something else.....");
                                }
                            }
                        }
                    }
                }
                _ => {
                    print!("{}", c as char);
                }
            }
        }
    }
}
