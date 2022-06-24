use std::time::Instant;

use druid::{Data, Lens, Widget, Size, TimerToken, Event};

#[derive(Clone, Data)]
pub struct Transport {
    /// Current time, in milliseconds.
    current_time: u64,
    playing: bool,
}

impl Transport {
    pub fn new() -> Self {
        Self {
            current_time: 0,
            playing: false,
        }
    }

    pub fn play(&mut self) {
        self.playing = true;
    }

    pub fn pause(&mut self) {
        self.playing = false;
    }

    pub fn rewind(&mut self) {
        self.current_time = 0;
    }

    /// Pause and rewind.
    pub fn stop(&mut self) {
        self.pause();
        self.rewind();
    }

    /// Jump back 10 seconds.
    pub fn jump_back(&mut self) {
        self.current_time -= 10_000;
    }

    /// Jump forward 10 seconds.
    pub fn jump_fwd(&mut self) {
        self.current_time += 10_000;
    }

    pub fn update(&mut self) {
        if self.playing {
            self.current_time += 1;
        }
    }

    pub fn is_playing(&self) -> bool {
        self.playing
    }

    pub fn current_time(&self) -> u64 {
        self.current_time
    }
}


/// A widget that manages a timer to manage the transport. Phew! That's a lot of management.
struct TransportManager {
    timer_id: TimerToken,
    last_update: Instant
}

impl Widget<Transport> for TransportManager {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut Transport, env: &druid::Env) {
        match event {
            Event::WindowConnected => {

            },
            _ => ()
        }
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &Transport, env: &druid::Env) {
        
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &Transport, data: &Transport, env: &druid::Env) {
        
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &Transport, env: &druid::Env) -> druid::Size {
        bc.constrain(Size::new(0., 0.))
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Transport, env: &druid::Env) {
        
    }
}