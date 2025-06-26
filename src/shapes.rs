use macroquad::prelude::*;
#[derive(Debug)]
pub struct LineSegment{
    pub start: Vec2,
    pub end: Vec2,
    pub absorbtion: f32
}

pub struct Border{
    pub borders: [LineSegment; 4]
}
impl Border{
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32, absorbtion: f32) -> Self {
        Border {
            borders: [
                LineSegment { start: vec2(x1, y1), end: vec2(x2, y1), absorbtion },
                LineSegment { start: vec2(x2, y1), end: vec2(x2, y2), absorbtion },
                LineSegment { start: vec2(x2, y2), end: vec2(x1, y2), absorbtion },
                LineSegment { start: vec2(x1, y2), end: vec2(x1, y1), absorbtion }
            ]
        }
    }
}


impl Ray{
    pub fn new(x:f32, y:f32, xdir:f32, ydir:f32, depth:usize, path_len:f32, energy:f32) -> Ray{
        let origin: Vec2 = vec2(x,y);
        let dir = vec2(xdir,ydir);
        let end = origin + dir.normalize() * 10.0;
        Ray { origin: origin, dir: dir,end: end, depth: (depth), path_len: (path_len), energy: (energy) }
    }
    pub fn collides_with(self,){
        
    }
}
pub struct Ray {
    pub origin:   Vec2,   // where the ray currently starts
    pub dir:      Vec2,   // normalized direction of travel
    pub end:      Vec2,    // End of arrow, easy to draw
    pub depth:    usize,  // how many times it has reflected so far
    pub path_len: f32,    // total distance (in world units) it’s traveled
    pub energy:   f32,    // remaining amplitude/energy (1.0 → full, 0.0 → none)
}
pub struct Hit {
    point:   Vec2,   // exact intersection point in world space
    normal:  Vec2,   // surface normal at that point (unit length)
    t:       f32,    // “how far along the ray” the hit occurred
    wall_ix: usize,  // index into your walls array (for material/absorption info)
}