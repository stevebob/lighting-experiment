use std::time::Duration;
use cgmath::Vector2;

use append::Append;
use entity_store::{EntityId, EntityChange, insert};
use content::SpriteAnimation;

pub enum Animation {
    Slide {
        id: EntityId,
        base: Vector2<f32>,
        path: Vector2<f32>,
        progress: f32,
        duration: Duration,
    },
    Sprites {
        id: EntityId,
        animation: SpriteAnimation,
        then: EntityChange,
        index: usize,
        remaining: Duration,
    },
}

pub enum AnimationStatus {
    Running(Animation),
    Finished,
}

pub enum AnimatedChange {
    Checked(EntityChange),
    Unchecked(EntityChange),
}

fn duration_ratio(a: Duration, b: Duration) -> f32 {
    let a_nanos = (a.subsec_nanos() as u64 + a.as_secs() * 1_000_000_000) as f32;
    let b_nanos = (b.subsec_nanos() as u64 + b.as_secs() * 1_000_000_000) as f32;
    a_nanos / b_nanos
}

impl Animation {
    pub fn populate<A: Append<AnimatedChange>>(self, time_delta: Duration, changes: &mut A) -> AnimationStatus {
        use self::Animation::*;
        use self::AnimatedChange::*;
        match self {
            Slide { id, base, path, mut progress, duration } => {
                let progress_delta = duration_ratio(time_delta, duration);
                progress += progress_delta;
                if progress > 1.0 {
                    progress = 1.0;
                }

                let new_position = base + path * progress;
                changes.append(Unchecked(insert::position(id, new_position)));

                if progress < 1.0 {
                    AnimationStatus::Running(Animation::Slide { id, base, path, progress, duration })
                } else {
                    AnimationStatus::Finished
                }
            }
            Sprites { id, animation, then, mut index, mut remaining } => {
                if time_delta < remaining {
                    remaining -= time_delta;
                    return AnimationStatus::Running(Animation::Sprites { id, animation, then, index, remaining });
                }

                let mut time_delta_rest = time_delta - remaining;
                index += 1;

                loop {
                    if index == animation.len() {
                        changes.append(Checked(then));
                        return AnimationStatus::Finished;
                    }

                    let frame = &animation[index];
                    let frame_duration = Duration::from_millis(frame.millis as u64);

                    if time_delta_rest < frame_duration {
                        remaining = frame_duration - time_delta_rest;
                        changes.append(Unchecked(insert::sprite(id, frame.sprite)));
                        break;
                    }

                    time_delta_rest -= frame_duration;
                    index += 1;
                }

                AnimationStatus::Running(Animation::Sprites { id, animation, then, index, remaining })
            }
        }
    }
}
