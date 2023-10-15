#![no_std]
#![no_main]

use core::cell::RefCell;

use cortex_m::{asm, interrupt::Mutex, peripheral::NVIC};
use cortex_m_rt::entry;
use microbit::{
    hal::{prelude::*, timer::{Periodic, Instance}, Clocks, Timer},
    pac::{interrupt, Interrupt, TIMER0},
    Board, Peripherals,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use void::{ResultVoidExt, Void};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("hi");

    let mut board = Board::take().unwrap();

    let stat = board.CLOCK.hfclkstat.read().bits();
    rprintln!("{:032b}", stat);

    let clocks = Clocks::new(board.CLOCK);
    clocks.enable_ext_hfosc();

    let clock_periph = unsafe { Peripherals::steal() }.CLOCK;
    let stat = clock_periph.hfclkstat.read().bits();
    rprintln!("{:032b}", stat);

    toggle(&mut board.display_pins.row1);
    toggle(&mut board.display_pins.col1);

    let mut ticker = Timer::periodic(board.TIMER0);
    ticker.start(100_000u32);
    ticker.enable_interrupt();
    unsafe {
        NVIC::unmask(Interrupt::TIMER0);
    }

    loop {
        // nb::block!(ticker.wait()).unwrap();

        // rprintln!("toggle");
        // toggle(&mut board.display_pins.col1);
    }
}

#[interrupt]
fn TIMER0() {
    rprintln!("timer 0 fired");

    toggle(&mut board.display_pins.col1);

    // clear the event register
    cortex_m::interrupt::free(|_cs| {
        unsafe { Peripherals::steal() }.TIMER0.timer_reset_event();
    });
}

fn toggle(pin: &mut dyn StatefulOutputPin<Error = Void>) {
    if pin.is_set_high().void_unwrap() {
        pin.set_low().void_unwrap();
    } else {
        pin.set_high().void_unwrap();
    }
}
