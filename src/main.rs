mod shapes;
use shapes::{Border, Ray};
use macroquad::prelude::*;
use std::thread::sleep;
#[macroquad::main("Ray-Traced Audio Viz")]
async fn main() {
    // 0) Find out how big our window actually is:
    let w = screen_width();
    let h = screen_height();

    // 1) Build a room that exactly matches our window (so edges aren’t off-screen)
    let room = Border::new(0.0, 0.0, w, h, 0.05);

    // 2) Choose an emitter location in the center
    let origin = vec2(w * 0.5, h * 0.5);

    // 3) Spawn 360 rays around that point
    let mut rays: Vec<Ray> = (0..360)
        .map(|i| {
            let theta = (i as f32).to_radians();
            let dir   = vec2(theta.cos(), theta.sin()).normalize();
            Ray::new(origin.x, origin.y, dir.x, dir.y, 0, 0.0, 1.0)
        })
        .collect();

    // Log once so we know they exist:
    println!("Spawned {} rays", rays.len());

    // 4) Main loop
    loop {
        
        clear_background(WHITE);

        // 4a) Draw the emitter so we can see it’s there
        draw_circle(origin.x, origin.y, 5.0, BLUE);
        
        // 4b) Draw the room borders
        for wall in room.borders.iter() {
            draw_line(
                wall.start.x, wall.start.y,
                wall.pos.x,   wall.pos.y,
                2.0, BLACK,
            );
        }
        sleep(std::time::Duration::from_secs_f32(0.5));
        // 4c) For each ray, cast one bounce and draw that segment
        for ray in rays.iter_mut() {
            // save the “from” point before we mutate ray.source
            let start = ray.source;
            println!("Start: {}",start);
            if let Some(hit) = ray.cast(&room.borders) {
                // we hit a wall—draw from start → hit
                println!("Hit");
                draw_line(
                    start.x, start.y,
                    hit.point.x, hit.point.y,
                    ray.energy * 2.0, // thicker when more energy
                    RED,
                );
            } else {
                println!("No Hit");
                // no intersection—draw all the way to the edge of the screen
                let end = start + ray.dir * (w.max(h));
                draw_line(
                    start.x, start.y,
                    end.x,   end.y,
                    1.0,
                    RED,
                );
            }
        }
        rays.retain(|p| p.energy > 0.5);
        println!("{}",rays.len());
        next_frame().await;
    }
}
