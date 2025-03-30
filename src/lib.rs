use wasm_bindgen::prelude::*;
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgba, imageops::FilterType};
use rayon::prelude::*;
use std::io::Cursor;

#[wasm_bindgen]
pub struct ImageProcessor {
    img: DynamicImage,
}

#[wasm_bindgen]
impl ImageProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &[u8]) -> Result<ImageProcessor, JsValue> {
        let img = image::load_from_memory(data)
            .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;
        Ok(ImageProcessor { img })
    }

    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.img.to_rgba8().into_raw()
    }

    #[wasm_bindgen]
    pub fn width(&self) -> u32 {
        self.img.width()
    }

    #[wasm_bindgen]
    pub fn height(&self) -> u32 {
        self.img.height()
    }

    #[wasm_bindgen]
    pub fn grayscale(&mut self) {
        self.img = self.img.grayscale();
    }

    #[wasm_bindgen]
    pub fn rotate90(&mut self) {
        self.img = self.img.rotate90();
    }

    #[wasm_bindgen]
    pub fn flip_vertical(&mut self) {
        self.img = self.img.flipv();
    }

    #[wasm_bindgen]
    pub fn flip_horizontal(&mut self) {
        self.img = self.img.fliph();
    }

    #[wasm_bindgen]
    pub fn adjust_brightness(&mut self, value: i32) {
        self.img = self.img.brighten(value);
    }

    #[wasm_bindgen]
    pub fn adjust_contrast(&mut self, contrast: f32) {
        let img_rgba = self.img.to_rgba8();
        let mut output = ImageBuffer::new(img_rgba.width(), img_rgba.height());
        
        for (x, y, pixel) in img_rgba.enumerate_pixels() {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;
            
            let r = ((r - 0.5) * contrast + 0.5).clamp(0.0, 1.0) * 255.0;
            let g = ((g - 0.5) * contrast + 0.5).clamp(0.0, 1.0) * 255.0;
            let b = ((b - 0.5) * contrast + 0.5).clamp(0.0, 1.0) * 255.0;
            
            output.put_pixel(x, y, Rgba([
                r as u8,
                g as u8,
                b as u8,
                pixel[3]
            ]));
        }
        
        self.img = DynamicImage::ImageRgba8(output);
    }

    #[wasm_bindgen]
    pub fn blur(&mut self, sigma: f32) {
        self.img = self.img.blur(sigma);
    }

    #[wasm_bindgen]
    pub fn sharpen(&mut self, sigma: f32, threshold: i32) {
        let img_rgba = self.img.to_rgba8();
        let mut output = ImageBuffer::new(img_rgba.width(), img_rgba.height());
        
        for y in 1..img_rgba.height()-1 {
            for x in 1..img_rgba.width()-1 {
                let mut r = 0i32;
                let mut g = 0i32;
                let mut b = 0i32;
                
                for ky in 0..3 {
                    for kx in 0..3 {
                        let px = img_rgba.get_pixel(x + kx - 1, y + ky - 1);
                        let weight = if kx == 1 && ky == 1 { 9 } else { -1 };
                        r += px[0] as i32 * weight;
                        g += px[1] as i32 * weight;
                        b += px[2] as i32 * weight;
                    }
                }
                
                let original = img_rgba.get_pixel(x, y);
                let r_diff = (r / 9 - original[0] as i32).abs();
                let g_diff = (g / 9 - original[1] as i32).abs();
                let b_diff = (b / 9 - original[2] as i32).abs();
                
                if r_diff > threshold || g_diff > threshold || b_diff > threshold {
                    output.put_pixel(x, y, Rgba([
                        r.clamp(0, 255) as u8 / 9,
                        g.clamp(0, 255) as u8 / 9,
                        b.clamp(0, 255) as u8 / 9,
                        original[3]
                    ]));
                } else {
                    output.put_pixel(x, y, *original);
                }
            }
        }
        
        self.img = DynamicImage::ImageRgba8(output);
    }

    #[wasm_bindgen]
    pub fn detect_edges(&mut self) {
        let gray_img = self.img.to_luma8();
        let (width, height) = gray_img.dimensions();
        let mut output = ImageBuffer::new(width, height);

        let rows: Vec<Vec<u8>> = (0..height).into_par_iter().map(|y| {
            let mut row = vec![0u8; width as usize];
            
            for x in 0..width {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    row[x as usize] = 0;
                    continue;
                }

                let mut gx = 0i16;
                let mut gy = 0i16;

                for ky in 0..3 {
                    for kx in 0..3 {
                        let px = gray_img.get_pixel(x + kx - 1, y + ky - 1)[0] as i16;
                        gx += px * SOBEL_X[ky as usize][kx as usize];
                        gy += px * SOBEL_Y[ky as usize][kx as usize];
                    }
                }

                row[x as usize] = (gx.abs() + gy.abs()).min(255) as u8;
            }
            
            row
        }).collect();

        for (y, row) in rows.into_iter().enumerate() {
            for x in 0..width {
                output.put_pixel(x, y as u32, Luma([row[x as usize]]));
            }
        }

        self.img = DynamicImage::ImageLuma8(output);
    }

    #[wasm_bindgen]
    pub fn generate_thumbnail(&self, max_width: u32, max_height: u32) -> Vec<u8> {
        let (width, height) = self.img.dimensions();
        
        let ratio = f64::min(
            max_width as f64 / width as f64,
            max_height as f64 / height as f64
        );
        
        let new_width = (width as f64 * ratio).round() as u32;
        let new_height = (height as f64 * ratio).round() as u32;
        
        let thumbnail = self.img.resize(
            new_width,
            new_height,
            FilterType::Lanczos3
        );
        
        let mut buf = Cursor::new(Vec::new());
        thumbnail.write_to(&mut buf, image::ImageOutputFormat::Jpeg(85)).unwrap();
        buf.into_inner()
    }

    #[wasm_bindgen]
    pub fn to_png(&self) -> Result<Vec<u8>, JsValue> {
        let mut buf = Cursor::new(Vec::new());
        self.img.write_to(&mut buf, image::ImageOutputFormat::Png)
            .map_err(|e| JsValue::from_str(&format!("PNG encoding failed: {}", e)))?;
        Ok(buf.into_inner())
    }

    #[wasm_bindgen]
    pub fn to_jpeg(&self, quality: u8) -> Result<Vec<u8>, JsValue> {
        let mut buf = Cursor::new(Vec::new());
        self.img.write_to(&mut buf, image::ImageOutputFormat::Jpeg(quality))
            .map_err(|e| JsValue::from_str(&format!("JPEG encoding failed: {}", e)))?;
        Ok(buf.into_inner())
    }
}

static SOBEL_X: [[i16; 3]; 3] = [
    [-1, 0, 1],
    [-2, 0, 2],
    [-1, 0, 1]
];

static SOBEL_Y: [[i16; 3]; 3] = [
    [-1, -2, -1],
    [0, 0, 0],
    [1, 2, 1]
];