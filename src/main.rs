use macroquad::prelude::*;

mod draw;

fn window() -> Conf {
    Conf {
        window_title: "Smuth".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        high_dpi: true,
        fullscreen: false,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window)]
async fn main() {
    loop {
        clear_background(draw::BG_COLOR);

        #[cfg(debug_assertions)]
        draw_text(&format!("FPS: {}", get_fps()), 0.0, 20.0, 20.0, GREEN);
        next_frame().await;
    }
}
