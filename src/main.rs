//! Pong Tutorial 1 
mod systems;
mod pong;
use crate::pong::Pong;

extern crate amethyst;
use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir, core::TransformBundle,
    input::{InputBundle, StringBindings},
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
    .with_bindings_from_file(binding_path)?;
    let game_data = GameDataBuilder::default() /*central repository of all the game logic that runs periodically during the game runtime */
    .with_bundle( /*Bundles are essentially sets of systems preconfigured to work together, so you don't have to write them all down one by one. */
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default()), //Those plugins will equip our renderer with the ability to open a window and draw sprites to it.//
    )?
    .with_bundle(TransformBundle::new())?
    .with_bundle(input_bundle)?
    .with(systems::PaddleSystem, "paddle_system", &["input_system"])//For InputBundle<StringBindings>, the parameter type determines how axes and actions are identified in the bindings.ron file (e.g. "left_paddle")
    .with(systems::MoveBallsSystem, "ball_system", &[])
    .with(systems::BounceSystem, "bounce_system", &["paddle_system", "ball_system"])
    .with(systems::WinnerSystem, "winner_system", &["ball_system"]); 
    let mut game = Application::new(assets_dir, Pong, game_data)?; /* starts the game loop. */
    game.run();
    Ok(())
}





