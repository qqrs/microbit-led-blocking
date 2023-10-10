#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::asm;
use microbit::{
    Board,
    hal::{
        prelude::*,
        Timer,
    }
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

    board.CLOCK.tasks_hfclkstart.write(|w| w.tasks_hfclkstart().trigger());

    for _ in 0..(1_600_000/4) {
        asm::nop()
    }

    let stat = board.CLOCK.hfclkstat.read().bits();
    rprintln!("{:032b}", stat);

    //let mut timer = Timer::new(board.TIMER1);
    let mut ticker = Timer::periodic(board.TIMER2);
    ticker.start(100_000u32);

    toggle(&mut board.display_pins.row1);
    toggle(&mut board.display_pins.col1);
    
    loop {
        //for _ in 0..(1_600_000/4) {
            //asm::nop()
        //}

        //timer.start(100_000u32);
        //nb::block!(timer.wait());

        nb::block!(ticker.wait());

        //rprintln!("toggle");
        toggle(&mut board.display_pins.col1);
    }
}


fn toggle(pin: &mut dyn StatefulOutputPin<Error = Void>) {
    if pin.is_set_high().void_unwrap() {
        pin.set_low().void_unwrap();
    } else {
        pin.set_high().void_unwrap();
    }
}
