use druid::{Data, Lens};

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
