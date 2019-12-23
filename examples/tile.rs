extern crate pushrod;
extern crate sdl2;

use pushrod::render::engine::Engine;
use pushrod::render::widget::Widget;
use pushrod::render::widget_config::{CONFIG_BORDER_WIDTH, CONFIG_COLOR_BORDER, CONFIG_COLOR_HOVER, CONFIG_COLOR_SELECTED};
use pushrod::render::{make_points, make_size};
use sdl2::pixels::Color;
use pushrod::widgets::tile_widget::TileWidget;

pub fn main() {
    let hover_color = Color::RGBA(0, 0, 0, 175);
    let selected_color = Color::RGBA(0, 0, 0, 75);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("pushrod-render tile demo", 370, 100)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut engine = Engine::new(370, 100, 60);
    let mut tile1 = TileWidget::new(
        make_points(10, 10),
        make_size(80, 80),
        String::from("assets/checkbox_selected.png"),
        make_size(32, 32),
        String::from("Check 1"),
    );

    tile1.set_color(CONFIG_COLOR_HOVER, hover_color.clone());
    tile1.set_color(CONFIG_COLOR_SELECTED, selected_color.clone());
    tile1.on_click(|_, _widgets, _layouts, state| {
        eprintln!("Tile 1 selected: {}", state);
    });

    let mut tile2 = TileWidget::new(
        make_points(100, 10),
        make_size(80, 80),
        String::from("assets/checkbox_unselected.png"),
        make_size(32, 32),
        String::from("Check 2"),
    );

    tile2.set_color(CONFIG_COLOR_HOVER, hover_color.clone());
    tile2.set_color(CONFIG_COLOR_SELECTED, selected_color.clone());

//    button2.on_toggle(|_, _widgets, _layouts, _state| {
//        eprintln!("2 Toggled: {}", _state);
//    });

    let mut tile3 = TileWidget::new(
        make_points(190, 10),
        make_size(80, 80),
        String::from("assets/rust-48x48.jpg"),
        make_size(32, 32),
        String::from("Rust Crab"),
    );

    tile3.set_color(CONFIG_COLOR_HOVER, hover_color.clone());
    tile3.set_color(CONFIG_COLOR_SELECTED, selected_color.clone());

    let mut tile4 = TileWidget::new(
        make_points(280, 10),
        make_size(80, 80),
        String::from("assets/radio_selected.png"),
        make_size(32, 32),
        String::from("Radio"),
    );

    tile4.set_color(CONFIG_COLOR_HOVER, hover_color.clone());
    tile4.set_color(CONFIG_COLOR_SELECTED, selected_color.clone());

    engine.add_widget(Box::new(tile1), String::from("tile1"));
    engine.add_widget(Box::new(tile2), String::from("tile2"));
    engine.add_widget(Box::new(tile3), String::from("tile3"));
    engine.add_widget(Box::new(tile4), String::from("tile4"));

    engine.run(sdl_context, window);
}
