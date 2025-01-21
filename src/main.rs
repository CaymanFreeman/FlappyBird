use bevy::app::App;
use flappy_bird;

fn main() {
    App::new().add_plugins(flappy_bird::AppPlugin).run();
}
