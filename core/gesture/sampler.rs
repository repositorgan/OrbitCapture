// core/gesture/sampler.rs

#[derive(Debug, Clone, Copy)]
pub struct CursorPoint {
    pub x: f32,
    pub y: f32,
    pub timestamp_ms: u64,
}

#[derive(Debug)]
pub struct GestureConfig {
    pub min_points: usize,
    pub min_perimeter_px: f32,
    pub closure_radius_px: f32,
    pub max_pause_ms: u64,
}
