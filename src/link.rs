use nannou::prelude::*;

pub struct Link {
    pub origin: Vec2,
    pub p2: Vec2,
    pub radius: f32,

}

pub struct Circle {
    pub pos: Vec2,
    pub radius: f32,
}

impl Link {
    pub fn from(origin: Vec2, p2: Vec2, radius: f32) -> Self {
        Self { origin, p2,radius }
    }
    pub fn angle_radius(pos: Vec2, angle: f32, radius: f32) -> Vec2 {
        pos + vec2(f32::cos(angle) * radius, f32::sin(angle) * radius)
    }
    pub fn draw(&self, draw: &Draw) {
        draw.line().start(self.origin).end(self.p2).color(WHITE);
        draw.ellipse().radius(10.0).xy(self.origin).color(WHITE);
        draw.ellipse().radius(10.0).xy(self.p2).color(WHITE);
    }
    pub fn convert_to_circles(&self) -> Circle{
        Circle::from(self.origin, self.radius)
    }
    pub fn radius_intersection(&self, link2: &Link) -> (Option<Vec2>, Option<Vec2>) {
        let circle1 = self.convert_to_circles();
        let circle2 = link2.convert_to_circles();

        Circle::intersection_circle_to_circle(circle1, circle2)
    }
    pub fn link_to_point(point: Vec2, link1: &mut Vec2, link2: &mut Vec2){
        *link1 = point.clone();
        *link2 = point.clone();
    }
}

impl Circle {
    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            radius: 10.0,
        }
    }
    pub fn from(pos: Vec2, radius: f32) -> Self {
        Self { pos, radius }
    }
    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .radius(self.radius)
            .xy(self.pos)
            .color(Rgba8::new(0, 0, 0, 0))
            .stroke_color(WHITE)
            .stroke_weight(2.0);
    }
    pub fn intersection_circle_to_circle(c1: Circle, c2: Circle) -> (Option<Vec2>, Option<Vec2>) {
        // Define the circles with their centers and radii
        let center1 = c1.pos; // Center of circle 1
        let radius1 = c1.radius; // Radius of circle 1
        let center2 = c2.pos; // Center of circle 2
        let radius2 = c2.radius; // Radius of circle 2

        // Calculate the distance between the centers of the two circles
        let distance = center1.distance(center2);

        // Check if the circles intersect
        if distance <= radius1 + radius2 && distance > (radius1 - radius2).abs() {
            // Calculate the intersection points
            let d = distance;
            let a = (radius1.powi(2) - radius2.powi(2) + d.powi(2)) / (2.0 * d);
            let h = (radius1.powi(2) - a.powi(2)).sqrt();
            let p0 = center1 + a * (center2 - center1) / d;
            let p1 = pt2(
                p0.x + h * (center2.y - center1.y) / d,
                p0.y - h * (center2.x - center1.x) / d,
            );
            let p2 = pt2(
                p0.x - h * (center2.y - center1.y) / d,
                p0.y + h * (center2.x - center1.x) / d,
            );

            (Some(p1), Some(p2))
        } else {
            (None, None)
        }
    }
}
