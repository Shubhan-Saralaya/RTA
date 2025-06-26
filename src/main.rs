
mod shapes;
use shapes::{Border, Ray}; // Changed from Borders to Border
use macroquad::prelude::*;
extern crate nalgebra as na;    

#[macroquad::main("BasicShapes")]

async fn main() {
    let room: Border = Border::new(10.0, 10.0, 810.0, 610.0, 5.0); // Use shapes::Border
    // ! DIR IS NOT NORMALIZED. ANY FUNCTIONS THAT NEED IT MUST BE NORMALIZED FIRST
    let mut rays: Vec<Ray> = vec![Ray::new(50.0, 50.0, 0.1, 0.1, 0, 0.0, 10.0),Ray::new(100.0, 100.0, 0.1, -0.1, 0, 0.0, 12.0)];
    loop {
        clear_background(WHITE);
        
        for i in room.borders.iter(){
            draw_line(i.start.x, i.start.y, i.end.x, i.end.y, i.absorbtion, BLACK);
        }
        for ray  in rays.iter_mut(){
            draw_line(ray.origin.x, ray.origin.y, ray.end.x, ray.end.y, 3.0, RED);
            move_fixed(ray, 10.0)
        }

        next_frame().await;
    }
}

fn move_fixed(ray: &mut Ray, speed: f32){
    ray.end += ray.dir * speed;
}

//: MIGHT HAVE TO ADD CUR_POS vector to the struct for the graphic representation. inorder to keep origin stable