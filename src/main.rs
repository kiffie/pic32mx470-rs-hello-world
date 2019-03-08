
#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(asm)]

use core::panic::PanicInfo;
use core::fmt::Write;
use log;
use log::{info, error};
use mips_rt;
use pic32mx4xxfxxxh;

use core::cell::Cell;
use mips_rt::interrupt;
use mips_rt::interrupt::Mutex;

// PIC32 configuration registers
#[link_section = ".configsfrs"]
#[no_mangle]
pub static CONFIGSFRS: [u32; 4] = [
    0xffffffff,     // DEVCFG3
    0xfff879f9,     // DEVCFG2
    0xff744ddb,     // DEVCFG1
    0x7ffffff3,     // DEVCFG0
];

static TICKS: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

// Timer 1 ISR
#[no_mangle]
pub extern "C" fn _vector_4_fn() {
    let ctr = interrupt::free(|cs| {
        let cell = TICKS.borrow(cs);
        let ctr = cell.get() + 1;
        cell.set(ctr);
        ctr
    });
    set_yellow_led(ctr & 0x01 != 0);
    let  p = unsafe { pic32mx4xxfxxxh::Peripherals::steal()};
    p.INT.ifs0clr.write(|w| { w.t1if().bit(true) });
}

fn timer_init(){

    let  p = unsafe { pic32mx4xxfxxxh::Peripherals::steal()};
    p.TMR1.pr1.write(|w| unsafe { w.bits(0xffff) });
    p.TMR1.t1con.write(|w| unsafe { w.on().bit(true)
                                    .tckps().bits(0b11)});

    p.INT.ifs0clr.write(|w| { w.t1if().bit(true) });
    p.INT.iec0set.write(|w| { w.t1ie().bit(true) });
    p.INT.ipc1.modify(|_, w| unsafe { w.t1ip().bits(1) });
}


fn set_yellow_led(on: bool){
    let bit = 1 << 1;
    let  p = unsafe { pic32mx4xxfxxxh::Peripherals::steal()};
    p.PORTD.anseldclr.write(|w| unsafe { w.bits(bit)});
    p.PORTD.trisdclr.write(|w| unsafe { w.bits(bit) });
    if on {
        p.PORTD.latdset.write(|w| unsafe { w.bits(bit)});
    }else{
        p.PORTD.latdclr.write(|w| unsafe {w.bits(bit)});
    }
}

fn set_green_led(on: bool){
    let bit = 1 << 6;
    let  p = unsafe { pic32mx4xxfxxxh::Peripherals::steal()};
    let port = &p.PORTG;
    port.anselgclr.write(|w| unsafe { w.bits(bit)});
    port.trisgclr.write(|w| unsafe { w.bits(bit) });
    if on {
        port.latgset.write(|w| unsafe { w.bits(bit)});
    }else{
        port.latgclr.write(|w| unsafe {w.bits(bit)});
    }
}

struct Uart;

impl Uart {

    fn init() -> Uart {
        let  p = unsafe { pic32mx4xxfxxxh::Peripherals::steal()};

        let pps = p.PPS;
        pps.rpf5r.write(|w| unsafe { w.rpf5r().bits(0b0001) }); // UART 2 on RPF5

        let sys_clock : u32 = 96000000;
        let pb_clock : u32 = sys_clock; // TODO: consider PBDIV
        let baud_rate : u32 = 115200;
        let brg = (pb_clock/(4*baud_rate)-1) as u16;

        let uart2 = p.UART2;
        uart2.u2mode.write(|w|  { w.brgh().bit(true) });
        uart2.u2sta.write(|w| unsafe { w.utxen().bit(true).utxisel().bits(0b10) });
        uart2.u2brg.write(|w| unsafe { w.brg().bits(brg) });
        uart2.u2modeset.write(|w| w.on().bit(true));
        Uart
    }

    fn write_byte(&self, byte: u8){

        let  uart2 = unsafe { pic32mx4xxfxxxh::Peripherals::steal()}.UART2;

        while uart2.u2sta.read().utxbf().bit() { }
        uart2.u2txreg.write(|w| unsafe { w.u2txreg().bits(byte as u16) });
    }
}

impl Write for Uart {

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.write_byte(b);
        }
        Ok(())
    }

}

fn debug_write() -> Uart {
    Uart
}

struct UartLogger;

impl log::Log for UartLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            writeln!(debug_write(), "{} - {}", record.level(), record.args()).unwrap();
        }
    }

    fn flush(&self) {}
}

static UART_LOGGER: UartLogger = UartLogger;

#[no_mangle]
pub fn main() -> ! {

    log::set_logger(&UART_LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    let mut uart = Uart::init();
    let mut state = false;
    set_yellow_led(false);
    info!("initializing Timer 1");
    unsafe {
        interrupt::enable_mv_irq();
        let  p = pic32mx4xxfxxxh::Peripherals::steal();
        p.INT.intconset.write(|w| { w.mvec().bit(true).ss0().bit(false) });
        interrupt::enable();
    }
    timer_init();
    info!("starting loop");
    loop {
        let ticks: u32 = { interrupt::free(|cs| { TICKS.borrow(cs).get() }) };
        writeln!(uart, "Hello World! ticks = {}", ticks).unwrap();
        //set_yellow_led(state);
        set_green_led(!state);
        let mut i = 1000000;
        while i > 0 {
            i-= 1;
            unsafe { asm!("nop") };
        }
        state = !state;
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    error!("Panic: entering endless loop");
    loop {}
}

