use std::cmp;
use sdl2::render::Renderer;
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use sdl2::pixels::{ PixelFormatEnum, Color };

pub struct Video {
	surface: Surface<'static>,
	draw_color: Color,
	clear_color: Color,
	width: i32,
	height: i32,
}

pub struct Point {
	x: i32,
	y: i32,
}

impl Video {
	pub fn new(width: u32, height: u32) -> Video {
		Video {
			surface: Surface::new(width, height, PixelFormatEnum::RGBA8888).unwrap(),
			draw_color: Color::RGB(255, 255, 255),
			clear_color: Color::RGB(0, 0, 0),
			width: width as i32,
			height: height as i32,
		} 
	}
	
	pub fn set_draw_color(&mut self, color: Color) {
		self.draw_color = color;
	}
	
	pub fn set_clear_color(&mut self, color: Color) {
		self.clear_color = color;
	}
	
	pub fn clear(&mut self) {
		let _ = self.surface.fill_rect(None, self.clear_color);
	}
	
	pub fn test(&mut self) {
		self.set_draw_color(Color::RGB(255, 0, 0));
		self.hline(20, 0, 380);
		self.set_draw_color(Color::RGB(0, 255, 0));
		self.vline(20, 0, 380);
		
		self.set_draw_color(Color::RGB(0, 0, 255));
		for i in 0..10 {
			self.point(&Point { x: (i + 1) * 20, y: 50 });
		}

		self.set_draw_color(Color::RGB(0, 255, 255));
		let points = vec!(Point{x: 20, y: 60}, Point{x: 40, y: 60}, Point{x: 60, y: 60});
		self.points(&points[..]);
		
		self.fill_rect(&Rect { x: 200, y: 200, w: 20, h: 20 });
		self.set_draw_color(Color::RGB(255, 255, 0));
		let mut rects = Vec::new();
		for i in 0..5 {
			rects.push(Rect { x: i*21+200, y: 220, w: 20, h: 20 });
		}
		self.fill_rects(&rects[..]);
		
		self.set_draw_color(Color::RGB(255, 0, 255));
		self.line(50, 0, 300, 100);
	}
	
	pub fn point(&mut self, point: &Point) {
		if point.x < 0 || point.x >= self.width || point.y < 0 || point.y >= self.height {
			return;
		}

		// Todo: Don't use fill rect, poke memory directly
		let r = Rect {
			x: point.x,
			y: point.y,
			w: 1,
			h: 1,
		};
		self.surface.fill_rect(Some(r), self.draw_color).unwrap();
	}
	
	pub fn points(&mut self, points: &[Point]) {
		for p in points.iter() {
			self.point(p);
		}
	}
	
	fn vline(&mut self, x: i32, y1: i32, y2: i32) {
		let ymin = cmp::min(y1, y2);
		let ymax = cmp::max(y1, y2);
		let r = Rect {
			x: x,
			y: ymin,
			w: 1,
			h: (ymax - ymin), 
		};
		self.surface.fill_rect(Some(r), self.draw_color).unwrap();
	}
	
	fn hline(&mut self, y: i32, x1: i32, x2: i32) {
		let xmin = cmp::min(x1, x2);
		let xmax = cmp::max(x1, x2);
		let r = Rect {
			x: xmin,
			y: y,
			w: (xmax - xmin),
			h: 1, 
		};
		self.surface.fill_rect(Some(r), self.draw_color).unwrap();
	}
	
	pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
		let dx = (x2 - x1).abs();
		let dy = (y2 - y1).abs();
		
		if dx == 0 {
			return self.vline(x1, y1, y2);
		} else if dy == 0 {
			return self.hline(y1, x1, x2);
		}
		
		let deltax = dx as f32;
		let deltay = dy as f32;
		let mut error = 0_f32;
		let mut deltaerr = (deltay / deltax).abs();
		if (deltaerr - deltay).abs() <= 0.1_f32 {
			deltaerr = deltay - 1f32;
		} 
		
		let xmin = cmp::min(x1, x2);
		let xmax = cmp::max(x1, x2);
		
		let ymin;
		let ymax;
		if x1 < x2 {
			ymin = cmp::min(y1, y2);
			ymax = cmp::max(y1, y2);
		} else {
			ymin = cmp::max(y1, y2);
			ymax = cmp::min(y1, y2);
		}
		
		let mut y = ymin;
		for x in xmin..(xmax+1) {
			self.point(&Point{x: x, y: y});
			error += deltaerr;
			while error >= 0.5_f32 {
				self.point(&Point{x: x, y: y});
				y += (ymax - ymin).signum();
				error -= 1_f32;
			}
		}
	}
	
	pub fn fill_rect(&mut self, rect: &Rect) {
		// Todo: must be a better way?
		let r = Rect {
			x: rect.x,
			y: rect.y,
			w: rect.w,
			h: rect.h,
		};
		self.surface.fill_rect(Some(r), self.draw_color);
	}
	
	pub fn fill_rects(&mut self, rects: &[Rect]) {
		for r in rects.iter() {
			self.fill_rect(r);
		}
	}
	
	pub fn render(&self, renderer: &mut Renderer) {
		let texture = renderer.create_texture_from_surface(&self.surface).unwrap();
		
		renderer.drawer().copy(&texture, None, None);
	}
}