use macroquad::prelude::*;
use std::f32::EPSILON;
#[derive(Debug)]
pub struct LineSegment{
    pub start: Vec2,
    pub pos: Vec2,
    pub absorption: f32
}
impl LineSegment {
    /// 2D “cross” (determinant)
    fn cross(a: Vec2, b: Vec2) -> f32 {
        a.x * b.y - a.y * b.x
    }

    /// Ray–segment intersection: origin + t·dir ?= start + u·(pos−start)
    /// Returns Some(Hit) if t≥0 and u∈[0,1].
    pub fn intersect(
        &self,
        origin: Vec2,
        dir: Vec2,
        wall_ix: usize,
    ) -> Option<Hit> {
        let seg_dir = self.pos - self.start;
        let denom   = Self::cross(dir, seg_dir);
        if denom.abs() < 1e-6 {
            return None; // parallel or nearly parallel
        }

        let diff = self.start - origin;
        let t    = Self::cross(diff, seg_dir) / denom;
        let u    = Self::cross(diff, dir)     / denom;

        if t >= 0.0 && (0.0..=1.0).contains(&u) {
            let point = origin + dir * t;
            // perp normal
            let mut normal = Vec2::new(-seg_dir.y, seg_dir.x).normalize();
            if normal.dot(dir) > 0.0 {
                normal = -normal;
            }
            Some(Hit { point, normal, t, wall_ix })
        } else {
            None
        }
    }
}

pub struct Border{
    pub borders: [LineSegment; 4]
}
impl Border{
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32, absorption: f32) -> Self {
        Border {
            borders: [
                LineSegment { start: vec2(x1, y1), pos: vec2(x2, y1), absorption },
                LineSegment { start: vec2(x2, y1), pos: vec2(x2, y2), absorption },
                LineSegment { start: vec2(x2, y2), pos: vec2(x1, y2), absorption },
                LineSegment { start: vec2(x1, y2), pos: vec2(x1, y1), absorption }
            ]
        }
    }
}


impl Ray{
    pub fn new(x:f32, y:f32, xdir:f32, ydir:f32, depth:usize, path_len:f32, energy:f32) -> Ray{
        let source: Vec2 = vec2(x,y);
        let dir = vec2(xdir,ydir);
        let pos = source + dir.normalize() * 10.0;
        Ray { source: source, dir: dir,pos: pos, depth: (depth), path_len: (path_len), energy: (energy) }
    }
    pub fn cast(&mut self, walls: &[LineSegment]) -> Option<Hit> {
        // 1) find nearest intersection
        let mut best: Option<Hit> = None;
        for (ix, wall) in walls.iter().enumerate() {
            if let Some(hit) = wall.intersect(self.source, self.dir, ix) {
                best = match best {
                    Some(ref b) if hit.t < b.t => Some(hit),
                    None                       => Some(hit),
                    _                          => best,
                };
            }
        }

        // 2) if we hit something, update ray for the bounce
        if let Some(hit) = best.as_ref() {
            // accumulate distance traveled
            self.path_len += hit.t;
            // reflect direction: d' = d − 2(d·n)n
            let d_dot_n = self.dir.dot(hit.normal);
            self.dir = (self.dir - 2.0 * d_dot_n * hit.normal).normalize();
            // attenuate energy by wall absorption
            let a = walls[hit.wall_ix].absorption;
            self.energy -= a;
            // move source just off the surface to avoid self‐hit
            self.source = hit.point + hit.normal * EPSILON;
            // increment bounce count
            self.depth += 1;
        }
        best
    }
}
#[derive(Clone)]
pub struct Ray {
    pub source:   Vec2,   // where the ray starts
    pub dir:      Vec2,   // normalized direction of travel
    pub pos:      Vec2,    // Rays current position
    pub depth:    usize,  // how many times it has reflected so far
    pub path_len: f32,    // total distance (in world units) it’s traveled
    pub energy:   f32,    // remaining amplitude/energy (1.0 → full, 0.0 → none)
}
pub struct Hit {
    pub point:   Vec2,   // exact intersection point in world space
    normal:  Vec2,   // surface normal at that point (unit length)
    t:       f32,    // “how far along the ray” the hit occurred
    wall_ix: usize,  // index into your walls array (for material/absorption info)
}