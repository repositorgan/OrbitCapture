// core/gesture/sampler.rs

//If returns True, executes the command

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

pub struct GestureSampler {
    points: Vec<CursorPoint>,
    last_timestamp: Option<u64>,
    config: GestureConfig,
}

impl GestureSampler {
    pub fn new(config: GestureConfig) -> Self {
        Self {
            points: Vec::with_capacity(256),
            last_timestamp: None,
            config,
        }
    }

    pub fn reset(&mut self) {
        self.points.clear();
        self.last_timestamp = None;
    }

    pub fn push(&mut self, point: CursorPoint) {
        if let Some(last_ts) = self.last_timestamp {
            if point.timestamp_ms - last_ts > self.config.max_pause_ms {
                self.reset();
            }
        }

        self.last_timestamp = Some(point.timestamp_ms);
        self.points.push(point);
    }
}

impl GestureSampler {
    pub fn is_closed_loop(&self) -> bool {
        if self.points.len() < self.config.min_points {
            return false;
        }

        let start = self.points.first().unwrap();
        let end = self.points.last().unwrap();

        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > self.config.closure_radius_px {
            return false;
        }

        self.total_perimeter() >= self.config.min_perimeter_px
    }

    fn total_perimeter(&self) -> f32 {
        self.points
            .windows(2)
            .map(|w| {
                let dx = w[1].x - w[0].x;
                let dy = w[1].y - w[0].y;
                (dx * dx + dy * dy).sqrt()
            })
            .sum()
    }
}
