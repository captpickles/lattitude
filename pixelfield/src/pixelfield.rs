use crate::color::Color;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::ops::{Mul, Range};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<(u32, u32)> for Point {
    fn from((x, y): (u32, u32)) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub nw: Point,
    pub se: Point,
}

impl Rectangle {
    pub fn contains<P: Into<Point>>(&self, point: P) -> bool {
        let point = point.into();

        point.x >= self.nw.x && point.x <= self.se.x && point.y >= self.nw.y && point.y <= self.se.y
    }

    pub fn dimensions(&self) -> Dimensions {
        (*self).into()
    }

    pub fn x_range(&self) -> Range<i32> {
        self.nw.x..self.se.x
    }

    pub fn y_range(&self) -> Range<i32> {
        self.nw.y..self.se.y
    }

    pub fn bounding_square(&self) -> Rectangle {
        let dimensions = self.dimensions();
        match dimensions.width.cmp(&dimensions.height) {
            Ordering::Less => {
                Rectangle {
                    nw: self.nw,
                    se: Point {
                        x: self.nw.x + dimensions.height as i32,
                        y: self.nw.y + dimensions.height as i32,
                    },
                }
            },
            Ordering::Equal => {
                *self
            },
            Ordering::Greater => {
                Rectangle {
                    nw: self.nw,
                    se: Point {
                        x: self.nw.y + dimensions.width as i32,
                        y: self.nw.y + dimensions.width as i32,
                    },
                }
            },
        }
    }

    pub fn center_point(&self) -> Point {
        let dimensions = self.dimensions();

        Point {
            x: self.nw.x + ((dimensions.width as i32 - 1) / 2),
            y: self.nw.y + ((dimensions.height as i32 - 1) / 2),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Dimensions {
    width: u32,
    height: u32,
}

impl Dimensions {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Mul<f32> for Dimensions {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Dimensions {
            width: (self.width as f32 * rhs) as u32,
            height: (self.height as f32 * rhs) as u32,
        }
    }
}

impl From<Rectangle> for Dimensions {
    fn from(rect: Rectangle) -> Self {
        let width = rect.se.x - rect.nw.x;
        let height = rect.se.y - rect.nw.y;

        (width as u32, height as u32).into()
    }
}

impl From<(u32, u32)> for Dimensions {
    fn from((width, height): (u32, u32)) -> Self {
        Dimensions { width, height }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Rotation {
    Clockwise(f32),
    CounterClockwise(f32),
}

impl Rotation {
    pub fn clockwise(degrees: f32) -> Self {
        Self::Clockwise(degrees)
    }

    pub fn counter_clockwise(degrees: f32) -> Self {
        Self::Clockwise(degrees)
    }
}

impl Rotation {
    pub fn as_degrees(&self) -> f32 {
        match self {
            Rotation::Clockwise(degrees) => *degrees,
            Rotation::CounterClockwise(degrees) => *degrees + -1.0,
        }
    }

    pub fn as_radians(&self) -> f32 {
        (self.as_degrees() * PI) / 180.0
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    point: Point,
    color: Color,
}

impl Pixel {
    pub fn point(&self) -> Point {
        self.point
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

#[derive(Clone)]
pub struct PixelField {
    //pixels: Vec<Pixel>,
    pixels: HashMap<Point, Color>
}

impl Default for PixelField {
    fn default() -> Self {
        //Self { pixels: vec![] }
        Self {
            pixels: HashMap::default()
        }
    }
}

impl PixelField {
    pub fn iter(&self) -> impl Iterator<Item = Pixel> + '_ {
        self.pixels.iter().map(|(point, color)| {
            Pixel {
                point:  *point,
                color: *color,
            }
        })
    }

    pub fn len(&self) -> usize {
        self.pixels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pixels.is_empty()
    }

    pub fn set<P: Into<Point>>(&mut self, point: P, color: Color) {
        let point = point.into();
        self.pixels.insert(point, color);
    }

    pub fn get<P: Into<Point>>(&self, point: P) -> Option<Color> {
        let point = point.into();
        self.pixels.get(&point).cloned()
    }


    pub fn bounding_box(&self) -> Rectangle {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;

        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for point in self.pixels.keys() {
            if point.x < min_x {
                min_x = point.x;
            }
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y < min_y {
                min_y = point.y;
            }
            if point.y > max_y {
                max_y = point.y;
            }
        }

        Rectangle {
            nw: (min_x, min_y).into(),
            se: (max_x, max_y).into(),
        }
    }

    pub fn dimensions(&self) -> Dimensions {
        self.bounding_box().dimensions()
    }

    pub fn rotate(&self, rotation: Rotation) -> PixelField {
        let radians = rotation.as_radians();

        // The bbox of the original image
        let original_bbox = self.bounding_box();

        // inflated to be a square to allow rotation
        let rotated_bbox = original_bbox.bounding_square();
        println!("rotated bbox : {:?}", rotated_bbox);

        let mut rotated = PixelField::default();

        let cos = radians.cos();
        let sin = radians.sin();

        // center of the original
        let center = original_bbox.center_point();

        //let original_dimensions = original_bbox.dimensions();
        let rotated_dimensions = rotated_bbox.dimensions();

        println!("rotated dims : {:?}", rotated_dimensions);


        for x in rotated_bbox.nw.x..=rotated_bbox.se.x {
            for y in rotated_bbox.nw.y..=rotated_bbox.se.y {
                let ox = (cos * (x as f32 - center.x as f32)
                    + (sin * (y as f32 - center.y as f32))
                    + rotated_dimensions.width() as f32 / 2.0) as u32;
                let oy = (cos * (y as f32 - center.y as f32) - (sin * (x as f32 - center.x as f32))
                    + rotated_dimensions.height() as f32 / 2.0) as u32;

                if let Some(pixel) = self.get((ox, oy)) {
                    rotated.set((x, y), pixel);
                }
            }
        }

        rotated
    }

    pub fn scale(&self, scale: f32) -> PixelField {
        let original_bbox = self.bounding_box();
        let scaled_dimensions = original_bbox.dimensions() * scale;

        let mut scaled = PixelField::default();

        for x in original_bbox.nw.x..(original_bbox.nw.x + scaled_dimensions.width() as i32) {
            for y in original_bbox.nw.y..(original_bbox.nw.y + scaled_dimensions.height() as i32) {
                let scaled_x = (x as f32 * 1.0 / scale) as u32;
                let scaled_y = (y as f32 * 1.0 / scale) as u32;

                if let Some(pixel) = self.get((scaled_x, scaled_y)) {
                    scaled.set((x, y), pixel);
                }
            }
        }

        scaled
    }

    pub fn trim(&self, background: Color) -> PixelField {
        let original_bbox = self.bounding_box();

        let mut nw = original_bbox.nw;
        let mut se = original_bbox.se;

        'outer: for x in original_bbox.x_range() {
            for y in original_bbox.y_range() {
                if let Some(color) = self.get((x, y)) {
                    if color != background {
                        nw.y = y;
                        break 'outer;
                    }
                }
            }
        }

        'outer: for x in original_bbox.x_range().rev() {
            for y in original_bbox.y_range() {
                if let Some(color) = self.get((x, y)) {
                    if color != background {
                        se.y = y;
                        break 'outer;
                    }
                }
            }
        }

        'outer: for y in original_bbox.y_range() {
            for x in original_bbox.x_range() {
                if let Some(color) = self.get((x, y)) {
                    if color != background {
                        nw.x = x;
                        break 'outer;
                    }
                }
            }
        }

        'outer: for y in original_bbox.y_range().rev() {
            for x in original_bbox.x_range() {
                if let Some(color) = self.get((x, y)) {
                    if color != background {
                        se.x = x;
                        break 'outer;
                    }
                }
            }
        }

        let trimmed_bbox = Rectangle { nw, se };

        PixelField {
            pixels: self
                .pixels
                .iter()
                .filter(|(point, _color)| trimmed_bbox.contains(**point))
                .map(|(point, color)| {
                    (*point, *color)
                })
                .collect(),
        }
    }

    pub fn to_bmp(&self) -> bmp::Image {
        let bbox = self.bounding_box();

        let x_adjustment = if bbox.x_range().start < 0 {
            bbox.x_range().start.abs()
        } else {
            0
        };

        let y_adjustment = if bbox.y_range().start < 0 {
            bbox.y_range().start.abs()
        } else {
            0
        };

        let mut bmp = bmp::Image::new((bbox.x_range().end + x_adjustment + 1) as u32, (bbox.y_range().end + y_adjustment + 1) as u32);

        for x in 0..bmp.get_width() {
            for y in 0..bmp.get_height() {
                bmp.set_pixel(x, y, bmp::Pixel {
                    r: 255,
                    g: 255,
                    b: 255,
                });

            }

        }

        for pixel in self.iter() {
            bmp.set_pixel(
                (pixel.point.x + x_adjustment) as u32,
                (pixel.point.y + y_adjustment) as u32,
                pixel.color.into(),
            )
        }

        bmp
    }
}


#[cfg(test)]
mod test {
    use crate::pixelfield::Rectangle;

    #[test]
    fn origin_bbox_to_dimensions() {
        let rect = Rectangle {
            nw: (0,0).into(),
            se: (100,200).into(),
        };

        let dims = rect.dimensions();

        assert_eq!(100, dims.width);
        assert_eq!(200, dims.height);
    }

    #[test]
    fn origin_bbox_to_center() {
        let rect = Rectangle {
            nw: (0,0).into(),
            se: (100,200).into(),
        };

        let point = rect.center_point();

        assert_eq!(49, point.x);
        assert_eq!(99, point.y);
    }

    #[test]
    fn offset_bbox_to_dimensions() {
        let rect = Rectangle {
            nw: (20,20).into(),
            se: (100,200).into(),
        };

        let dims = rect.dimensions();

        assert_eq!(80, dims.width);
        assert_eq!(180, dims.height);
    }

    #[test]
    fn bounding_square() {

        let rect = Rectangle {
            nw: (8,6).into(),
            se: (493, 387).into()
        };

        let square = rect.bounding_square();

        println!("{:#?}",square.dimensions());

    }

}