pub mod gpio;

/*const DMA: usize = 0x400c2000;
const AES: usize = 0x400e0000;
const USB: usize = 0x400c4000;
const MSC: usize = 0x400c0000;
const EMU: usize = 0x400c6000;
const RMU: usize = 0x400ca000;
const CMU: usize = 0x400c8000;
const LESENSE: usize = 0x4008c000;
const EBI: usize = 0x40008000;
const USART0: usize = 0x4000c000;
const USART1: usize = 0x4000c400;
const USART2: usize = 0x4000c800;
const UART0: usize = 0x4000e000;
const UART1: usize = 0x4000e400;
const TIMER0: usize = 0x40010000;
const TIMER1: usize = 0x40010400;
const TIMER2: usize = 0x40010800;
const TIMER3: usize = 0x40010c00;
const ACMP0: usize = 0x40001000;
const ACMP1: usize = 0x40001400;
const LEUART0: usize = 0x40084000;
const LEUART1: usize = 0x40084400;
const RTC: usize = 0x40080000;
const LETIMER0: usize = 0x40082000;
const PCNT0: usize = 0x40086000;
const PCNT1: usize = 0x40086400;
const PCNT2: usize = 0x40086800;
const I2C0: usize = 0x4000a000;
const I2C1: usize = 0x4000a400;*/
const GPIO: usize = 0x40006000;
/*const VCMP: usize = 0x40000000;
const PRS: usize = 0x400cc000;
const ADC0: usize = 0x40002000;
const DAC0: usize = 0x40004000;
const LCD: usize = 0x4008a000;
const BURTC: usize = 0x40081000;
const WDOG: usize = 0x40088000;
const ETM: usize = 0xe0041000;*/

pub fn gpio() -> &'static mut gpio::Gpio {
    unsafe { &mut *(GPIO as *mut gpio::Gpio) }
}
