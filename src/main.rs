use macroquad::prelude::*;

const G: f64 = 6.67430e-11;
const PLANET_MASS: f64 = 5.0e14;
const PLANET_RADIUS: f64 = 1_200.0;
const ENGINE_THRUST: f64 = 18_000.0;
const DRY_MASS: f64 = 900.0;
const FUEL_START: f64 = 1_800.0;
const ISP: f64 = 285.0;
const G0: f64 = 9.80665;

struct Rocket {
    pos: Vec2, vel: Vec2, angle: f32, fuel: f64, throttle: f64,
    time: f64, crashed: bool, landed: bool, max_altitude: f64,
}
impl Rocket {
    fn new() -> Self {
        Self { pos: vec2(0.0, (PLANET_RADIUS + 8.0) as f32), vel: Vec2::ZERO,
            angle: 0.0, fuel: FUEL_START, throttle: 0.0, time: 0.0,
            crashed: false, landed: false, max_altitude: 0.0 }
    }
    fn mass(&self) -> f64 { DRY_MASS + self.fuel }
    fn altitude(&self) -> f64 { self.pos.length() as f64 - PLANET_RADIUS }
    fn step(&mut self, dt: f64) {
        if self.crashed || self.landed { return; }
        let r = self.pos.length().max(1.0) as f64;
        let gravity = (G * PLANET_MASS / (r * r)) as f32;
        let down = -self.pos.normalize_or_zero();
        let density = (-(self.altitude().max(0.0) / 8_500.0)).exp() as f32;
        let speed = self.vel.length();
        let drag = -self.vel * (0.5 * 1.225 * density * 0.42 * 1.8 * speed / self.mass() as f32);
        let mut acceleration = down * gravity + drag;
        if self.throttle > 0.0 && self.fuel > 0.0 {
            let direction = vec2(self.angle.sin(), self.angle.cos());
            let thrust = ENGINE_THRUST * self.throttle;
            acceleration += direction * (thrust / self.mass()) as f32;
            self.fuel = (self.fuel - thrust / (ISP * G0) * dt).max(0.0);
        }
        self.vel += acceleration * dt as f32;
        self.pos += self.vel * dt as f32;
        self.time += dt;
        self.max_altitude = self.max_altitude.max(self.altitude());
        if self.altitude() <= 0.0 {
            let radial_speed = self.vel.dot(self.pos.normalize_or_zero());
            if radial_speed < -12.0 || self.vel.length() > 18.0 { self.crashed = true; }
            else {
                self.landed = true;
                self.vel = Vec2::ZERO;
                self.pos = self.pos.normalize_or_zero() * (PLANET_RADIUS as f32 + 3.0);
            }
        }
    }
}
fn window_conf() -> Conf {
    Conf { window_title: "SFS Rust — Flight Test".into(), window_width: 1280,
        window_height: 800, high_dpi: true, ..Default::default() }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut rocket = Rocket::new();
    let mut paused = false;
    let mut warp = 1.0_f64;
    let mut show_help = true;
    loop {
        let frame = get_frame_time().min(0.05) as f64;
        if is_key_pressed(KeyCode::R) { rocket = Rocket::new(); paused = false; }
        if is_key_pressed(KeyCode::H) { show_help = !show_help; }
        if is_key_pressed(KeyCode::P) { paused = !paused; }
        if is_key_pressed(KeyCode::Key1) { warp = 1.0; }
        if is_key_pressed(KeyCode::Key2) { warp = 2.0; }
        if is_key_pressed(KeyCode::Key3) { warp = 5.0; }
        if is_key_pressed(KeyCode::Key4) { warp = 10.0; }
        if !paused && !rocket.crashed && !rocket.landed {
            if is_key_down(KeyCode::A) { rocket.angle -= (1.8 * frame) as f32; }
            if is_key_down(KeyCode::D) { rocket.angle += (1.8 * frame) as f32; }
            rocket.throttle = if is_key_down(KeyCode::Space) && rocket.fuel > 0.0 { 1.0 } else { 0.0 };
            let total = frame * warp;
            let steps = (total / 0.005).ceil().max(1.0) as usize;
            let dt = total / steps as f64;
            for _ in 0..steps { rocket.step(dt); }
        } else { rocket.throttle = 0.0; }

        // Keep the rocket visible while gradually zooming out as altitude grows.
        let scale = (500.0 / rocket.pos.length().max(PLANET_RADIUS as f32)).clamp(0.08, 0.42);
        let camera = Camera2D {
            target: rocket.pos,
            zoom: vec2(2.0 * scale / screen_width(), -2.0 * scale / screen_height()),
            ..Default::default()
        };
        set_camera(&camera);
        clear_background(Color::from_rgba(5, 9, 20, 255));
        for i in 0..180 {
            let x = ((i * 7919 % 4000) as f32 - 2000.0) * 2.0;
            let y = ((i * 104729 % 4000) as f32 - 2000.0) * 2.0;
            draw_circle(x, y, 2.0 / scale, Color::from_rgba(180, 195, 220, 170));
        }
        draw_circle(0.0, 0.0, PLANET_RADIUS as f32, Color::from_rgba(34, 91, 135, 255));
        draw_circle(0.0, 0.0, PLANET_RADIUS as f32 - 18.0, Color::from_rgba(39, 119, 91, 255));
        draw_circle_lines(0.0, 0.0, PLANET_RADIUS as f32, 8.0 / scale, SKYBLUE);
        let dir = vec2(rocket.angle.sin(), rocket.angle.cos());
        let side = vec2(dir.y, -dir.x);
        let nose = rocket.pos + dir * 25.0;
        let left = rocket.pos - dir * 20.0 - side * 10.0;
        let right = rocket.pos - dir * 20.0 + side * 10.0;
        if rocket.throttle > 0.0 {
            let flame = rocket.pos - dir * (30.0 + rand::gen_range(0.0, 18.0));
            draw_triangle(rocket.pos - dir * 15.0, flame + side * 5.0, flame - side * 5.0, ORANGE);
        }
        draw_triangle(nose, left, right, LIGHTGRAY);
        set_default_camera();

        let alt = rocket.altitude();
        let speed = rocket.vel.length() as f64;
        let gravity = G * PLANET_MASS / (rocket.pos.length() as f64).powi(2);
        let orbit_speed = (G * PLANET_MASS / rocket.pos.length() as f64).sqrt();
        draw_rectangle(16.0, 16.0, 330.0, 174.0, Color::from_rgba(8, 14, 28, 220));
        draw_text("SFS RUST  |  FLIGHT TEST", 30.0, 42.0, 22.0, SKYBLUE);
        draw_text(&format!("ALTITUDE  {:>9.1} m", alt.max(0.0)), 30.0, 70.0, 20.0, WHITE);
        draw_text(&format!("VELOCITY  {:>9.1} m/s", speed), 30.0, 96.0, 20.0, WHITE);
        draw_text(&format!("FUEL      {:>8.1} kg", rocket.fuel), 30.0, 122.0, 20.0, WHITE);
        draw_text(&format!("GRAVITY   {:>8.2} m/s²", gravity), 30.0, 148.0, 20.0, WHITE);
        draw_text(&format!("ORBIT SPEED {:>7.1} m/s", orbit_speed), 30.0, 174.0, 20.0, WHITE);
        draw_rectangle(16.0, screen_height() - 56.0, 620.0, 40.0, Color::from_rgba(8, 14, 28, 220));
        draw_text("SPACE: THRUST   A/D: ROTATE   P: PAUSE   R: RESET   1-4: TIME WARP", 28.0, screen_height() - 30.0, 17.0, WHITE);
        if show_help {
            draw_rectangle(screen_width()-300.0, 16.0, 284.0, 100.0, Color::from_rgba(8, 14, 28, 220));
            draw_text("FLIGHT COMPUTER", screen_width()-284.0, 42.0, 19.0, SKYBLUE);
            draw_text(&format!("MISSION TIME  {:>7.1}s", rocket.time), screen_width()-284.0, 68.0, 17.0, WHITE);
            draw_text(&format!("MAX ALTITUDE  {:>7.1}m", rocket.max_altitude), screen_width()-284.0, 92.0, 17.0, WHITE);
        }
        if paused || rocket.crashed || rocket.landed {
            let msg = if paused { "PAUSED" } else if rocket.crashed { "VEHICLE LOST — PRESS R TO RETRY" } else { "SOFT LANDING — PRESS R TO FLY AGAIN" };
            let w = measure_text(msg, None, 28, 1.0).width;
            draw_text(msg, (screen_width()-w)/2.0, screen_height()*0.42, 28.0, if rocket.crashed { RED } else { GREEN });
        }
        next_frame().await;
    }
}
