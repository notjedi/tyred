#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use embassy_nrf::config::Config;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::{Duration, Timer};

#[entry]
fn main() -> ! {
    let config = Config::default();
    let nrf = embassy_nrf::init(config);

    let mut led = Output::new(nrf.P0_15, Level::Low, OutputDrive::Standard);

    loop {
        led.set_high();
        embassy_time::block_for(Duration::from_secs(2));
        led.set_low();
        embassy_time::block_for(Duration::from_secs(2));
    }

    // let periph = nrf52840_hal::pac::Peripherals::take().unwrap();
    // let clocks = Clocks::new(periph.CLOCK);
    // let clocks = clocks.enable_ext_hfosc();
    //
    // let usb_bus = UsbBusAllocator::new(Usbd::new(UsbPeripheral::new(periph.USBD, &clocks)));
    // let mut serial = SerialPort::new(&usb_bus);
    //
    // let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
    //     .strings(&[StringDescriptors::default()
    //         .manufacturer("Indiska")
    //         .product("Serial port")
    //         .serial_number("tyred")])
    //     .unwrap()
    //     .device_class(USB_CLASS_CDC)
    //     .max_packet_size_0(64) // (makes control transfers 8x faster)
    //     .unwrap()
    //     .build();
    //
    // loop {
    //     if !usb_dev.poll(&mut [&mut serial]) {
    //         continue;
    //     }
    //
    //     let mut buf = [0u8; 64];
    //
    //     match serial.read(&mut buf) {
    //         Ok(count) if count > 0 => {
    //             // Echo back in upper case
    //             for c in buf[0..count].iter_mut() {
    //                 if 0x61 <= *c && *c <= 0x7a {
    //                     *c &= !0x20;
    //                 }
    //             }
    //
    //             let mut write_offset = 0;
    //             while write_offset < count {
    //                 match serial.write(&buf[write_offset..count]) {
    //                     Ok(len) if len > 0 => {
    //                         write_offset += len;
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //         _ => {}
    //     }
    // }
}
