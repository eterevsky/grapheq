use image::{ImageBuffer, Pixel};

pub struct Style<P>
where
    P: Pixel + 'static,
    P::Subpixel: 'static,
{
    // The window in graph coordinates which will be shown. The actual viewport may be wider or taller if the aspect ratio of the image in pixel doesn't match the aspect ratio (xmax - xmin):(ymax - ymin)
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,

    // Width and height of image in pixels.
    wpixels: u32,
    hpixels: u32,

    // Foreground color
    fg: P,
    // Background color
    bg: P,

    // Width of the line in the graph coordinates.
    width: f64,

    // Coefficients for translation from pixel coordinates to the graph coordinates
    scale: f64,
    xoffset: f64,
    yoffset: f64,
}

impl<P> Style<P>
where
    P: Pixel<Subpixel = u8> + 'static,
{
    pub fn new() -> Self {
        let mut style = Style {
            xmin: -1.0,
            xmax: 1.0,
            ymin: -1.0,
            ymax: 1.0,
            wpixels: 1920,
            hpixels: 1080,
            fg: P::from_channels(0, 0, 0, 255),
            bg: P::from_channels(255, 255, 255, 255),
            scale: 1.,
            width: 0.001,
            xoffset: 0.,
            yoffset: 0.,
        };
        style.update_conversion();
        style
    }

    // Update the coefficients for conversion between pixel coordinates and graph coordinates.
    fn update_conversion(&mut self) {
        self.scale = ((self.xmax - self.xmin) / self.wpixels as f64)
            .max((self.ymax - self.ymin) / self.hpixels as f64);

        let cx = (self.xmin + self.xmax) / 2.;
        let cy = (self.ymin + self.ymax) / 2.;

        let px = self.wpixels as f64 / 2.;
        let py = self.hpixels as f64 / 2.;

        self.xoffset = cx - px * self.scale;
        self.yoffset = cy - py * self.scale;
    }

    fn pixel_to_coords(&self, x: f64, y: f64) -> (f64, f64) {
        (self.scale * x + self.xoffset, self.scale * y + self.yoffset)
    }

    pub fn set_xmin(mut self, v: f64) -> Self {
        self.xmin = v;
        self.update_conversion();
        self
    }
    pub fn set_xmax(mut self, v: f64) -> Self {
        self.xmax = v;
        self.update_conversion();
        self
    }
    pub fn set_ymin(mut self, v: f64) -> Self {
        self.ymin = v;
        self.update_conversion();
        self
    }
    pub fn set_ymax(mut self, v: f64) -> Self {
        self.ymax = v;
        self.update_conversion();
        self
    }
    pub fn set_width(mut self, v: f64) -> Self {
        self.width = v;
        self
    }
    pub fn set_image_width(mut self, v: u32) -> Self {
        self.wpixels = v;
        self.update_conversion();
        self
    }
    pub fn set_image_height(mut self, v: u32) -> Self {
        self.hpixels = v;
        self.update_conversion();
        self
    }
}

pub fn plot<F, G, P>(f: &F, gradient: &G, style: &Style<P>) -> ImageBuffer<P, Vec<u8>>
where
    F: Fn(f64, f64) -> f64,
    G: Fn(f64, f64) -> (f64, f64),
    P: Pixel<Subpixel = u8> + 'static,
{
    let mut image = ImageBuffer::new(style.wpixels, style.hpixels);

    for (px, py, pixel) in image.enumerate_pixels_mut() {
        let px = px as f64;
        let py = py as f64;

        // TODO: Add a heuristic to skip subpixel iterations when we are far away from any roots (gradient is big, and second derivatives are low).

        let mut count = 0;
        for dx in 0..16 {
            for dy in 0..16 {
                let (x, y) = style.pixel_to_coords(px + dx as f64 / 16., py + dy as f64 / 16.);
                let (gx, gy) = gradient(x, y);
                let gradient_abs = (gx * gx + gy * gy).sqrt();
                if f(x, y).abs() < style.width * gradient_abs {
                    count += 1;
                }
            }
        }

        let mut pixel_color = style.bg;
        for i in 0..P::channel_count() {
            let bg = style.bg.channels()[i as usize] as f64;
            let fg = style.fg.channels()[i as usize] as f64;
            pixel_color.channels_mut()[i as usize] = (bg + (fg - bg) * (count as f64 / 256.)) as u8;
        }
        *pixel = pixel_color;
    }
    image
}
