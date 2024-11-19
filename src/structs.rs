pub struct BeatBarTick {
    bar: u32,
    beat: u32,
    tick: u32,
}

impl BeatBarTick {
    pub fn new(bar: u32, beat: u32, tick: u32) -> Self {
        let mut bar = bar;
        let mut beat = beat % 4;
        bar += beat / 4;
        let tick = tick % 320;
        beat += tick / 320;
        Self { bar, beat, tick }
    }

    pub fn to_seconds(&self, tempo: f32) -> f64 {
        let bar_duration = self.bar as f64 * (240.0 / tempo as f64);
        let beat_duration = self.beat as f64 * (60.0 / tempo as f64);
        let tick_duration = self.tick as f64 * (0.1875 / tempo as f64);
        return bar_duration + beat_duration + tick_duration;
    }
}
