use effigy::color::Color;

pub struct InvalidSize;

pub trait DrawTarget<C: Color> {
    fn split_horizontal(&self, height: u32) -> Result<(Region<C>, Region<C>), InvalidSize>;
    fn split_vertical(&self, width: u32) -> Result<(Region<C>, Region<C>), InvalidSize>;
}

#[derive(Clone)]
pub struct Canvas<C: Color> {
    width: u32,
    height: u32,
    pixels: Vec<Pixel<C>>,
    default: C,
}

impl<C: Color> DrawTarget<C> for Canvas<C> {
    fn split_horizontal(&self, height: u32) -> Result<(Region<C>, Region<C>), InvalidSize> {
        if height >= self.height {
            return Err(InvalidSize);
        }

        Ok((
            Region {
                canvas: self,
                x: 0,
                y: 0,
                width: self.width,
                height,
                default: self.default,
            },
            Region {
                canvas: self,
                x: 0,
                y: height,
                width: self.width,
                height: self.height - height,
                default: self.default,
            }
        ))
    }

    fn split_vertical(&self, width: u32) -> Result<(Region<C>, Region<C>), InvalidSize> {
        todo!()
    }
}

impl<C: Color> Canvas<C> {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![],
            default: C::WHITE,
        }
    }
}

pub struct Region<'c, C: Color> {
    canvas: &'c Canvas<C>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    default: C,
}

impl<'c, C: Color> DrawTarget<C> for Region<'c, C> {
    fn split_horizontal(&self, height: u32) -> Result<(Region<C>, Region<C>), InvalidSize> {
        if height >= self.height {
            return Err(InvalidSize);
        }

        Ok(
            (
                Region {
                    canvas: self.canvas,
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height,
                    default: self.default,
                },
                Region {
                    canvas: self.canvas,
                    x: self.x,
                    y: self.y + height,
                    width: self.width,
                    height: self.height - height,
                    default: self.default,
                }
            )
        )
    }

    fn split_vertical(&self, width: u32) -> Result<(Region<C>, Region<C>), InvalidSize> {
        todo!()
    }
}
