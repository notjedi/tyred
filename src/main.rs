#![no_std]
#![no_main]

use panic_halt as _;

use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf::config::{Config, HfclkSource};
use embassy_nrf::usb::Driver;
use embassy_nrf::usb::vbus_detect::{HardwareVbusDetect, VbusDetect};
use embassy_nrf::{bind_interrupts, peripherals, usb};
use embassy_usb::Builder;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;

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
        &mut control_buf,
    );
    let mut cdc_acm = CdcAcmClass::new(&mut builder, &mut state, max_packet_size);

    let mut usb = builder.build();
    let usb_fut = usb.run();

    let echo_fut = async {
        cdc_acm.wait_connection().await;
        let _ = echo(&mut cdc_acm).await;
    };

    join(usb_fut, echo_fut).await;
}

async fn echo<'d, V: VbusDetect + 'd>(
    cdc_acm: &mut CdcAcmClass<'d, Driver<'d, V>>,
) -> Result<(), EndpointError> {
    let mut buf = [0; 64];
    loop {
        let size = cdc_acm.read_packet(&mut buf).await?;
        for i in 0..size {
            if buf[i] >= b'A' && buf[i] <= b'Z' {
                buf[i] += 32;
            } else if buf[i] >= b'a' && buf[i] <= b'z' {
                buf[i] -= 32;
            }
        }
        cdc_acm.write_packet(&mut buf[..size]).await?;
    }
}
