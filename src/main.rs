use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, startup).run();
}

fn startup(mut exit: EventWriter<AppExit>) {
    println!("Hello, Bevy + Cranelift!");
    exit.write(AppExit::Success);
}
