#![feature(lang_items)]
#![no_std]
#![no_main]
#![allow(dead_code)]

use core::prelude::*;
use embedded_hal::prelude::*;
use stm32f1xx_hal::prelude::*;

use embedded_hal::digital::v2::{InputPin, OutputPin};

use cortex_m::peripheral::DWT;

use rtic::app;
use rtic::cyccnt::{Instant, U32Ext as _};

use stm32f1xx_hal::gpio;
use stm32f1xx_hal::pac;

use core::convert::{Into};

const HSE_CLOCK_MHZ: u32 = 8;
const SYS_CLOCK_MHZ: u32 = 72;

const HSE_CLOCK_HZ: u32 = HSE_CLOCK_MHZ * 1_000_000;
const SYS_CLOCK_HZ: u32 = SYS_CLOCK_MHZ * 1_000_000;

#[app(device=stm32f1::stm32f103, peripherals = true, monotonic=rtic::cyccnt::CYCCNT)]
const APP: () = {

    struct Resources {
    }
    
    #[init(schedule=[])]
    fn init(cx: init::Context) {
        let mut peripherals = cx.core;
        let device: stm32f1xx_hal::stm32::Peripherals = cx.device;
        
        let mut flash = device.FLASH.constrain();
        let mut rcc = device.RCC.constrain();
        let mut afio = device.AFIO.constrain(&mut rcc.apb2);
        
        let clocks = rcc.cfgr.use_hse(HSE_CLOCK_MHZ.mhz())
            .sysclk(SYS_CLOCK_MHZ.mhz())
            .hclk(SYS_CLOCK_MHZ.mhz())
            .pclk1((SYS_CLOCK_MHZ / 2).mhz())
            .pclk2(SYS_CLOCK_MHZ.mhz())
            .freeze(&mut flash.acr);

        let mut gpioa = device.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = device.GPIOB.split(&mut rcc.apb2);
        
        peripherals.DCB.enable_trace();
        DWT::unlock();
        peripherals.DWT.enable_cycle_counter();
    
        // let now = cx.start;
    }

    #[idle()]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[allow(non_snake_case)]
    extern "C" {
        fn USART2();

        fn SPI2();
        fn SPI3();
    }

};

#[panic_handler]
fn my_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
