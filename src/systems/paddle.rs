use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

#[derive(SystemDesc)]
pub struct PaddleSystem;

/*Next, we implement the System trait for it with the lifetime of the components on which it operates.
 Inside the implementation, we define the data the system operates on in the SystemData tuple:
 WriteStorage, ReadStorage, and Read.
 More specifically, the generic types we've used here tell us that the
 PaddleSystem mutates Transform components, WriteStorage<'s, Transform>,
 it reads Paddle components, ReadStorage<'s, Paddle>, 
 and also accesses the InputHandler<StringBindings> resource we created earlier,
 using the Read structure. */
impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement { //If there is a movement do the following steps
            let scaled_amount = 1.2 * mv_amount as f32;
            let paddle_y = transform.translation().y;
            transform.set_translation_y(
                (paddle_y + scaled_amount)
                    .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5) //if coordinate more than this then transform won't change
                    .max(PADDLE_HEIGHT * 0.5),                  //if coordinate less than this then transform won't change
            );
                }
            }
        }
      }