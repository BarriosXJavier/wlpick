use anyhow::{Context, Result};
use image::ImageReader;
use std::io::Cursor;
use std::process::Command;

pub struct Screenshot {
    pub width: u32,
    pub height: u32,
    pixels: Vec<u8>, // RGBA, row-major
}

impl Screenshot {
    pub fn capture() -> Result<Self> {
        let output = Command::new("grim")
            .arg("-")
            .output()
            .context("Failed to run grim — is it installed?")?;

        if !output.status.success() {
            anyhow::bail!("grim failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        let img = ImageReader::new(Cursor::new(output.stdout))
            .with_guessed_format()
            .context("Failed to read grim output")?
            .decode()
            .context("Failed to decode screenshot")?
            .into_rgba8();

        let (width, height) = img.dimensions();
        Ok(Self {
            width,
            height,
            pixels: img.into_raw(),
        })
    }
    // Bounds-checked pixel access — returns RGBA
    pub fn get_pixel(&self, x: u32, y: u32) -> [u8; 4] {
        assert!(x < self.width && y < self.height, "pixel out of bounds");
        let i = ((y * self.width + x) * 4) as usize;
        [
            self.pixels[i],
            self.pixels[i + 1],
            self.pixels[i + 2],
            self.pixels[i + 3],
        ]
    }

    pub fn pixels_raw(&self) -> &[u8] {
        &self.pixels
    }
}
