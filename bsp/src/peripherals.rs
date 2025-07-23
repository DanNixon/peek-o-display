// Non renamed re-exported peripherals
pub use embassy_rp::peripherals::{
    ADC, ADC_TEMP_SENSOR, BOOTSEL, CORE1, DMA_CH0, DMA_CH1, DMA_CH10, DMA_CH11, DMA_CH2, DMA_CH3,
    DMA_CH4, DMA_CH5, DMA_CH6, DMA_CH7, DMA_CH8, DMA_CH9, FLASH, I2C0, I2C1, PIN_0, PIN_1, PIN_10,
    PIN_11, PIN_12, PIN_13, PIN_16, PIN_17, PIN_18, PIN_19, PIN_20, PIN_21, PIN_22, PIN_23, PIN_24,
    PIN_25, PIN_26, PIN_27, PIN_28, PIN_29, PIN_QSPI_SCLK, PIN_QSPI_SD0, PIN_QSPI_SD1,
    PIN_QSPI_SD2, PIN_QSPI_SD3, PIN_QSPI_SS, PIO0, PIO1, PWM_SLICE0, PWM_SLICE1, PWM_SLICE2,
    PWM_SLICE3, PWM_SLICE4, PWM_SLICE5, PWM_SLICE6, PWM_SLICE7, RTC, SPI0, SPI1, UART0, UART1, USB,
    WATCHDOG,
};

// Non renamed re-exported peripherals
#[cfg(feature = "rp235xa")]
pub use embassy_rp::peripherals::{
    DMA_CH12, DMA_CH13, DMA_CH14, DMA_CH15, PIO2, PWM_SLICE10, PWM_SLICE11, PWM_SLICE8, PWM_SLICE9,
    TRNG,
};

// Renamed pins
pub use embassy_rp::peripherals::{
    PIN_14 as TOUCH_CS, PIN_15 as TOUCH_IRQ, PIN_2 as SPI_CLK, PIN_3 as SPI_MOSI,
    PIN_4 as SPI_MISO, PIN_5 as DISPLAY_CS, PIN_6 as SDCARD_CS, PIN_7 as DISPLAY_RESET,
    PIN_8 as DISPLAY_DC, PIN_9 as DISPLAY_BACKLIGHT,
};

pub use embassy_rp::Peri;

/// Struct containing peripherals, appropriately named for the Peek-o-Display board.
#[allow(non_snake_case)]
pub struct Peripherals {
    pub PIN_0: Peri<'static, PIN_0>,
    pub PIN_1: Peri<'static, PIN_1>,
    pub PIN_10: Peri<'static, PIN_10>,
    pub PIN_11: Peri<'static, PIN_11>,
    pub PIN_12: Peri<'static, PIN_12>,
    pub PIN_13: Peri<'static, PIN_13>,
    pub PIN_16: Peri<'static, PIN_16>,
    pub PIN_17: Peri<'static, PIN_17>,
    pub PIN_18: Peri<'static, PIN_18>,
    pub PIN_19: Peri<'static, PIN_19>,
    pub PIN_20: Peri<'static, PIN_20>,
    pub PIN_21: Peri<'static, PIN_21>,
    pub PIN_22: Peri<'static, PIN_22>,
    pub PIN_23: Peri<'static, PIN_23>,
    pub PIN_24: Peri<'static, PIN_24>,
    pub PIN_25: Peri<'static, PIN_25>,
    pub PIN_26: Peri<'static, PIN_26>,
    pub PIN_27: Peri<'static, PIN_27>,
    pub PIN_28: Peri<'static, PIN_28>,
    pub PIN_29: Peri<'static, PIN_29>,

    pub PIN_QSPI_SCLK: Peri<'static, PIN_QSPI_SCLK>,
    pub PIN_QSPI_SS: Peri<'static, PIN_QSPI_SS>,
    pub PIN_QSPI_SD0: Peri<'static, PIN_QSPI_SD0>,
    pub PIN_QSPI_SD1: Peri<'static, PIN_QSPI_SD1>,
    pub PIN_QSPI_SD2: Peri<'static, PIN_QSPI_SD2>,
    pub PIN_QSPI_SD3: Peri<'static, PIN_QSPI_SD3>,

    pub UART0: Peri<'static, UART0>,
    pub UART1: Peri<'static, UART1>,

    pub SPI1: Peri<'static, SPI1>,

    pub I2C0: Peri<'static, I2C0>,
    pub I2C1: Peri<'static, I2C1>,

    pub DMA_CH0: Peri<'static, DMA_CH0>,
    pub DMA_CH1: Peri<'static, DMA_CH1>,
    pub DMA_CH2: Peri<'static, DMA_CH2>,
    pub DMA_CH3: Peri<'static, DMA_CH3>,
    pub DMA_CH4: Peri<'static, DMA_CH4>,
    pub DMA_CH5: Peri<'static, DMA_CH5>,
    pub DMA_CH6: Peri<'static, DMA_CH6>,
    pub DMA_CH7: Peri<'static, DMA_CH7>,
    pub DMA_CH8: Peri<'static, DMA_CH8>,
    pub DMA_CH9: Peri<'static, DMA_CH9>,
    pub DMA_CH10: Peri<'static, DMA_CH10>,
    pub DMA_CH11: Peri<'static, DMA_CH11>,
    #[cfg(feature = "rp235xa")]
    pub DMA_CH12: Peri<'static, DMA_CH12>,
    #[cfg(feature = "rp235xa")]
    pub DMA_CH13: Peri<'static, DMA_CH13>,
    #[cfg(feature = "rp235xa")]
    pub DMA_CH14: Peri<'static, DMA_CH14>,
    #[cfg(feature = "rp235xa")]
    pub DMA_CH15: Peri<'static, DMA_CH15>,

    pub PWM_SLICE0: Peri<'static, PWM_SLICE0>,
    pub PWM_SLICE1: Peri<'static, PWM_SLICE1>,
    pub PWM_SLICE2: Peri<'static, PWM_SLICE2>,
    pub PWM_SLICE3: Peri<'static, PWM_SLICE3>,
    pub PWM_SLICE4: Peri<'static, PWM_SLICE4>,
    pub PWM_SLICE5: Peri<'static, PWM_SLICE5>,
    pub PWM_SLICE6: Peri<'static, PWM_SLICE6>,
    pub PWM_SLICE7: Peri<'static, PWM_SLICE7>,
    #[cfg(feature = "rp235xa")]
    pub PWM_SLICE8: Peri<'static, PWM_SLICE8>,
    #[cfg(feature = "rp235xa")]
    pub PWM_SLICE9: Peri<'static, PWM_SLICE9>,
    #[cfg(feature = "rp235xa")]
    pub PWM_SLICE10: Peri<'static, PWM_SLICE10>,
    #[cfg(feature = "rp235xa")]
    pub PWM_SLICE11: Peri<'static, PWM_SLICE11>,

    pub USB: Peri<'static, USB>,

    pub RTC: Peri<'static, RTC>,

    pub FLASH: Peri<'static, FLASH>,

    pub ADC: Peri<'static, ADC>,
    pub ADC_TEMP_SENSOR: Peri<'static, ADC_TEMP_SENSOR>,

    pub CORE1: Peri<'static, CORE1>,

    pub PIO0: Peri<'static, PIO0>,
    pub PIO1: Peri<'static, PIO1>,
    #[cfg(feature = "rp235xa")]
    pub PIO2: Peri<'static, PIO2>,

    pub WATCHDOG: Peri<'static, WATCHDOG>,

    pub BOOTSEL: Peri<'static, BOOTSEL>,

    #[cfg(feature = "rp235xa")]
    pub TRNG: Peri<'static, TRNG>,
}
