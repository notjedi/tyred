#![no_std]
#![no_main]

use panic_halt as _;

use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf::{bind_interrupts, peripherals, usb};
use embassy_nrf::config::{Config, HfclkSource};
use embassy_nrf::usb::Driver;
use embassy_nrf::usb::vbus_detect::HardwareVbusDetect;
use embassy_usb::Builder;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};

bind_interrupts!(struct Irqs {
    USBD => usb::InterruptHandler<peripherals::USBD>;
    CLOCK_POWER => usb::vbus_detect::InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    /*
    we can also do the following:
      use embassy_nrf::pac;

      pac::CLOCK.tasks_hfclkstart().write_value(1);
      while pac::CLOCK.events_hfclkstarted().read() != 1 {}
       */
    let mut config = Config::default();
    config.hfclk_source = HfclkSource::ExternalXtal;
    let nn = embassy_nrf::init(config);

    let driver = Driver::new(nn.USBD, Irqs, HardwareVbusDetect::new(Irqs));

    let mut usb_config = embassy_usb::Config::new(0xc0de, 0xcafe);
    usb_config.manufacturer = Some("Indiska");
    usb_config.product = Some("Getting started w Embassy");
    usb_config.serial_number = Some("tyred");
    usb_config.max_power = 100;
    usb_config.max_packet_size_0 = 64;

    let mut config_descriptor_buf = [0; 256];
    let mut bos_descriptor_buf = [0; 256];
    let mut msos_descriptor_buf = [0; 256];
    let mut control_buf = [0; 256];

    let max_packet_size = 64;
    let mut state = State::new();
    let mut builder = Builder::new(
        driver,
        usb_config,
        &mut config_descriptor_buf,
        &mut bos_descriptor_buf,
        &mut msos_descriptor_buf,
        &mut control_buf
    );
    let mut cdc_acm = CdcAcmClass::new(&mut builder, &mut state, max_packet_size);

    let mut usb = builder.build();
    let usb_fut = usb.run();

    join(usb_fut, cdc_acm.wait_connection()).await;

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
