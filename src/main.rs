#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::asm;
use microbit::{
    Board,
    hal::prelude::*,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use void::{ResultVoidExt, Void};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("hi");

    let mut board = Board::take().unwrap();

    toggle(&mut board.display_pins.row1);
    toggle(&mut board.display_pins.col1);
    
    loop {
        for _ in 0..16_000_000 {
            asm::nop()
        }
        rprintln!("toggle");
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
