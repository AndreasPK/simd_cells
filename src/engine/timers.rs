use std::{collections::HashMap, ops::Not};

use crate::engine::types::EngineState;

/// Timers
///
/// Timers allow us to register events that happen once, or repeatedly
/// They can be keyed on ticks or on actual time (using Instant)
///
/// But I only bothered to implement tick based ones for now ...

fn timer_fun(engine_state: &mut super::types::EngineState<'_>) {}

type TimerCallBack = dyn FnMut(&mut EngineState);
type TimerHandle = u64;

struct TickTimer
{
    last_fire: u64,
    freq: u32,
    oneshot: bool,
    fun: Box<TimerCallBack>,
}

pub struct TimerManager {
    tick_timers: HashMap<TimerHandle, TickTimer>,
    next_handle: TimerHandle
}

impl TimerManager {
    pub fn new() -> Self{
        TimerManager { tick_timers: (HashMap::new()), next_handle: 0 }
    }
    fn register_tick_timer(
        &mut self,
        freq: u32,
        callback: Box<TimerCallBack>,
        oneshot:bool
    ) -> TimerHandle
    where
    {
        let timer = TickTimer {
            last_fire: 0,
            freq,
            fun: callback,
            oneshot: oneshot
        };

        self.tick_timers.insert(self.next_handle,timer);
        let h = self.next_handle;
        self.next_handle += 1;
        self.next_handle
    }

    fn unregister(&mut self, timer: TimerHandle) {
        self.tick_timers.remove(&timer);
    }

    fn run_timers(&mut self, state: &mut EngineState) {
        self.tick_timers.retain(|k, timer: &mut TickTimer| {
            if state.tick >= timer.last_fire + timer.freq as u64 {
                timer.last_fire = state.tick;
                (timer.fun)(state)
            };
            timer.oneshot.not()
            }
        );
    }

}
