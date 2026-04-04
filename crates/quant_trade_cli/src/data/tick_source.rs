use crate::data::types::Tick;

/// Trait for tick data sources. Swappable between fake and real (Shioaji).
#[allow(dead_code)]
pub trait TickSource {
    fn next_tick(&mut self) -> Option<Tick>;
    fn reset(&mut self);
    fn total_ticks(&self) -> usize;
    fn position(&self) -> usize;
}

/// Stub for future 永豐金 Shioaji real-time API.
#[allow(dead_code)]
pub struct ShioajiTickSource;

impl TickSource for ShioajiTickSource {
    fn next_tick(&mut self) -> Option<Tick> {
        None
    }
    fn reset(&mut self) {}
    fn total_ticks(&self) -> usize {
        0
    }
    fn position(&self) -> usize {
        0
    }
}
