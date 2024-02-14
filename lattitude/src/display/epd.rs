use crate::display::Display;
use it8951::interface::IT8951SPIInterface;
use it8951::memory_converter_settings::MemoryConverterSetting;
use it8951::{memory_converter_settings, AreaImgInfo, Run, IT8951};
use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
use linux_embedded_hal::spidev::{SpiModeFlags, Spidev, SpidevOptions};
use linux_embedded_hal::{CdevPin, Delay};
use memory_converter_settings::{MemoryConverterBitPerPixel, MemoryConverterEndianness};
use pixelfield::color::Gray4;
use pixelfield::pixelfield::PixelField;

pub struct Epd103VerticalDisplay {
    pub epd: IT8951<IT8951SPIInterface<Spidev, CdevPin, CdevPin, Delay>, Run>,
}

impl Epd103VerticalDisplay {
    pub const WIDTH: usize = 1404;
    pub const HEIGHT: usize = 1872;

    pub fn new(vcom: u16, hz: u32) -> Self {
        let mut spi = Spidev::open("/dev/spidev0.0").expect("open spi");
        let spi_options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(hz)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&spi_options).expect("configure spi");

        let mut chip = Chip::new("/dev/gpiochip0").expect("open GPIO");
        // RST: 17
        let rst_output = chip.get_line(17).expect("line 17: rst output");
        let rst_output_handle = rst_output
            .request(LineRequestFlags::OUTPUT, 0, "meeting-room")
            .expect("line 17: rst handle");
        let rst = CdevPin::new(rst_output_handle).expect("line 17: rst");
        // BUSY / HDRY: 24
        let busy_input = chip.get_line(24).expect("line 24: busy input");
        let busy_input_handle = busy_input
            .request(LineRequestFlags::INPUT, 0, "meeting-room")
            .expect("line 24: busy handle");
        let busy = CdevPin::new(busy_input_handle).expect("line 24: busy");

        let driver = IT8951SPIInterface::new(spi, busy, rst, Delay);
        let mut epd = it8951::IT8951::new(driver).init(vcom).unwrap();

        Self { epd }
    }
}

impl Display for Epd103VerticalDisplay {
    fn display(&mut self, pixel_field: &PixelField) {
        // for each row
        for y in 0..Self::HEIGHT {
            // chunk 1 row at a time
            let mut data = [0; Self::WIDTH / 4];
            let mut cur = 0;
            for x in 0..Self::WIDTH {
                let luma = if let Some(color) = pixel_field.get((x, y)) {
                    color.luma()
                } else {
                    Gray4::White.luma()
                };

                data[cur] = data[cur] | ((luma as u16) << ((x % 4) * 4));
                if x % 4 == 3 {
                    cur += 1
                }
            }
            if let Err(err) = self.epd.load_image_area(
                self.epd.get_dev_info().memory_address,
                MemoryConverterSetting {
                    endianness: MemoryConverterEndianness::LittleEndian,
                    bit_per_pixel: MemoryConverterBitPerPixel::BitsPerPixel4,
                    rotation: memory_converter_settings::MemoryConverterRotation::Rotate270,
                },
                &AreaImgInfo {
                    area_x: 0,
                    area_y: y as u16,
                    area_w: Self::WIDTH as u16,
                    area_h: 1,
                },
                &data,
            ) {
                println!("{:#?}", err);
            }
        }

        self.epd
            .display(it8951::WaveformMode::GrayscaleClearing16)
            .unwrap();
    }
}
