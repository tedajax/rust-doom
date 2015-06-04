use sdl2::timer::{get_performance_counter, get_performance_frequency};

pub struct GameTime {
	last_tick: u64,
	since_start_ns: u64,
	last_frame_ns: u64,
	delta_ns: u64,
	dt: f32,
	timescale: f32,
}

impl GameTime {
	pub fn new() -> GameTime {
		GameTime {
			last_tick: 0,
			since_start_ns: 0,
			last_frame_ns: 0,
			delta_ns: 0,
			dt: 0_f32,
			timescale: 1_f32,
		}
	}
	
	pub fn update(&mut self) {
		{
			let ticks = get_performance_counter();
			let frequency = get_performance_frequency();
			
			if self.last_tick == 0 {
				self.last_tick = ticks;
			}
			
			let mut diff = ticks - self.last_tick;
			diff *= 1_000_000_000_u64;
			diff /= frequency;
			
			self.delta_ns = diff;
			self.last_tick = ticks;
		}
		
		self.last_frame_ns = self.since_start_ns;
		self.since_start_ns += self.delta_ns;
		
		self.dt = (self.since_start_ns as f32) - (self.last_frame_ns as f32);
		self.dt /= 1_000_000_000_f32;
	}
	
	#[allow(dead_code)]
	pub fn dt(&self) -> f32 {
		self.dt * self.timescale
	}
	
	#[allow(dead_code)]
	pub fn dt_no_scale(&self) -> f32 {
		self.dt
	}
}