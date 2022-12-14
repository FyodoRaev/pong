pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;
pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;
use amethyst::{
  assets::{AssetStorage, Loader, Handle},
  core::transform::Transform,
  ecs::{Component, DenseVecStorage},
  prelude::*,
  renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
  // Load the sprite sheet necessary to render the graphics.
  // The texture is the pixel data
  // `texture_handle` is a cloneable reference to the texture
  let texture_handle = {
      let loader = world.read_resource::<Loader>();
      let texture_storage = world.read_resource::<AssetStorage<Texture>>();
      loader.load(
          "texture/pong_spritesheet.png",
          ImageFormat::default(),
          (),
          &texture_storage,
      )
  };
  let loader = world.read_resource::<Loader>();
  let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
  loader.load(
        "texture/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )

  //The Loader will take the file containing the sprites' positions and the texture handle, and create a nicely packaged SpriteSheet struct.
  //It is this struct that we will be using to actually draw stuff on the screen.
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle { //By implementing Component for the Paddle struct, it can now be attached to entities in the game.//
  type Storage = DenseVecStorage<Self>;
}

pub struct Ball {
  pub velocity_x: f32,
  pub velocity_y: f32,
  pub radius: f32,
}


impl Component for Ball {
  type Storage = DenseVecStorage<Self>;
}

fn initialise_camera(world: &mut World) {
  // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
  let mut transform = Transform::default();
  transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

  world
      .create_entity()
      .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
      .with(transform)
      .build();
}

pub struct Pong;

impl SimpleState for Pong {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) { 
    // This method is called when the State starts// 
    // The StateData<'_, GameData<'_, '_>> is a structure given to all State methods.//
    // The important part of its content here is its world field.//
    let world = data.world;
    let sprite_sheet_handle= load_sprite_sheet(world);
    world.register::<Ball>();
    initialise_paddles(world, sprite_sheet_handle.clone());
    initialise_ball(world, sprite_sheet_handle);
    initialise_camera(world);
  }
}

/// Initialises one paddle on the left, and one paddle on the right.
fn initialise_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
  /*SpriteRender is the Component that indicates which sprite of which sprite sheet should be drawn for a particular entity.
   Since the paddle is the first sprite in the sprite sheet, we use 0 for the sprite_number. */
  let sprite_render = SpriteRender::new(sprite_sheet_handle,0);  
  let mut left_transform = Transform::default();
  let mut right_transform = Transform::default();

  // Correctly position the paddles.
  let y = ARENA_HEIGHT / 2.0;
  left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
  right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

  // Create a left plank entity.
  world
      .create_entity()
      .with(sprite_render.clone())
      .with(Paddle::new(Side::Left))
      .with(left_transform)
      .build();

  // Create right plank entity.
  world
      .create_entity()
      .with(sprite_render)
      .with(Paddle::new(Side::Right))
      .with(right_transform)
      .build();
}

fn initialise_ball (world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
  // Create the translation.
  let mut local_transform = Transform::default();
  local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
  // Assign the sprite for the ball. The ball is the second sprite in the sheet.
  let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);
  world
    .create_entity()
    .with(sprite_render)
    .with(Ball {
      radius: BALL_RADIUS,
      velocity_x: BALL_VELOCITY_X,
      velocity_y: BALL_VELOCITY_Y,
  })
    .with(local_transform)
    .build();

}