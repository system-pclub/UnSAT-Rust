#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;
use usb_device::{
    device::{UsbDeviceBuilder, UsbVidPid},
    test_class::TestClass,
};

use core::ops::Deref;

use lpc546xx_hal::{
    pac::{self, SYSCON, USB0, USBFSH},
    prelude::*,
    syscon::{ClockControl, Config, Syscon},
};
use lpc_usbd::{self, bus::UsbBus, UsbPeripheral};
//pub use lpc_usbd::UsbBus;
use cortex_m_rt::entry;

pub struct USB {
    pub usb_dev: pac::USB0,
    pub usb_host: pac::USBFSH,
}

/*
> POWER_DisablePD(kPDRUNCFG_PD_USB0_PHY); /*< Turn on USB Phy */
> CLOCK_SetClkDiv(kCLOCK_DivUsb0Clk, 1, false);
>   CLOCK_AttachClk(kFRO_HF_to_USB0_CLK);
   /* enable usb0 host clock */
   CLOCK_EnableClock(kCLOCK_Usbhsl0);
   /*According to reference mannual, device mode setting has to be set by access usb host register */
   *((uint32_t *)(USBFSH_BASE + 0x5C)) |= USBFSH_PORTMODE_DEV_ENABLE_MASK;
   /* disable usb0 host clock */
   CLOCK_DisableClock(kCLOCK_Usbhsl0);
    */
impl USB {
    /// Construct a USB peripheral wrapper.
    ///
    /// Call `UsbBus::new` to construct and initialize the USB peripheral driver.
    pub fn new(usb_dev: pac::USB0, usb_host: pac::USBFSH, syscon: &mut Syscon) -> Self
    where
        pac::USB0: ClockControl,
    {
        let syscon_raw = unsafe { &*pac::SYSCON::ptr() };
        defmt::trace!("new_usb");
        cortex_m::asm::delay(100000);
        usb_dev.enable_clock(syscon);
        syscon_raw.usb0clkdiv.modify(|_, w| unsafe { w.div().bits(1) });
        syscon_raw.usb0clkdiv.modify(|_, w| w.halt().clear_bit());
        syscon_raw.usb0clksel.modify(|_, w| w.sel().fro_hf());
        while syscon_raw.usb0clkdiv.read().reqflag().bit_is_set() {}
        syscon_raw.ahbclkctrl2.modify(|_, w| w.usb1ram().set_bit());
        defmt::info!("clock: {:?}", usb_dev.get_clock_freq(syscon).unwrap().0);

        cortex_m::asm::delay(100000);
        Self { usb_dev, usb_host }
    }
}

unsafe impl Sync for USB {}

// impl<USB: UsbPeripheral> core::ops::Deref for UsbRegisters<USB> {
//     type Target = RegisterBlock;

//     fn deref(&self) -> &Self::Target {
//         let ptr = USB::REGISTERS as *const Self::Target;
//         unsafe { &*ptr }
//     }
// }

impl Deref for USB {
    type Target = lpc_usbd::pac::usb::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        let ptr = USB::REGISTERS as *const Self::Target;
        unsafe { &*ptr }
    }
}
impl UsbPeripheral for USB {
    const REGISTERS: *const () = pac::USB0::ptr() as *const ();
    fn enable() {
        cortex_m::asm::delay(100000);
        defmt::trace!("enable");
        cortex_m::asm::delay(100000);
        let usbd = unsafe { &*pac::USB0::ptr() };
        cortex_m::asm::delay(100000);
        let usbh = unsafe { &*pac::USBFSH::ptr() };
        cortex_m::asm::delay(100000);
        let syscon = unsafe { &*pac::SYSCON::ptr() };
        defmt::trace!("clkctrl2");
        cortex_m::asm::delay(100000);
        //syscon.ahbclkctrl2.modify(|_, w| w.usb0hsl().set_bit());
        cortex_m::asm::delay(100000);
        defmt::trace!("portmode");
        usbh.portmode.modify(|_, w| w.dev_enable().set_bit());
        cortex_m::asm::delay(100000);
        syscon.ahbclkctrl2.modify(|_, w| w.usb0hsl().clear_bit());

        // cortex_m::interrupt::free(|_| unsafe {
        //     // Enable USB peripheral
        //     pac::OTG_FS_GLOBAL::enable_unchecked();

        //     // Reset USB peripheral
        //     pac::OTG_FS_GLOBAL::reset_unchecked();
        // });
    }

    const SPEED: lpc_usbd::UsbSpeed = lpc_usbd::UsbSpeed::FullSpeed;
}

//pub type UsbBusType = UsbBus<USB>;

#[entry]
fn main() -> ! {
    let dp = lpc546xx_hal::pac::Peripherals::take().unwrap();
    defmt::info!("freezing!");
    cortex_m::asm::delay(100000);
    let mut syscon = dp.SYSCON.freeze(Config::frohf_96mhz());
    let mut iocon = dp.IOCON;
    let gpio = dp.GPIO.split(&mut syscon, &mut iocon);
    //let mut vbus_pin = gpio.pio0_22;
    //vbus_pin.set_alt_mode(lpc546xx_hal::gpio::AltMode::FUNC7);
    let usb = USB::new(dp.USB0, dp.USBFSH, &mut syscon);
    let usb_bus = UsbBus::new(usb);

    const VID: u16 = 0x16c0;
    const PID: u16 = 0x05dc;
    const MANUFACTURER: &'static str = "TestClass Manufacturer";
    const PRODUCT: &'static str = "virkkunen.net usb-device TestClass";
    const SERIAL_NUMBER: &'static str = "TestClass Serial";

    let mut test = TestClass::new(&usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(VID, PID))
        .manufacturer(MANUFACTURER)
        .product(PRODUCT)
        .serial_number(SERIAL_NUMBER)
        .max_packet_size_0(64)
        .build();

    defmt::info!("polling");

    loop {
        if usb_dev.poll(&mut [&mut test]) {
            defmt::info!("data!");
            test.poll();
        }
    }
}
