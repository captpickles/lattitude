use std::path::PathBuf;
use pixelfield::color::{BlackAndWhite, Color, Gray4};
use pixelfield::pixelfield::PixelField;
use engine::display::Display;

const N: char = '.';
const B: char = '#';
const W: char = '_';

const PIXEL_INTENSITY: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

pub struct ASCIIDisplay<const WIDTH: u32, const HEIGHT: u32> {
    data: String,
}

fn to_ascii(color: Color) -> char {
    let index = ((PIXEL_INTENSITY.len() - 1) as f32 * color.luma() as f32 / 255.0).ceil() as usize;
    PIXEL_INTENSITY.chars().nth(index).unwrap()
}

impl<const WIDTH: u32, const HEIGHT: u32> ASCIIDisplay<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        let mut data = String::new();
        data.reserve((WIDTH * HEIGHT) as usize);
        Self { data }
    }

    pub fn to_string(&self) -> String {
        let mut lines = Vec::new();

        for chunk in self.data.chars().collect::<Vec<char>>().chunks(WIDTH as usize) {
            lines.push(String::from_iter(chunk));
        }

        let mut result = lines.join("\n");
        result.push_str("\n");

        result
    }
}

impl<const WIDTH: u32, const HEIGHT: u32> Display for ASCIIDisplay<WIDTH, HEIGHT> {
    fn display(&mut self, pixel_field: &PixelField) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = y * WIDTH + x;
                let dot = pixel_field.get((x, y)).map_or(N, |p| to_ascii(p));
                self.data.insert(index as usize, dot);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use engine::display::Display;
    use engine::model::ModelManager;
    use engine::view::canvas::{Canvas, Component};
    use engine::view::{HorizontalAlignment, Renderable, VerticalAlignment};
    use engine::view::text::{Source, Text};
    use pixelfield::color::BlackAndWhite::Black;
    use pixelfield::color::Color;
    use pixelfield::pixelfield::PixelField;
    use crate::display::ascii::{ASCIIDisplay, B, N};

    use crate::font::{build_font_registry, Font};

    #[test]
    fn ascii_display() {
        let mut displayer = ASCIIDisplay::<3, 3>::new();
        let mut pixel_field = PixelField::default();

        pixel_field.set((0, 0), Color::Binary(Black));
        pixel_field.set((1, 1), Color::Binary(Black));
        pixel_field.set((2, 2), Color::Binary(Black));

        displayer.display(&pixel_field);
        let s = displayer.to_string();
        print!("{}", s);

        let expected: String = [
            B, N, N,
            N, B, N,
            N, N, B].iter().collect();

        assert_eq!(expected, displayer.data);
    }

    #[tokio::test]
    async fn ascii_font_display() {
        let font_registry = build_font_registry().unwrap();
        let mut displayer = ASCIIDisplay::<200, 60>::new();


        let component = Component {
            point: (10, 20).into(),
            horizontal_alignment: HorizontalAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
            renderable: Box::new(Text::new(
                200,
                font_registry.get(Font::Typewriter),
                20.0,
                Source::Static("Låttitüdé".to_string()),
            )),
        };


        let mut manager = ModelManager::default();
        let mut pixel_field = PixelField::default();
        let rendering = component.render(&manager, &mut pixel_field).await;

        println!("BBOX: {:?}", &pixel_field.bounding_box());
        displayer.display(&pixel_field);
        print!("{}", displayer.to_string())
    }

}