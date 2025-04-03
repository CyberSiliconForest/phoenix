use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoStaticStr};

/// State can transition in this way
/// Disarmed <-> Armed -> Alarm
#[derive(Serialize, Deserialize, Debug, IntoStaticStr, PartialEq, Eq, Clone)]
pub(crate) enum PhoenixPhase {
    #[serde(rename = "disarmed")]
    Disarmed,
    #[serde(rename = "armed")]
    Armed,
    #[serde(rename = "alarm")]
    Alarm,
}

pub(crate) struct PhoenixState {
    phase: PhoenixPhase,
    expires: Option<DateTime<Utc>>,
}

impl PhoenixState {
    pub fn new() -> Self {
        Self {
            phase: PhoenixPhase::Disarmed,
            expires: None,
        }
    }

    pub fn get_state(&self) -> (PhoenixPhase, Option<DateTime<Utc>>) {
        (self.phase.clone(), self.expires)
    }

    pub fn update_state(&mut self, phase: PhoenixPhase, expires: DateTime<Utc>) {
        match self.phase {
            PhoenixPhase::Disarmed | PhoenixPhase::Armed => {
                if phase != PhoenixPhase::Disarmed {
                    self.expires = Some(expires);
                } else {
                    self.expires = None;
                }
                self.phase = phase;
            }
            PhoenixPhase::Alarm => {
                // You cannot change the phase except editing persistency file directly.
            }
        }
    }

    pub fn do_tick(&mut self) {
        if self.phase == PhoenixPhase::Armed {
            let cur_time = chrono::Utc::now();
            match self.expires {
                None => {
                    // Huh? How it can happen?
                    self.phase = PhoenixPhase::Alarm;
                }
                Some(expires) => {
                    if cur_time > expires {
                        self.phase = PhoenixPhase::Alarm;
                        self.expires = None;
                    }
                }
            }
        }
    }
}
