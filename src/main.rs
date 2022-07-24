use raylib::color::Color;
use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = (SCREEN_WIDTH as f32 * 0.75) as i32;
static TEXTURE_FILENAME: &str = "assets/wall_horizontal_03.png";

fn main() {
    let (mut rl_handle, rl_thread) = raylib::init()
        .width(SCREEN_WIDTH)
        .height(SCREEN_HEIGHT)
        .title("RAYLIB TEXTURE DEMO")
        .build();

    let texture = rl_handle.load_texture(&rl_thread, TEXTURE_FILENAME);
    if texture.is_ok() {
        println!("Loaded texture: {}", TEXTURE_FILENAME);
    } else {
        eprintln!("Failed loading texture: {}", TEXTURE_FILENAME);
        return;
    }

    let texture_2d = texture.unwrap();
    while !rl_handle.window_should_close() {
        let mut draw_handle = rl_handle.begin_drawing(&rl_thread);
        draw_handle.clear_background(Color::BLACK);

        let screen_width : f32 = draw_handle.get_screen_width() as f32;
        let screen_height_halved : f32 = draw_handle.get_screen_height() as f32 / 2.0;
        let texture_height_halved : f32 = texture_2d.height() as f32;
        let vertical_offset = screen_height_halved - texture_height_halved;

        let target_rect = Rectangle::new(0.0, vertical_offset, screen_width, 16.0);
        let tiling = target_rect.width / texture_2d.width() as f32;

        draw_handle.draw_texture_quad(
            &texture_2d,
            Vector2::new(tiling, 1.0), // tiling
            Vector2::new(0.0, 0.0), // offset
            target_rect,
            Color::WHITE,
        )
    }
}
