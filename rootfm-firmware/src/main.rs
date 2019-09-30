#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_itm;

use cortex_m_rt::entry;
use embedded_graphics::{fonts::Font6x8, pixelcolor::BinaryColor, prelude::*};
use numtoa::NumToA;
use rootfm_core::{Synthesizer, PRESET_1};
use ssd1306::{mode::GraphicsMode, Builder};
use stm32h7xx_hal::{pac, prelude::*};
use xca9548a::{SlaveAddr, TCA9548A};

#[entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let vos = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let mut ccdr = rcc.sys_ck(400.mhz()).freeze(vos, &dp.SYSCFG);

    let gpiof = dp.GPIOF.split(&mut ccdr.ahb4);

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

    let mut displays = [disp3, disp2, disp1];

    let mut buffer_pos = 0;
    let mut buf = [0u8; 20];

    synthesizer.note_on(67, 99.0);

    loop {
        let f = synthesizer.compute();
        buffer[buffer_pos] = f;

        for display in 2..5 {
            let display_index = display - 2;
            i2c_switch.select_channels(1 << display).unwrap();
            displays[display_index].clear();
            displays[display_index].draw(
                Font6x8::render_str(synthesizer.active_voices().numtoa_str(10, &mut buf))
                    .stroke(Some(BinaryColor::On))
                    .into_iter(),
            );
            for i in 0..128 {
                let y = (37.0
                    + buffer[(buffer_pos + (display_index * 128) + i) % (3 * 128)] * 118.0)
                    as i32;
                displays[display_index].set_pixel(i as u32, 10, 1);
                displays[display_index].set_pixel(i as u32, 10 + y as u32, 1);
            }
            displays[display_index].flush().unwrap();
        }
        buffer_pos = (buffer_pos + 1) % (3 * 128);
    }
}
