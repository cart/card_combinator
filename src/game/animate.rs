use std::{ops::Range, time::Duration};

use bevy::prelude::Timer;

pub struct AnimateRange {
    timer: Timer,
    ease: Ease,
    range: Range<f32>,
}

impl AnimateRange {
    pub fn new(duration: Duration, ease: Ease, range: Range<f32>, repeat: bool) -> Self {
        Self {
            timer: Timer::new(duration, repeat),
            ease,
            range,
        }
    }

    pub fn set_percent(&mut self, percent: f32) {
        self.timer.set_elapsed(Duration::from_secs_f32(
            self.timer.duration().as_secs_f32() * percent,
        ));
    }

    pub fn percent(&mut self) -> f32 {
        self.timer.percent()
    }

    pub fn reset(&mut self) {
        self.timer.reset();
    }

    pub fn just_finished(&mut self) -> bool {
        self.timer.just_finished()
    }

    pub fn finished(&mut self) -> bool {
        self.timer.finished()
    }

    pub fn tick(&mut self, delta: Duration) -> f32 {
        self.timer.tick(delta);
        let amount = self.ease.ease(self.timer.percent());
        self.range.start + ((self.range.end - self.range.start) * amount)
    }
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Ease {
    Linear,
    // Sin,
    InOutCirc,
    OutBack,
    // Custom(fn(f32) -> f32),
}

impl Ease {
    pub fn ease(&self, x: f32) -> f32 {
        match self {
            Ease::Linear => x,
            // Ease::Sin => x.sin(),
            Ease::InOutCirc => {
                if x < 0.5 {
                    (1. - (1. - (2. * x).powf(2.)).sqrt()) / 2.
                } else {
                    ((1. - (-2. * x + 2.).powf(2.)).sqrt() + 1.) / 2.
                }
            }
            Ease::OutBack => {
                const C1: f32 = 1.70158;
                const C3: f32 = C1 + 1.0;

                1. + C3 * (x - 1.).powf(3.) + C1 * (x - 1.).powf(2.)
            }
        }
    }
}
