use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, System, WriteStorage},
};

use crate::pong::{Ball, ARENA_WIDTH};

pub struct WinningSystem;

impl<'s> System<'s> for WinningSystem {
    type SystemData = (WriteStorage<'s, Ball>, WriteStorage<'s, Transform>);

    fn run(&mut self, (mut balls, mut transforms): Self::SystemData) {
        (&mut balls, &mut transforms)
            .join()
            .into_iter()
            .for_each(|(ball, transform)| {
                let ball_x = transform.translation().x;
                let did_hit = if ball_x <= ball.radius {
                    println!("Player 2 scores!");
                    true
                } else if ball_x >= ARENA_WIDTH - ball.radius {
                    println!("Player 1 scores!");
                    true
                } else {
                    false
                };
                if did_hit {
                    ball.velocity[0] = -ball.velocity[0];
                    transform.set_translation_x(ARENA_WIDTH / 2.0);
                }
            })
    }
}
