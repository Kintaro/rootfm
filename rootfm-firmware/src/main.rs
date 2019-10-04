//#![deny(warnings)]
//#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_itm;

mod output;

use cortex_m_rt::entry;
use embedded_graphics::{fonts::Font6x8, pixelcolor::BinaryColor, prelude::*};
use numtoa::NumToA;
use rootfm_core::{Synthesizer, PRESET_1};
use ssd1306::{mode::GraphicsMode, Builder};
use stm32h7xx_hal::hal::digital::v2::OutputPin;
use stm32h7xx_hal::{pac, prelude::*};
use xca9548a::{SlaveAddr, TCA9548A};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let vos = pwr.freeze();

    // Constrain and Freeze clock
    dp.RCC.apb1lenr.modify(|_, w| w.dac12en().enabled());
    let rcc = dp.RCC.constrain();
    //rcc.apb1lenr.modify(|w| w.dac12en().enabled());
    let mut ccdr = rcc
        .sys_ck(400.mhz())
        .per_ck(64.mhz())
        .freeze(vos, &dp.SYSCFG);

    let gpioa = dp.GPIOA.split(&mut ccdr.ahb4);
    let gpiof = dp.GPIOF.split(&mut ccdr.ahb4);
    gpioa.pa4.into_analog();
    gpioa.pa5.into_analog();

    let mut synthesizer = Synthesizer::new(PRESET_1);

    let i2c = {
        let scl = gpiof.pf1.into_alternate_af4().set_open_drain();
        let sda = gpiof.pf0.into_alternate_af4().set_open_drain();
        dp.I2C2.i2c((scl, sda), 1.mhz(), &ccdr)
    };
    let manager = shared_bus::CortexMBusManager::new(i2c);

    let mut buffer = [0.0; 3 * 128];

    let address = SlaveAddr::default();
    let mut i2c_switch = TCA9548A::new(manager.acquire(), address.clone());

    i2c_switch.select_channels(0b0000_0100).unwrap();
    let mut disp1: GraphicsMode<_> = { Builder::new().connect_i2c(manager.acquire()).into() };
    disp1.init().unwrap();

    i2c_switch.select_channels(0b0000_1000).unwrap();
    let mut disp2: GraphicsMode<_> = Builder::new().connect_i2c(manager.acquire()).into();
    disp2.init().unwrap();

    i2c_switch.select_channels(0b0001_0000).unwrap();
    let mut disp3: GraphicsMode<_> = Builder::new().connect_i2c(manager.acquire()).into();
    disp3.init().unwrap();

    let mut displays = [disp1]; //[disp3, disp2, disp1];
    let num_displays = displays.len();

    let mut buffer_pos = 0;

    // enable DAC
    dp.DAC.cr.modify(|_, w| unsafe {
        w.en1()
            .set_bit()
            .ten1()
            .set_bit()
            .en2()
            .set_bit()
            .ten2()
            .set_bit()
    });
    dp.DAC
        .mcr
        .modify(|_, w| unsafe { w.mode1().bits(0).mode2().bits(0) });

    let mut delay = cp.SYST.delay(ccdr.clocks);

    let mut us_per_beat = 500000;
    let ticks_per_beat = 240;
    let mut us_per_tick = us_per_beat as f32 / ticks_per_beat as f32;
    let mut wait_us = 0.0; //us_per_tick * note.delta() as f32;
    let mut cycles = 0.0; //(wait_us * SAMPLE_RATE) / 1_000_000.0;
    let mut current_cycle = 0.0;
    let mut current_note = 0;

    synthesizer.note_on(67, 0.5);

    loop {
        while current_cycle > cycles {
            current_note = current_note % 1000;
            let next_note = (current_note + 1) % 1000;
            match output::NOTES[current_note] {
                (0, key, velocity, _) => synthesizer.note_on(key, velocity),
                (1, key, _, _) => synthesizer.note_off(key),
                (2, tempo, _, _) => {
                    us_per_beat = tempo;
                    us_per_tick = us_per_beat as f32 / ticks_per_beat as f32;
                }
                _ => continue,
            }
            wait_us = us_per_tick * output::NOTES[next_note].3 as f32;
            cycles = (wait_us * rootfm_core::SAMPLE_RATE as f32) / 1_000_000.0;
            current_cycle = 0.0;
            current_note += 1;
        }
        let f = synthesizer.compute();
        let f_int = (2048.0 + f * 4096.0) as u16;
        dp.DAC
            .dhr12rd
            .write(|w| unsafe { w.dacc1dhr().bits(f_int).dacc2dhr().bits(f_int) });
        dp.DAC
            .swtrgr
            .write(|w| w.swtrig1().set_bit().swtrig2().set_bit());
        //delay.delay_us(22_u16);
        current_cycle += 1.0;

        //for display in 2..2 + num_displays {
        //    let display_index = display - 2;
        //    i2c_switch.select_channels(1 << display).unwrap();
        //    displays[display_index].clear();
        //    displays[display_index].draw(
        //        Font6x8::render_str(synthesizer.active_voices().numtoa_str(10, &mut buf))
        //            .stroke(Some(BinaryColor::On))
        //            .into_iter(),
        //    );
        //    for i in 0..128 {
        //        let y = (37.0
        //            + buffer[(buffer_pos + (display_index * 128) + i) % (3 * 128)] * 118.0)
        //            as i32;
        //        displays[display_index].set_pixel(i as u32, 10, 1);
        //        displays[display_index].set_pixel(i as u32, 10 + y as u32, 1);
        //    }
        //    displays[display_index].flush().unwrap();
        //}
    }
}
