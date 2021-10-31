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
    i2c::I2c,
    };

use akafugu_twidisplay::*;

const BOOT_DELAY_MS: u16 = 100; 

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

    // delay necessary for the I2C to initiate correctly and start on boot without having to reset the board
    delay.delay_ms(BOOT_DELAY_MS);
    
    let mut scl = gpioa.pa9.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    
    scl.internal_pull_up(&mut gpioa.pupdr, true);
    let scl = scl.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut sda = gpioa.pa10.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    sda.internal_pull_up(&mut gpioa.pupdr, true);
    let sda = sda.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks, &mut rcc.apb1r1);
    
    let mut akafugu = TWIDisplay::new(i2c, DEFAULT_ADDRESS);
    
    akafugu.clear_display().unwrap();

 

    loop {
        
        // demo sequence

        led.set_high().ok();

        akafugu.display_address();

        delay.delay_ms(1000_u32);
        
        led.set_low().ok();

        akafugu.display_temperature(-10,TempUnits::Celsius, Some(-50), Some(199)).unwrap();
    
        delay.delay_ms(1000_u32);

        led.set_high().ok();

        akafugu.set_brightness(80).unwrap();
    
        akafugu.display_time(10,35, true).unwrap();            

        delay.delay_ms(1000_u32);

        led.set_low().ok();
    
        akafugu.display_time(23,59, false).unwrap();
    
        delay.delay_ms(1000_u32);
    
        led.set_high().ok();

        akafugu.display_humidity(55, None, None).unwrap();
    
        delay.delay_ms(1000_u32);
        
        akafugu.clear_display().unwrap();

        led.set_low().ok();

        delay.delay_ms(1000 as u32);    
        
        }
    
    
}

