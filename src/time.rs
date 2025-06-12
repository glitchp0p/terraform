//Time Module originally made for terreact

// src/time.rs
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum TimeCurve {
    Linear,
    Log, 
    Jump,
}

#[derive(Debug)]
pub struct TimeSystem {
    pub current_tick: u64,
    tick_accum: f32,            // tracks fractional ticks
    pub tick_rate: f32,          // ticks per second (can be negative)
    pub base_rate: f32,          // normal speed (1.0)
    pub curve_type: TimeCurve,
    pub key_hold_duration: f32,  // how long key held
    pub is_paused: bool,
    pub reverse_unlocked: bool,  // gameplay gate
    
    last_update: Instant,
}

impl TimeSystem {
    pub fn new() -> Self {
        Self {
            current_tick: 0,
            tick_accum: 0.0,    //init fractional ticks 
            tick_rate: 1.0,
            base_rate: 1.0,
            curve_type: TimeCurve::Log,
            key_hold_duration: 0.0,
            is_paused: false,
            reverse_unlocked: false, // Start locked
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self, delta_time: f32, right_held: bool, left_held: bool) {
        // update key hold Duration
        if right_held || left_held {
            self.key_hold_duration += delta_time;
        } else {
            self.key_hold_duration = 0.0;
            self.tick_rate = self.base_rate; //return to base rate when released
        }

        // calculate time rate based on input and curve
        if right_held {
            self.tick_rate = self.calculate_rate(self.key_hold_duration, false);
        } else if left_held && self.reverse_unlocked {
            self.tick_rate = self.calculate_rate(self.key_hold_duration, true);
        }

        // update current tick based on tick rate
        let tick_delta = self.tick_rate * delta_time;

        // accumulate fractional ticks properly
        let tick_delta = self.tick_rate * delta_time;
        self.tick_accum += tick_delta;

        //when we have a full tick advance counter
        if self.tick_accum >= 1.0 {
            let full_ticks = self.tick_accum.floor() as u64;
            self.current_tick += full_ticks;
            self.tick_accum -= full_ticks as f32;
        }

        //old tick logic
       // self.current_tick = (self.current_tick as f32 + tick_delta).max(0.0) as u64;
    }

    fn calculate_rate(&self, hold_duration: f32, reverse: bool) -> f32 {
        let rate = match self.curve_type {
            TimeCurve::Log => (1.0 + hold_duration).ln(),
            TimeCurve::Jump => hold_duration.floor() + 1.0,
            TimeCurve::Linear => 1.0 + hold_duration,
        };

        if reverse { -rate } else {rate}
    }
}
