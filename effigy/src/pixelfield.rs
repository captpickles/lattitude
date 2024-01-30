use std::f32::consts::PI;
use std::ops::{Mul, Range};
use crate::color::Color;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self {
            x,
            y,
        }
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
    nw: Point,
    se: Point,
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
        if dimensions.width > dimensions.height {
            Rectangle {
                nw: self.nw,
                se: Point {
                    x: self.nw.y + dimensions.width as i32,
                    y: self.nw.y + dimensions.height as i32,
                },
            }
        } else if dimensions.height > dimensions.width {
            Rectangle {
                nw: self.nw,
                se: Point {
                    x: self.nw.x + dimensions.height as i32,
                    y: self.nw.y + dimensions.height as i32,
                },
            }
        } else {
            *self
        }
    }

    pub fn center_point(&self) -> Point {
        let dimensions = self.dimensions();

        Point {
            x: self.se.x + ((dimensions.width as i32 - 1) / 2),
            y: self.se.y + ((dimensions.height as i32 - 1) / 2),
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
        Dimensions {
            width,
            height,
        }
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
            Rotation::Clockwise(degrees) => {
                *degrees
            }
            Rotation::CounterClockwise(degrees) => {
                *degrees + -1.0
            }
        }
    }

    pub fn as_radians(&self) -> f32 {
        (self.as_degrees() * PI) / 180.0
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Pixel<C: Color> {
    point: Point,
    color: C,
}

impl<C:Color> Pixel<C> {
    pub fn point(&self) -> Point {
        self.point
    }

    pub fn color(&self) -> C {
        self.color
    }

}

#[derive(Clone)]
pub struct PixelField<C: Color> {
    pixels: Vec<Pixel<C>>,
}

impl<C: Color> Default for PixelField<C> {
    fn default() -> Self {
        Self {
            pixels: vec![],
        }
    }
}

impl<C: Color> PixelField<C> {

    pub fn iter(&self) -> impl Iterator<Item=&Pixel<C>> {
        self.pixels.iter()
    }

    pub fn len(&self) -> usize {
        self.pixels.len()
    }

    pub fn set<P: Into<Point>>(&mut self, point: P, color: C) {
        let point = point.into();
        if let Some(pixel) = self.find_pixel_mut(point) {
            pixel.color = color;
        } else {
            self.pixels.push(
                Pixel {
                    point,
                    color,
                }
            )
        }
    }

    pub fn get<P: Into<Point>>(&self, point: P) -> Option<C> {
        self.find_pixel(point).map(|inner| inner.color)
    }

    fn find_pixel<P: Into<Point>>(&self, point: P) -> Option<Pixel<C>> {
        let point = point.into();
        self.pixels.iter().find(|e| {
            e.point == point
        }).cloned()
    }

    fn find_pixel_mut<P: Into<Point>>(&mut self, point: P) -> Option<&mut Pixel<C>> {
        let point = point.into();
        self.pixels.iter_mut().find(|e| {
            e.point == point
        })
    }

    pub fn bounding_box(&self) -> Rectangle {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;

        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for pixel in &self.pixels {
            if pixel.point.x < min_x {
                min_x = pixel.point.x
            }
            if pixel.point.x > max_x {
                max_x = pixel.point.x
            }
            if pixel.point.y < min_y {
                min_y = pixel.point.y
            }
            if pixel.point.y > max_y {
                max_y = pixel.point.y
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

    pub fn rotate(&self, rotation: Rotation) -> PixelField<C> {
        let radians = rotation.as_radians();

        // The bbox of the original image
        let original_bbox = self.bounding_box();

        // inflated to be a square to allow rotation
        let scaled_bbox = original_bbox.bounding_square();

        let mut rotated = PixelField::<C>::default();

        let cos = radians.cos();
        let sin = radians.sin();

        // center of the original
        let center = original_bbox.center_point();

        //let original_dimensions = original_bbox.dimensions();
        let rotated_dimensions = scaled_bbox.dimensions();

        for x in scaled_bbox.nw.x..scaled_bbox.se.x {
            for y in scaled_bbox.nw.y..scaled_bbox.se.y {
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

    pub fn scale(&self, scale: f32) -> PixelField<C> {
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

    pub fn trim(&self, background: C) -> PixelField<C> {
        let original_bbox = self.bounding_box();

        let mut nw = original_bbox.nw.clone();
        let mut se = original_bbox.se.clone();

        'outer:
        for x in original_bbox.x_range() {
            for y in original_bbox.y_range() {
                if let Some(color) = self.get((x, y)) {
                    if color != background {
                        nw.y = y;
                        break 'outer;
                    }
                }
            }
        }

        'outer:
        for x in original_bbox.x_range().rev() {
            for y in original_bbox.y_range() {
                if let Some(color) = self.get((x, y)) {
                    if color != background {
                        se.y = y;
                        break 'outer;
                    }
                }
            }
        }

        'outer:
        for y in original_bbox.y_range() {
            for x in original_bbox.x_range() {
                if let Some(color) = self.get((x, y)) {
                    if color != background {
                        nw.x = x;
                        break 'outer;
                    }
                }
            }
        }

        'outer:
        for y in original_bbox.y_range().rev() {
            for x in original_bbox.x_range() {
                if let Some(color) = self.get((x, y)) {
                    if color != background {
                        se.x = x;
                        break 'outer;
                    }
                }
            }
        }

        let trimmed_bbox = Rectangle {
            nw,
            se,
        };

        PixelField {
            pixels: self.pixels.iter().filter(|e| {
                trimmed_bbox.contains(e.point)
            })
                .cloned()
                .collect()
        }
    }
}