use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum Format {
    Hex,
    Rgb,
    Hsl,
}

pub fn format_color(color: [u8; 4], format: &Format) -> String {
    let [r, g, b, _] = color;
    match format {
        Format::Hex => format!("#{:02X}{:02X}{:02X}", r, g, b),
        Format::Rgb => format!("rgb({}, {}, {})", r, g, b),
        Format::Hsl => {
            let (h, s, l) = rgb_to_hsl(r, g, b);
            format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, l * 100.0)
        }
    }
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let l = (max + min) / 2.0;

    if delta == 0.0 {
        return (0.0, 0.0, l);
    }

    let s = delta / (1.0 - (2.0 * l - 1.0).abs());
    let h = if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    ((h + 360.0) % 360.0, s, l)
}
