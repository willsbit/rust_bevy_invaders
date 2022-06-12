use bevy::prelude::Component;
use rand::{Rng, thread_rng};
use crate::{BASE_SPEED, FORMATION_MEMBERS_MAX, WinSize};

/// Component - Enemy formation for each enemy
#[derive(Clone, Component)]
pub struct Formation {
    pub start: (f32, f32),
    pub radius: (f32, f32),
    pub pivot: (f32, f32),
    pub speed: f32,
    pub angle: f32, // change per tick
}

/// Resource - Formation maker
#[derive(Default)]
pub struct FormationMaker {
    current_template: Option<Formation>,
    current_members: u32
}

/// Formation factory impl

impl FormationMaker {
    pub fn make(&mut self, win_size: &WinSize) -> Formation {
        match (&self.current_template, self.current_members >= FORMATION_MEMBERS_MAX) {
            // if has curren template and stil within max members
            (Some(tmpl), false) => {
                self.current_members += 1;
                tmpl.clone()
            },
            // if first formation or previous formation is full
            (None, _) | (_, true) => {
                let mut rng = thread_rng();

                // compute the start x,y
                let w_span = win_size.w / 2.0 + 100.0;
                let h_span = win_size.h / 2.0 + 100.0;

                let x = if rng.gen_bool(0.5) { w_span } else { -w_span };
                let y = rng.gen_range(-h_span..h_span) as f32;
                let start = (x, y);

                // compute the pivot point
                let w_span = win_size.w / 4.0;
                let h_span = win_size.h / 3.0 + 50.0;

                let pivot = (rng.gen_range(-w_span..w_span), rng.gen_range(0.0..h_span));

                // compute the radius
                let radius = (rng.gen_range(80.0..150.0), 100.0);

                // compute the start angle
                let angle = (y - pivot.1).atan2(x - pivot.0);

                // compute the speed (const for now)
                let speed = BASE_SPEED;

                // create the formation
                let formation = Formation {
                    start,
                    radius,
                    pivot,
                    speed,
                    angle,
                };

                // store as template
                self.current_template = Some(formation.clone());
                // reset members to 1
                self.current_members = 1;

                formation
                }
            }
        }
    }