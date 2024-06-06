#![no_std]
#![no_main]

mod image;
mod drivers;

extern crate panic_semihosting;
extern crate stm32_hal as hal;

use core::fmt::Write;

use cortex_m_rt::entry;
use cortex_m_semihosting::hio::{ self, HostStream };
use drivers::ili9341::ILI9341;
use hal::{
    embedded_hal::digital::{ InputPin, OutputPin, StatefulOutputPin },
    gpio::{ self, pin::{ Input, Output, OutputType, Pull, Speed }, PinMask },
    ltdc::{ self, Color, LTDCConfig, PixelClockPolarity, PixelFormat, Polarity },
    rcc::{
        self,
        AHBPrescaler,
        APBPrescaler,
        BUSConfig,
        LCDClockDivisionFactor,
        PLLClockSource,
        PLLConfig,
        PLLSAIConfig,
        PLLSysClockDivisionFactor,
        SystemClockConfig,
    },
    spi::{
        self, BaudRate, BusConfiguration, ClockPhase, ClockPolarity, DataFrameFormat, Mode, SPIConfig, SPI
    },
    Peripheral,
    PeripheralRef,
};
use log::{ info, Log };

#[entry]
fn main() -> ! {
    SemihostingLogger::init().expect("Failed to initialize logger!");

    init_system_clocks();
    init_ltdc_pins();

    let gpioa = gpio::GPIOA::take();
    let gpiog = gpio::GPIOG::take();

    let mut green_led = Output::new(gpiog.pin(13), Speed::High);
    let mut red_led = Output::new(gpiog.pin(14), Speed::High);

    green_led.set_high().unwrap();
    red_led.set_low().unwrap();

    let mut user_button = Input::new(gpioa.pin(0), Pull::None);
    // user_button.set_irq_handler(InterruptType::RisingEdge, interrupt_handler);

    let lcd = LCD::init();
    let _ili9341 = ILI9341::init(lcd);

    let ltdc = ltdc::LTDC::take();
    ltdc.init(LTDCConfig {
        horizontal_sync_polarity: Polarity::ActiveLow,
        vertical_sync_polarity: Polarity::ActiveLow,
        data_enable_polarity: Polarity::ActiveLow,
        pixel_clock_polarity: PixelClockPolarity::Normal,
        horizontal_sync: ILI9341::ILI9341_HSYNC,
        vertical_sync: ILI9341::ILI9341_VSYNC,
        horizontal_back_porch: ILI9341::ILI9341_HBP,
        vertical_back_porch: ILI9341::ILI9341_VBP,
        active_width: ILI9341::ILI9341_LCD_PIXEL_WIDTH,
        active_height: ILI9341::ILI9341_LCD_PIXEL_HEIGHT,
        horizontal_front_porch: ILI9341::ILI9341_HFP,
        vertical_front_porch: ILI9341::ILI9341_VFP,
        background_color: Color(0, 0, 0, 0),
    });
    ltdc.layer1_configure(
        0,
        0,
        240,
        320,
        PixelFormat::RGB565,
        Color(0, 0, 0, 0),
        &image::IMAGE as *const _
    );
    // ltdc.layer1_disable();
    // ltdc.layer2_disable();

    loop {
        for _ in 0..100_000 {
            // let mut i = 100_000u32;
            // while i > 0 {
            //     i = i.wrapping_sub(1);
            if user_button.is_high().unwrap() {
                green_led.toggle().unwrap();
            }
        }

        green_led.toggle().unwrap();
        red_led.toggle().unwrap();
    }
}

/// System Clock Configuration
///   The system Clock is configured as follow :
///       System Clock source            = PLL (HSE)
///       SYSCLK(Hz)                     = 72000000
///       HCLK(Hz)                       = 72000000
///       AHB Prescaler                  = 1
///       APB1 Prescaler                 = 2
///       APB2 Prescaler                 = 1
///       HSE Frequency(Hz)              = 8000000
///       PLL_M                          = 4
///       PLL_N                          = 72
///       PLL_P                          = 2
///       PLL_Q                          = 3
///       VDD(V)                         = 3.3
///       Main regulator output voltage  = Scale1 mode
///       Flash Latency(WS)              = 5
///    The LTDC Clock is configured as follow :
///       PLLSAIN                        = 50
///       PLLSAIR                        = 2
///       PLLSAIDivR                     = 2
fn init_system_clocks() {
    let rcc = rcc::RCC::take();
    rcc.configure_system_clock(
        SystemClockConfig::PLL(PLLConfig {
            clock_source: PLLClockSource::HSE,
            pllm: 4,
            plln: 72,
            pllq: 3,
            system_clock_div_factor: PLLSysClockDivisionFactor::DividedBy2,
        }),
        BUSConfig {
            ahb_prescaler: AHBPrescaler::NotDivided,
            apb1_prescaler: APBPrescaler::DividedBy2,
            apb2_prescaler: APBPrescaler::NotDivided,
        }
    );

    rcc.configure_pllsai(PLLSAIConfig {
        pllsain: 60, //50,
        pllsaiq: 4,
        pllsair: 5, //2,
        lcd_div_factor: LCDClockDivisionFactor::DividedBy4, //LCDClockDivisionFactor::DividedBy2,
    })
}

fn init_ltdc_pins() {
    let gpioa = gpio::GPIOA::take();
    let gpiob = gpio::GPIOB::take();
    let gpioc = gpio::GPIOC::take();
    let gpiod = gpio::GPIOD::take();
    let gpiof = gpio::GPIOF::take();
    let gpiog = gpio::GPIOG::take();

    gpioa.enable_clock();
    gpiob.enable_clock();
    gpioc.enable_clock();
    gpiod.enable_clock();
    gpiof.enable_clock();
    gpiog.enable_clock();

    gpioa.init_alternate_pins(
        PinMask::PIN3 | PinMask::PIN4 | PinMask::PIN6 | PinMask::PIN11 | PinMask::PIN12,
        OutputType::PushPull,
        Speed::High,
        Pull::None,
        14
    );
    gpiob.init_alternate_pins(
        PinMask::PIN8 | PinMask::PIN9 | PinMask::PIN10 | PinMask::PIN11,
        OutputType::PushPull,
        Speed::High,
        Pull::None,
        14
    );
    gpioc.init_alternate_pins(
        PinMask::PIN6 | PinMask::PIN7 | PinMask::PIN10,
        OutputType::PushPull,
        Speed::High,
        Pull::None,
        14
    );
    gpiod.init_alternate_pins(
        PinMask::PIN3 | PinMask::PIN6,
        OutputType::PushPull,
        Speed::High,
        Pull::None,
        14
    );
    gpiof.init_alternate_pins(PinMask::PIN10, OutputType::PushPull, Speed::High, Pull::None, 14);
    gpiog.init_alternate_pins(
        PinMask::PIN6 | PinMask::PIN7 | PinMask::PIN11,
        OutputType::PushPull,
        Speed::High,
        Pull::None,
        14
    );

    gpiob.init_alternate_pins(
        PinMask::PIN0 | PinMask::PIN1,
        OutputType::PushPull,
        Speed::High,
        Pull::None,
        9
    );
    gpiog.init_alternate_pins(
        PinMask::PIN10 | PinMask::PIN12,
        OutputType::PushPull,
        Speed::High,
        Pull::None,
        9
    );
}

pub struct LCD {
    spi: &'static mut SPI,
    ncs: Output,
    rdx: Output,
    wrx: Output,
}

impl LCD {
    pub fn init() -> Self {
        info!("Init LCD pins");
        // init GPIO pins
        // PC2 -> NCS
        // PD12 -> RDX
        // PD13 -> WRX
        // PF7 -> SPI5_SCK
        // PF8 -> SPI5_MISO
        // PF9 -> SPI5_MOSI
        let mut ncs = Output::new(gpio::GPIOC::take().pin(2), Speed::High);
        let rdx = Output::new(gpio::GPIOD::take().pin(12), Speed::High);
        let wrx = Output::new(gpio::GPIOD::take().pin(13), Speed::High);
        gpio::GPIOF
            ::take()
            .init_alternate_pins(
                PinMask::PIN7 | PinMask::PIN8 | PinMask::PIN9,
                OutputType::PushPull,
                Speed::VeryHigh,
                Pull::None,
                5
            );

        let spi = spi::SPI5::take();
        let config = SPIConfig {
            mode: Mode::Master,
            bus_config: BusConfiguration::FullDuplex,
            baud_rate: BaudRate::FpclkDiv16,
            data_format: DataFrameFormat::Format8Bit,
            cpol: ClockPolarity::IdleLow,
            cpha: ClockPhase::FirstClockTransition,
            ssm: true,
        };

        ncs.set_low().unwrap();
        ncs.set_high().unwrap();

        info!("Init SPI5 peripheral");
        spi.init(config).unwrap();

        Self { spi, ncs, rdx, wrx }
    }

    pub fn write_reg(&mut self, reg: u8) {
        self.wrx.set_low().unwrap();
        self.ncs.set_low().unwrap();
        self.spi.write_data(&[reg]).unwrap();
        self.ncs.set_high().unwrap();
    }

    pub fn write_data(&mut self, data: u8) {
        self.wrx.set_high().unwrap();
        self.ncs.set_low().unwrap();
        self.spi.write_data(&[data]).unwrap();
        self.ncs.set_high().unwrap();
    }
}



// fn interrupt_handler() {
//     GPIOG::pin::<13>().toggle_output();
// }

static mut SEMIHOSTING_LOGGER: SemihostingLogger = SemihostingLogger { host_stream: None };

pub struct SemihostingLogger {
    host_stream: Option<HostStream>,
}

impl SemihostingLogger {
    pub fn init() -> Result<(), ()> {
        unsafe {
            SEMIHOSTING_LOGGER.host_stream = Some(hio::hstdout()?);
            log::set_logger(&SEMIHOSTING_LOGGER).map_err(|_| ())?;
            log::set_max_level(log::LevelFilter::Trace);
        }

        Ok(())
    }
}

impl Log for SemihostingLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        self.host_stream.is_some()
    }

    fn log(&self, record: &log::Record) {
        if let Some(stream) = &self.host_stream {
            let mut stream = stream.clone();
            stream
                .write_fmt(
                    format_args!(
                        "[{}] <{}> {}\n",
                        record.target(),
                        record.metadata().level(),
                        record.args()
                    )
                )
                .unwrap();
        }
    }

    fn flush(&self) {}
}
