use terminal_graphics::{Colour, Display};
use termion::{event::Key, input::TermRead as _, raw::IntoRawMode as _};

trait Chaos {
    fn compute(&self, x: f32, y: f32) -> Colour;

    fn on_key(&mut self, ch: char);

    fn draw(&self, display: &mut Display, width: u16, height: u16) {
        let width_inv = 1.0 / (width - 1) as f32;
        let height_inv = 1.0 / (height - 1) as f32;
        for y in 0..height {
            for x in 0..width {
                let colour = self.compute((x as f32) * width_inv, (y as f32) * height_inv);
                display.set_pixel(x as _, y as _, '█', colour, Colour::Black);
            }
        }
    }
}

struct Mandelbrot {
    start_x: f32,
    len_x: f32,
    start_y: f32,
    len_y: f32,
    num_iter: usize,
    step: f32,
}

impl Mandelbrot {
    fn zoom(&mut self, amount: f32) {
        let old = (self.len_x, self.len_y);
        self.len_x *= amount;
        self.len_y *= amount;
        self.start_x -= 0.5 * (self.len_x - old.0);
        self.start_y -= 0.5 * (self.len_y - old.1);
    }
}

impl Chaos for Mandelbrot {
    fn compute(&self, raw_x: f32, raw_y: f32) -> Colour {
        let cx = self.start_x + raw_x * self.len_x;
        let cy = self.start_y + raw_y * self.len_y;
        let mut iter = 0;
        let (mut x, mut y) = (cx, cy);
        while x * x + y * y < 2.0 {
            iter += 1;
            if iter == self.num_iter {
                return Colour::Black;
            }
            let y2 = y * y;
            y = 2.0 * x * y + cy;
            x = x * x - y2 + cx;
        }
        let variants = Colour::variants();
        variants[1 + iter * (variants.len() - 1) / self.num_iter]
    }

    fn on_key(&mut self, ch: char) {
        match ch {
            'a' => self.start_x -= self.step * self.len_x,
            'd' => self.start_x += self.step * self.len_x,
            'w' => self.start_y -= self.step * self.len_y,
            's' => self.start_y += self.step * self.len_y,
            'q' => self.zoom(1.0 + self.step),
            'e' => self.zoom(1.0 - self.step),
            'z' => self.num_iter = (self.num_iter + 1) >> 1,
            'x' => self.num_iter = self.num_iter << 1,
            _ => (),
        }
    }
}

fn main() {
    let (width, height) = termion::terminal_size().unwrap();
    let mut chaos = {
        let x_center = -0.5;
        let standard_width = 3.0;
        let ratio = width as f32 / height as f32 / 3.0;
        let len_x = standard_width * ratio;
        Mandelbrot {
            start_x: x_center - 0.5 * len_x,
            len_x,
            start_y: -1.0,
            len_y: 2.0,
            num_iter: 50,
            step: 0.1,
        }
    };

    let _stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut display = Display::new(width as u32, height as u32);
    let mut keys = std::io::stdin().keys();
    loop {
        display.clear();
        chaos.draw(&mut display, width, height);
        display.print();

        match keys.next() {
            Some(Ok(Key::Esc)) | Some(Err(_)) => return,
            Some(Ok(Key::Char(ch))) => chaos.on_key(ch),
            _ => {}
        }
    }
}
