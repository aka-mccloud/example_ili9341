#![allow(unused)]

use log::info;

use crate::LCD;

pub struct ILI9341 {
    lcd: LCD,
}

impl ILI9341 {
    pub const ILI9341_LCD_PIXEL_WIDTH: usize = 240;
    pub const ILI9341_LCD_PIXEL_HEIGHT: usize = 320;

    ///
    /// @brief  ILI9341 Timing
    /// Timing configuration  (Typical configuration from ILI9341 datasheet)
    ///   HSYNC=10 (9+1)
    ///   HBP=20 (29-10+1)
    ///   ActiveW=240 (269-20-10+1)
    ///   HFP=10 (279-240-20-10+1)
    ///
    ///   VSYNC=2 (1+1)
    ///   VBP=2 (3-2+1)
    ///   ActiveH=320 (323-2-2+1)
    ///   VFP=4 (327-320-2-2+1)

    /// Horizontal synchronization
    pub const ILI9341_HSYNC: u16 = 10;
    /// Horizontal back porch
    pub const ILI9341_HBP: u16 = 20;
    /// Horizontal front porch
    pub const ILI9341_HFP: u16 = 10;
    /// Vertical synchronization
    pub const ILI9341_VSYNC: u16 = 2;
    /// Vertical back porch
    pub const ILI9341_VBP: u16 = 2;
    /// Vertical front porch
    pub const ILI9341_VFP: u16 = 4;

    ///
    ///  @brief  ILI9341 Registers
    ///

    /* Level 1 Commands */

    pub const LCD_SWRESET: u8 = 0x01; /* Software Reset */
    pub const LCD_READ_DISPLAY_ID: u8 = 0x04; /* Read display identification information */
    pub const LCD_RDDST: u8 = 0x09; /* Read Display Status */
    pub const LCD_RDDPM: u8 = 0x0a; /* Read Display Power Mode */
    pub const LCD_RDDMADCTL: u8 = 0x0b; /* Read Display MADCTL */
    pub const LCD_RDDCOLMOD: u8 = 0x0c; /* Read Display Pixel Format */
    pub const LCD_RDDIM: u8 = 0x0d; /* Read Display Image Format */
    pub const LCD_RDDSM: u8 = 0x0e; /* Read Display Signal Mode */
    pub const LCD_RDDSDR: u8 = 0x0f; /* Read Display Self-Diagnostic Result */
    pub const LCD_SPLIN: u8 = 0x10; /* Enter Sleep Mode */
    pub const LCD_SLEEP_OUT: u8 = 0x11; /* Sleep out register */
    pub const LCD_PTLON: u8 = 0x12; /* Partial Mode ON */
    pub const LCD_NORMAL_MODE_ON: u8 = 0x13; /* Normal Display Mode ON */
    pub const LCD_DINVOFF: u8 = 0x20; /* Display Inversion OFF */
    pub const LCD_DINVON: u8 = 0x21; /* Display Inversion ON */
    pub const LCD_GAMMA: u8 = 0x26; /* Gamma register */
    pub const LCD_DISPLAY_OFF: u8 = 0x28; /* Display off register */
    pub const LCD_DISPLAY_ON: u8 = 0x29; /* Display on register */
    pub const LCD_COLUMN_ADDR: u8 = 0x2a; /* Colomn address register */
    pub const LCD_PAGE_ADDR: u8 = 0x2b; /* Page address register */
    pub const LCD_GRAM: u8 = 0x2c; /* GRAM register */
    pub const LCD_RGBSET: u8 = 0x2d; /* Color SET */
    pub const LCD_RAMRD: u8 = 0x2e; /* Memory Read */
    pub const LCD_PLTAR: u8 = 0x30; /* Partial Area */
    pub const LCD_VSCRDEF: u8 = 0x33; /* Vertical Scrolling Definition */
    pub const LCD_TEOFF: u8 = 0x34; /* Tearing Effect Line OFF */
    pub const LCD_TEON: u8 = 0x35; /* Tearing Effect Line ON */
    pub const LCD_MAC: u8 = 0x36; /* Memory Access Control register*/
    pub const LCD_VSCRSADD: u8 = 0x37; /* Vertical Scrolling Start Address */
    pub const LCD_IDMOFF: u8 = 0x38; /* Idle Mode OFF */
    pub const LCD_IDMON: u8 = 0x39; /* Idle Mode ON */
    pub const LCD_PIXEL_FORMAT: u8 = 0x3a; /* Pixel Format register */
    pub const LCD_WRITE_MEM_CONTINUE: u8 = 0x3c; /* Write Memory Continue */
    pub const LCD_READ_MEM_CONTINUE: u8 = 0x3e; /* Read Memory Continue */
    pub const LCD_SET_TEAR_SCANLINE: u8 = 0x44; /* Set Tear Scanline */
    pub const LCD_GET_SCANLINE: u8 = 0x45; /* Get Scanline */
    pub const LCD_WDB: u8 = 0x51; /* Write Brightness Display register */
    pub const LCD_RDDISBV: u8 = 0x52; /* Read Display Brightness */
    pub const LCD_WCD: u8 = 0x53; /* Write Control Display register*/
    pub const LCD_RDCTRLD: u8 = 0x54; /* Read CTRL Display */
    pub const LCD_WRCABC: u8 = 0x55; /* Write Content Adaptive Brightness Control */
    pub const LCD_RDCABC: u8 = 0x56; /* Read Content Adaptive Brightness Control */
    pub const LCD_WRITE_CABC: u8 = 0x5e; /* Write CABC Minimum Brightness */
    pub const LCD_READ_CABC: u8 = 0x5f; /* Read CABC Minimum Brightness */
    pub const LCD_READ_ID1: u8 = 0xda; /* Read ID1 */
    pub const LCD_READ_ID2: u8 = 0xdb; /* Read ID2 */
    pub const LCD_READ_ID3: u8 = 0xdc; /* Read ID3 */

    /* Level 2 Commands */
    pub const LCD_RGB_INTERFACE: u8 = 0xb0; /* RGB Interface Signal Control */
    pub const LCD_FRMCTR1: u8 = 0xb1; /* Frame Rate Control (In Normal Mode) */
    pub const LCD_FRMCTR2: u8 = 0xb2; /* Frame Rate Control (In Idle Mode) */
    pub const LCD_FRMCTR3: u8 = 0xb3; /* Frame Rate Control (In Partial Mode) */
    pub const LCD_INVTR: u8 = 0xb4; /* Display Inversion Control */
    pub const LCD_BPC: u8 = 0xb5; /* Blanking Porch Control register */
    pub const LCD_DFC: u8 = 0xb6; /* Display Function Control register */
    pub const LCD_ETMOD: u8 = 0xb7; /* Entry Mode Set */
    pub const LCD_BACKLIGHT1: u8 = 0xb8; /* Backlight Control 1 */
    pub const LCD_BACKLIGHT2: u8 = 0xb9; /* Backlight Control 2 */
    pub const LCD_BACKLIGHT3: u8 = 0xba; /* Backlight Control 3 */
    pub const LCD_BACKLIGHT4: u8 = 0xbb; /* Backlight Control 4 */
    pub const LCD_BACKLIGHT5: u8 = 0xbc; /* Backlight Control 5 */
    pub const LCD_BACKLIGHT7: u8 = 0xbe; /* Backlight Control 7 */
    pub const LCD_BACKLIGHT8: u8 = 0xbf; /* Backlight Control 8 */
    pub const LCD_POWER1: u8 = 0xc0; /* Power Control 1 register */
    pub const LCD_POWER2: u8 = 0xc1; /* Power Control 2 register */
    pub const LCD_VCOM1: u8 = 0xc5; /* VCOM Control 1 register */
    pub const LCD_VCOM2: u8 = 0xc7; /* VCOM Control 2 register */
    pub const LCD_NVMWR: u8 = 0xd0; /* NV Memory Write */
    pub const LCD_NVMPKEY: u8 = 0xd1; /* NV Memory Protection Key */
    pub const LCD_RDNVM: u8 = 0xd2; /* NV Memory Status Read */
    pub const LCD_READ_ID4: u8 = 0xd3; /* Read ID4 */
    pub const LCD_PGAMMA: u8 = 0xe0; /* Positive Gamma Correction register */
    pub const LCD_NGAMMA: u8 = 0xe1; /* Negative Gamma Correction register */
    pub const LCD_DGAMCTRL1: u8 = 0xe2; /* Digital Gamma Control 1 */
    pub const LCD_DGAMCTRL2: u8 = 0xe3; /* Digital Gamma Control 2 */
    pub const LCD_INTERFACE: u8 = 0xf6; /* Interface control register */

    /* Extend register commands */
    pub const LCD_POWERA: u8 = 0xcb; /* Power control A register */
    pub const LCD_POWERB: u8 = 0xcf; /* Power control B register */
    pub const LCD_DTCA: u8 = 0xe8; /* Driver timing control A */
    pub const LCD_DTCB: u8 = 0xea; /* Driver timing control B */
    pub const LCD_POWER_SEQ: u8 = 0xed; /* Power on sequence register */
    pub const LCD_3GAMMA_EN: u8 = 0xf2; /* 3 Gamma enable register */
    pub const LCD_PRC: u8 = 0xf7; /* Pump ratio control register */

    pub fn init(mut lcd: LCD) -> Self {
        info!("Init ili9341 panel");

        lcd.write_reg(0xca);
        lcd.write_data(0xc3);
        lcd.write_data(0x08);
        lcd.write_data(0x50);
        lcd.write_reg(Self::LCD_POWERB);
        lcd.write_data(0x00);
        lcd.write_data(0xc1);
        lcd.write_data(0x30);
        lcd.write_reg(Self::LCD_POWER_SEQ);
        lcd.write_data(0x64);
        lcd.write_data(0x03);
        lcd.write_data(0x12);
        lcd.write_data(0x81);
        lcd.write_reg(Self::LCD_DTCA);
        lcd.write_data(0x85);
        lcd.write_data(0x00);
        lcd.write_data(0x78);
        lcd.write_reg(Self::LCD_POWERA);
        lcd.write_data(0x39);
        lcd.write_data(0x2c);
        lcd.write_data(0x00);
        lcd.write_data(0x34);
        lcd.write_data(0x02);
        lcd.write_reg(Self::LCD_PRC);
        lcd.write_data(0x20);
        lcd.write_reg(Self::LCD_DTCB);
        lcd.write_data(0x00);
        lcd.write_data(0x00);
        lcd.write_reg(Self::LCD_FRMCTR1);
        lcd.write_data(0x00);
        lcd.write_data(0x1b);
        lcd.write_reg(Self::LCD_DFC);
        lcd.write_data(0x0a);
        lcd.write_data(0xa2);
        lcd.write_reg(Self::LCD_POWER1);
        lcd.write_data(0x10);
        lcd.write_reg(Self::LCD_POWER2);
        lcd.write_data(0x10);
        lcd.write_reg(Self::LCD_VCOM1);
        lcd.write_data(0x45);
        lcd.write_data(0x15);
        lcd.write_reg(Self::LCD_VCOM2);
        lcd.write_data(0x90);
        lcd.write_reg(Self::LCD_MAC);
        lcd.write_data(0xc8);
        lcd.write_reg(Self::LCD_3GAMMA_EN);
        lcd.write_data(0x00);
        lcd.write_reg(Self::LCD_RGB_INTERFACE);
        lcd.write_data(0xc2);
        lcd.write_reg(Self::LCD_DFC);
        lcd.write_data(0x0a);
        lcd.write_data(0xa7);
        lcd.write_data(0x27);
        lcd.write_data(0x04);

        /* Colomn address set */
        lcd.write_reg(Self::LCD_COLUMN_ADDR);
        lcd.write_data(0x00);
        lcd.write_data(0x00);
        lcd.write_data(0x00);
        lcd.write_data(0xef);
        /* Page address set */
        lcd.write_reg(Self::LCD_PAGE_ADDR);
        lcd.write_data(0x00);
        lcd.write_data(0x00);
        lcd.write_data(0x01);
        lcd.write_data(0x3f);
        lcd.write_reg(Self::LCD_INTERFACE);
        lcd.write_data(0x01);
        lcd.write_data(0x00);
        lcd.write_data(0x06);

        lcd.write_reg(Self::LCD_GRAM);
        // LCD_Delay(200);
        for _ in 0..100_000 {
        }

        lcd.write_reg(Self::LCD_GAMMA);
        lcd.write_data(0x01);

        lcd.write_reg(Self::LCD_PGAMMA);
        lcd.write_data(0x0f);
        lcd.write_data(0x29);
        lcd.write_data(0x24);
        lcd.write_data(0x0c);
        lcd.write_data(0x0e);
        lcd.write_data(0x09);
        lcd.write_data(0x4e);
        lcd.write_data(0x78);
        lcd.write_data(0x3c);
        lcd.write_data(0x09);
        lcd.write_data(0x13);
        lcd.write_data(0x05);
        lcd.write_data(0x17);
        lcd.write_data(0x11);
        lcd.write_data(0x00);
        lcd.write_reg(Self::LCD_NGAMMA);
        lcd.write_data(0x00);
        lcd.write_data(0x16);
        lcd.write_data(0x1b);
        lcd.write_data(0x04);
        lcd.write_data(0x11);
        lcd.write_data(0x07);
        lcd.write_data(0x31);
        lcd.write_data(0x33);
        lcd.write_data(0x42);
        lcd.write_data(0x05);
        lcd.write_data(0x0c);
        lcd.write_data(0x0a);
        lcd.write_data(0x28);
        lcd.write_data(0x2f);
        lcd.write_data(0x0f);

        lcd.write_reg(Self::LCD_SLEEP_OUT);
        // LCD_Delay(200);
        for _ in 0..100_000 {
        }

        lcd.write_reg(Self::LCD_DISPLAY_ON);
        /* GRAM start writing */
        lcd.write_reg(Self::LCD_GRAM);

        Self { lcd }
    }

    pub fn on(&mut self) {
        self.lcd.write_reg(Self::LCD_DISPLAY_ON);
    }

    pub fn off(&mut self) {
        self.lcd.write_reg(Self::LCD_DISPLAY_OFF);
    }
}