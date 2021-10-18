/* 
Test various functions of the Akafugu TWIDisplay
*/

#![no_main]
#![no_std]

use cortex_m;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32l4xx_hal::{
    delay::Delay,
    prelude::*,
    //serial::{Config, Serial},
    i2c::I2c,
    };

use akafugu_twidisplay::*;


#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);

    let mut led = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    
    let mut delay = Delay::new(cp.SYST, clocks);
    
    let mut scl = gpioa.pa9.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    
    scl.internal_pull_up(&mut gpioa.pupdr, true);
    let scl = scl.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut sda = gpioa.pa10.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    sda.internal_pull_up(&mut gpioa.pupdr, true);
    let sda = sda.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks, &mut rcc.apb1r1);

    let mut display = TWIDisplay::new(i2c, 0x12);

    display.clear_display().unwrap();
    
    //display.display_address().unwrap();

    display.set_brightness(255).unwrap();

    let mut num: u8 = 0;

    //display.display_digit(1,7).unwrap();

    display.display_number(1234).unwrap();

    loop {
        
        led.set_high().ok();

        delay.delay_ms(50 as u32);    

        if num >= 9 {
            num = 0;
        } else {
            num += 1;
        }

        //display.display_digit(3, num).unwrap();

        //display.display_temperature(0,TempUnit::C).unwrap();
        //display.display_number(7707).unwrap();

        led.set_low().ok();

        delay.delay_ms(200 as u32);    

        }
    
    
}

