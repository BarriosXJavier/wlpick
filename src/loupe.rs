use crate::screenshot::Screenshot;
use raylib::prelude::*;

const LOUPE_RADIUS: i32 = 80; // pixels of the loupe circle on screen
const ZOOM: f32 = 8.0; // magnification factor

pub fn run(screen: Screenshot) -> Option<[u8; 4]> {
    let (mut rl, thread) = raylib::init()
        .size(screen.width as i32, screen.height as i32)
        .title("wlpick")
        .fullscreen()
        .build();

    // Load screenshot pixels into a raylib texture
    let img = Image::gen_image_color(screen.width as i32, screen.height as i32, Color::BLACK);
    let mut texture = rl
        .load_texture_from_image(&thread, &img)
        .expect("Failed to create texture");
    texture
        .update_texture(screen.pixels_raw())
        .expect("Failed to upload screenshot to GPU");

    rl.set_target_fps(60);
    rl.hide_cursor();

    let mut picked: Option<[u8; 4]> = None;

    while !rl.window_should_close() {
        let mouse = rl.get_mouse_position();
        let mx = mouse.x as i32;
        let my = mouse.y as i32;

        // Clamp to screen bounds
        let sx = mx.clamp(0, screen.width as i32 - 1) as u32;
        let sy = my.clamp(0, screen.height as i32 - 1) as u32;

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            picked = Some(screen.get_pixel(sx, sy));
            break;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE); // draw full screenshot

        draw_loupe(&mut d, &screen, mx, my);
    }

    picked
}

fn draw_loupe(d: &mut RaylibDrawHandle, screen: &Screenshot, mx: i32, my: i32) {
    // Draw each pixel of the loupe by sampling the screenshot
    for dy in -LOUPE_RADIUS..=LOUPE_RADIUS {
        for dx in -LOUPE_RADIUS..=LOUPE_RADIUS {
            // Only draw within the circle
            if dx * dx + dy * dy > LOUPE_RADIUS * LOUPE_RADIUS {
                continue;
            }

            // Map screen pixel back to source
            let src_x = (mx + (dx as f32 / ZOOM) as i32).clamp(0, screen.width as i32 - 1) as u32;
            let src_y = (my + (dy as f32 / ZOOM) as i32).clamp(0, screen.height as i32 - 1) as u32;

            let [r, g, b, a] = screen.get_pixel(src_x, src_y);
            d.draw_pixel(mx + dx, my + dy, Color { r, g, b, a });
        }
    }

    // Crosshair
    d.draw_line(mx - LOUPE_RADIUS, my, mx + LOUPE_RADIUS, my, Color::WHITE);
    d.draw_line(mx, my - LOUPE_RADIUS, mx, my + LOUPE_RADIUS, Color::WHITE);

    // Circle border
    d.draw_circle_lines(mx, my, LOUPE_RADIUS as f32, Color::WHITE);

    // Color preview
    let [r, g, b, a] = screen.get_pixel(
        (mx as u32).clamp(0, screen.width - 1),
        (my as u32).clamp(0, screen.height - 1),
    );
    let swatch = Color { r, g, b, a };
    let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
    let label_y = my + LOUPE_RADIUS + 5;
    let swatch_y = label_y + 14;
    d.draw_text(&hex, mx - 20, label_y, 10, Color::WHITE);
    d.draw_rectangle(mx - 20, swatch_y, 40, 20, swatch);
    d.draw_rectangle_lines(mx - 20, swatch_y, 40, 20, Color::WHITE);
}
