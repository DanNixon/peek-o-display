use embassy_rp::{config::Config, Peripherals};

// Non renamed re-exported peripherals
pub use embassy_rp::peripherals::{
    ADC, ADC_TEMP_SENSOR, BOOTSEL, CORE1, DMA_CH0, DMA_CH1, DMA_CH10, DMA_CH11, DMA_CH2, DMA_CH3,
    DMA_CH4, DMA_CH5, DMA_CH6, DMA_CH7, DMA_CH8, DMA_CH9, FLASH, I2C0, I2C1, PIN_10, PIN_11,
    PIN_12, PIN_13, PIN_16, PIN_17, PIN_18, PIN_19, PIN_20, PIN_21, PIN_22, PIN_23, PIN_24, PIN_25,
    PIN_26, PIN_27, PIN_28, PIN_29, PIN_QSPI_SCLK, PIN_QSPI_SD0, PIN_QSPI_SD1, PIN_QSPI_SD2,
    PIN_QSPI_SD3, PIN_QSPI_SS, PIO0, PIO1, PWM_SLICE0, PWM_SLICE1, PWM_SLICE2, PWM_SLICE3,
    PWM_SLICE4, PWM_SLICE5, PWM_SLICE6, PWM_SLICE7, RTC, SPI0, SPI1, UART0, UART1, USB, WATCHDOG,
};

#[cfg(feature = "rp235xa")]
pub use embassy_rp::peripherals::{
    DMA_CH12, DMA_CH13, DMA_CH14, DMA_CH15, PIO2, PWM_SLICE10, PWM_SLICE11, PWM_SLICE8, PWM_SLICE9,
    TRNG,
};

// Renamed pins
pub use embassy_rp::peripherals::{
    PIN_0, PIN_1, PIN_14 as TOUCH_CS, PIN_15 as TOUCH_IRQ, PIN_2 as SPI_CLK, PIN_3 as SPI_MOSI,
    PIN_4 as SPI_MISO, PIN_5 as DISPLAY_CS, PIN_6 as SDCARD_CS, PIN_7 as DISPLAY_RESET,
    PIN_8 as DISPLAY_DC, PIN_9 as DISPLAY_BACKLIGHT,
};

/// Struct containing peripherals, appropriately named for the Peek-o-Display board.
#[allow(non_snake_case)]
pub struct PeekODisplay {
    pub PIN_0: PIN_0,
    pub PIN_1: PIN_1,

    pub SPI_CLK: SPI_CLK,
    pub SPI_MOSI: SPI_MOSI,
    pub SPI_MISO: SPI_MISO,

    pub DISPLAY_CS: DISPLAY_CS,
    pub DISPLAY_RESET: DISPLAY_RESET,
    pub DISPLAY_DC: DISPLAY_DC,
    pub DISPLAY_BACKLIGHT: DISPLAY_BACKLIGHT,

    pub TOUCH_CS: TOUCH_CS,
    pub TOUCH_IRQ: TOUCH_IRQ,

    pub SDCARD_CS: SDCARD_CS,

    pub PIN_10: PIN_10,
    pub PIN_11: PIN_11,
    pub PIN_12: PIN_12,
    pub PIN_13: PIN_13,
    pub PIN_16: PIN_16,
    pub PIN_17: PIN_17,
    pub PIN_18: PIN_18,
    pub PIN_19: PIN_19,
    pub PIN_20: PIN_20,
    pub PIN_21: PIN_21,
    pub PIN_22: PIN_22,
    pub PIN_23: PIN_23,
    pub PIN_24: PIN_24,
    pub PIN_25: PIN_25,
    pub PIN_26: PIN_26,
    pub PIN_27: PIN_27,
    pub PIN_28: PIN_28,
    pub PIN_29: PIN_29,

    pub PIN_QSPI_SCLK: PIN_QSPI_SCLK,
    pub PIN_QSPI_SS: PIN_QSPI_SS,
    pub PIN_QSPI_SD0: PIN_QSPI_SD0,
    pub PIN_QSPI_SD1: PIN_QSPI_SD1,
    pub PIN_QSPI_SD2: PIN_QSPI_SD2,
    pub PIN_QSPI_SD3: PIN_QSPI_SD3,

    pub UART0: UART0,
    pub UART1: UART1,

    pub SPI0: SPI0,
    pub SPI1: SPI1,

    pub I2C0: I2C0,
    pub I2C1: I2C1,

    pub DMA_CH0: DMA_CH0,
    pub DMA_CH1: DMA_CH1,
    pub DMA_CH2: DMA_CH2,
    pub DMA_CH3: DMA_CH3,
    pub DMA_CH4: DMA_CH4,
    pub DMA_CH5: DMA_CH5,
    pub DMA_CH6: DMA_CH6,
    pub DMA_CH7: DMA_CH7,
    pub DMA_CH8: DMA_CH8,
    pub DMA_CH9: DMA_CH9,
    pub DMA_CH10: DMA_CH10,
    pub DMA_CH11: DMA_CH11,
    #[cfg(feature = "rp235xa")]
    pub DMA_CH12: DMA_CH12,
    #[cfg(feature = "rp235xa")]
    pub DMA_CH13: DMA_CH13,
    #[cfg(feature = "rp235xa")]
    pub DMA_CH14: DMA_CH14,
    #[cfg(feature = "rp235xa")]
    pub DMA_CH15: DMA_CH15,

    pub PWM_SLICE0: PWM_SLICE0,
    pub PWM_SLICE1: PWM_SLICE1,
    pub PWM_SLICE2: PWM_SLICE2,
    pub PWM_SLICE3: PWM_SLICE3,
    pub PWM_SLICE4: PWM_SLICE4,
    pub PWM_SLICE5: PWM_SLICE5,
    pub PWM_SLICE6: PWM_SLICE6,
    pub PWM_SLICE7: PWM_SLICE7,
    #[cfg(feature = "rp235xa")]
    pub PWM_SLICE8: PWM_SLICE8,
    #[cfg(feature = "rp235xa")]
    pub PWM_SLICE9: PWM_SLICE9,
    #[cfg(feature = "rp235xa")]
    pub PWM_SLICE10: PWM_SLICE10,
    #[cfg(feature = "rp235xa")]
    pub PWM_SLICE11: PWM_SLICE11,

    pub USB: USB,

    pub RTC: RTC,

    pub FLASH: FLASH,

    pub ADC: ADC,
    pub ADC_TEMP_SENSOR: ADC_TEMP_SENSOR,

    pub CORE1: CORE1,

    pub PIO0: PIO0,
    pub PIO1: PIO1,
    #[cfg(feature = "rp235xa")]
    pub PIO2: PIO2,

    pub WATCHDOG: WATCHDOG,

    pub BOOTSEL: BOOTSEL,

    #[cfg(feature = "rp235xa")]
    pub TRNG: TRNG,
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
        let p = Peripherals::steal();
        Self::from_peripherals(p)
    }

    fn from_peripherals(p: Peripherals) -> Self {
        Self {
            PIN_0: p.PIN_0,
            PIN_1: p.PIN_1,
            SPI_CLK: p.PIN_2,
            SPI_MOSI: p.PIN_3,
            SPI_MISO: p.PIN_4,
            DISPLAY_CS: p.PIN_5,
            SDCARD_CS: p.PIN_6,
            DISPLAY_RESET: p.PIN_7,
            DISPLAY_DC: p.PIN_8,
            DISPLAY_BACKLIGHT: p.PIN_9,
            PIN_10: p.PIN_10,
            PIN_11: p.PIN_11,
            PIN_12: p.PIN_12,
            PIN_13: p.PIN_13,
            TOUCH_CS: p.PIN_14,
            TOUCH_IRQ: p.PIN_15,
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
            SPI0: p.SPI0,
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
        }
    }
}
