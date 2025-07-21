#![no_std]
#![no_main]

pub mod display;
pub mod peripherals;
pub mod sdcard;
pub mod touch;

use crate::peripherals::*;
use core::cell::RefCell;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::{
    config::Config,
    gpio::{Input, Output},
    Peripherals as RpPeripherals,
};
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};
use mipidsi::options::Rotation;
use portable_atomic as _;
use static_cell::StaticCell;
use touch::Touch;

/// PeekODisplay board representation.
#[allow(non_snake_case)]
pub struct PeekODisplay {
    peripherals: RefCell<Option<Peripherals>>,

    spi_resources: RefCell<Option<SpiResources>>,

    display_resources: RefCell<Option<DisplayResources>>,
    touch_resources: RefCell<Option<TouchResources>>,
    sdcard_resources: RefCell<Option<SdCardResources>>,
}

impl Default for PeekODisplay {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl PeekODisplay {
    pub fn new(config: Config) -> Self {
        let p = embassy_rp::init(config);
        Self::from_peripherals(p)
    }

    /// # Safety
    ///
    /// OK providing that only one instance of `PicoPlc` exists.
    pub unsafe fn steal() -> Self {
        let p = unsafe { RpPeripherals::steal() };
        Self::from_peripherals(p)
    }

    fn from_peripherals(p: RpPeripherals) -> Self {
        let spi_resources = Some(SpiResources {
            peripheral: p.SPI0,
            clk: p.PIN_2,
            mosi: p.PIN_3,
            miso: p.PIN_4,
        })
        .into();

        let display_resources = Some(DisplayResources {
            cs: p.PIN_5,
            dc: p.PIN_8,
            reset: p.PIN_7,
            backlight: p.PIN_9,
        })
        .into();

        let touch_resources = Some(TouchResources {
            cs: p.PIN_14,
            irq: p.PIN_15,
        })
        .into();

        let sdcard_resources = Some(SdCardResources { cs: p.PIN_6 }).into();

        let peripherals = Some(Peripherals {
            PIN_0: p.PIN_0,
            PIN_1: p.PIN_1,
            PIN_10: p.PIN_10,
            PIN_11: p.PIN_11,
            PIN_12: p.PIN_12,
            PIN_13: p.PIN_13,
            PIN_16: p.PIN_16,
            PIN_17: p.PIN_17,
            PIN_18: p.PIN_18,
            PIN_19: p.PIN_19,
            PIN_20: p.PIN_20,
            PIN_21: p.PIN_21,
            PIN_22: p.PIN_22,
            PIN_23: p.PIN_23,
            PIN_24: p.PIN_24,
            PIN_25: p.PIN_25,
            PIN_26: p.PIN_26,
            PIN_27: p.PIN_27,
            PIN_28: p.PIN_28,
            PIN_29: p.PIN_29,
            PIN_QSPI_SCLK: p.PIN_QSPI_SCLK,
            PIN_QSPI_SS: p.PIN_QSPI_SS,
            PIN_QSPI_SD0: p.PIN_QSPI_SD0,
            PIN_QSPI_SD1: p.PIN_QSPI_SD1,
            PIN_QSPI_SD2: p.PIN_QSPI_SD2,
            PIN_QSPI_SD3: p.PIN_QSPI_SD3,
            UART0: p.UART0,
            UART1: p.UART1,
            SPI1: p.SPI1,
            I2C0: p.I2C0,
            I2C1: p.I2C1,
            DMA_CH0: p.DMA_CH0,
            DMA_CH1: p.DMA_CH1,
            DMA_CH2: p.DMA_CH2,
            DMA_CH3: p.DMA_CH3,
            DMA_CH4: p.DMA_CH4,
            DMA_CH5: p.DMA_CH5,
            DMA_CH6: p.DMA_CH6,
            DMA_CH7: p.DMA_CH7,
            DMA_CH8: p.DMA_CH8,
            DMA_CH9: p.DMA_CH9,
            DMA_CH10: p.DMA_CH10,
            DMA_CH11: p.DMA_CH11,
            #[cfg(feature = "rp235xa")]
            DMA_CH12: p.DMA_CH12,
            #[cfg(feature = "rp235xa")]
            DMA_CH13: p.DMA_CH13,
            #[cfg(feature = "rp235xa")]
            DMA_CH14: p.DMA_CH14,
            #[cfg(feature = "rp235xa")]
            DMA_CH15: p.DMA_CH15,
            PWM_SLICE0: p.PWM_SLICE0,
            PWM_SLICE1: p.PWM_SLICE1,
            PWM_SLICE2: p.PWM_SLICE2,
            PWM_SLICE3: p.PWM_SLICE3,
            PWM_SLICE4: p.PWM_SLICE4,
            PWM_SLICE5: p.PWM_SLICE5,
            PWM_SLICE6: p.PWM_SLICE6,
            PWM_SLICE7: p.PWM_SLICE7,
            #[cfg(feature = "rp235xa")]
            PWM_SLICE8: p.PWM_SLICE8,
            #[cfg(feature = "rp235xa")]
            PWM_SLICE9: p.PWM_SLICE9,
            #[cfg(feature = "rp235xa")]
            PWM_SLICE10: p.PWM_SLICE10,
            #[cfg(feature = "rp235xa")]
            PWM_SLICE11: p.PWM_SLICE11,
            USB: p.USB,
            RTC: p.RTC,
            FLASH: p.FLASH,
            ADC: p.ADC,
            ADC_TEMP_SENSOR: p.ADC_TEMP_SENSOR,
            CORE1: p.CORE1,
            PIO0: p.PIO0,
            PIO1: p.PIO1,
            #[cfg(feature = "rp235xa")]
            PIO2: p.PIO2,
            WATCHDOG: p.WATCHDOG,
            BOOTSEL: p.BOOTSEL,
            #[cfg(feature = "rp235xa")]
            TRNG: p.TRNG,
        })
        .into();

        Self {
            spi_resources,
            display_resources,
            touch_resources,
            sdcard_resources,
            peripherals,
        }
    }

    pub fn peripherals(&self) -> Peripherals {
        self.peripherals.take().unwrap()
    }

    pub fn board_spi(&self) -> &'static mut BoardSharedSpiBus {
        let res = self.spi_resources.take().unwrap();

        let mut config = embassy_rp::spi::Config::default();
        config.phase = embassy_rp::spi::Phase::CaptureOnSecondTransition;
        config.polarity = embassy_rp::spi::Polarity::IdleHigh;

        let spi =
            embassy_rp::spi::Spi::new_blocking(res.peripheral, res.clk, res.mosi, res.miso, config);

        let spi = Mutex::new(RefCell::new(spi));

        static SPI_BUS: StaticCell<BoardSharedSpiBus> = StaticCell::new();
        SPI_BUS.init(spi)
    }

    pub fn display_resources(&self) -> DisplayResources {
        self.display_resources.take().unwrap()
    }

    pub fn display(
        &self,
        spi: &'static BoardSharedSpiBus,
        rotation: Rotation,
    ) -> (display::Display, Output<'static>) {
        static BUFF: StaticCell<[u8; 512]> = StaticCell::new();
        let buffer: &'static mut [u8; 512] = BUFF.init([0u8; 512]);

        display::init(spi, self.display_resources(), buffer, rotation)
    }

    pub fn touch_resources(&self) -> TouchResources {
        self.touch_resources.take().unwrap()
    }

    pub fn touch(&self, spi: &'static BoardSharedSpiBus) -> (Touch, Input<'static>) {
        touch::init(spi, self.touch_resources())
    }

    pub fn sdcard_resources(&self) -> SdCardResources {
        self.sdcard_resources.take().unwrap()
    }

    pub fn sdcard(&self, spi: &'static BoardSharedSpiBus) -> sdcard::SdCard {
        sdcard::init(spi, self.sdcard_resources())
    }
}

pub type BoardSharedSpiBus =
    Mutex<NoopRawMutex, RefCell<embassy_rp::spi::Spi<'static, SPI0, embassy_rp::spi::Blocking>>>;

pub type BoardSpiDevice = SpiDeviceWithConfig<
    'static,
    NoopRawMutex,
    embassy_rp::spi::Spi<'static, embassy_rp::peripherals::SPI0, embassy_rp::spi::Blocking>,
    Output<'static>,
>;

pub struct SpiResources {
    peripheral: SPI0,
    clk: SPI_CLK,
    mosi: SPI_MOSI,
    miso: SPI_MISO,
}

pub struct DisplayResources {
    pub cs: DISPLAY_CS,
    pub dc: DISPLAY_DC,
    pub reset: DISPLAY_RESET,
    pub backlight: DISPLAY_BACKLIGHT,
}

pub struct TouchResources {
    pub cs: TOUCH_CS,
    pub irq: TOUCH_IRQ,
}

pub struct SdCardResources {
    pub cs: SDCARD_CS,
}
