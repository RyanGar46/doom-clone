extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{pixels, video};

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::Canvas;

const TARGET_FPS: u64 = 20;
const ENABLE_TARGET_FPS: bool = true;
const RESOLUTION_MULTIPLIER: i16 = 1;                   //0=160x120 1=360x240 4=640x480
const SCREEN_WIDTH: i16 = 160 * RESOLUTION_MULTIPLIER;  //screen width
const SCREEN_HEIGHT: i16 = 120 * RESOLUTION_MULTIPLIER; //screen height
const HALF_SCREEN_WIDTH: i16 = SCREEN_WIDTH / 2;        //half of screen width
const HALF_SCREEN_HEIGHT: i16 = SCREEN_HEIGHT / 2;      //half of screen height
const PIXEL_SCALE: i16 = 4 / RESOLUTION_MULTIPLIER;     //OpenGL pixel scale
const WINDOW_WIDTH: i16 = SCREEN_WIDTH * PIXEL_SCALE;   //OpenGL window width
const WINDOW_HEIGHT: i16 = SCREEN_HEIGHT * PIXEL_SCALE; //OpenGL window height

struct Controls {
  pub forward: bool,
  pub left: bool,
  pub backward: bool,
  pub right: bool,
  pub strafe_left: bool,
  pub strafe_right: bool,
  pub move_lock: bool,
}

impl Controls {
  pub fn new() -> Self {
    Self {
      forward: false,
      left: false,
      backward: false,
      right: false,
      strafe_left: false,
      strafe_right: false,
      move_lock: false,
    }
  }
}

enum Colors {
  Yellow,
  YellowDarker,
  Green,
  GreenDarker,
  Cyan,
  CyanDarker,
  Brown,
  BrownDarker,
  Background,
}

fn int_as_color(number: i8) -> Colors {
  match number {
    0 => Colors::Yellow,
    1 => Colors::YellowDarker,
    2 => Colors::Green,
    3 => Colors::GreenDarker,
    4 => Colors::Cyan,
    5 => Colors::CyanDarker,
    6 => Colors::Brown,
    7 => Colors::BrownDarker,
    8 | _ => Colors::Yellow,
  }
}

fn draw_pixel(canvas: &mut Canvas<video::Window>, x: i16, y: i16, c: Colors) {
  let color = match c {
    Colors::Yellow        => pixels::Color::RGB(255, 255, 0),
    Colors::YellowDarker  => pixels::Color::RGB(160, 160, 0),
    Colors::Green         => pixels::Color::RGB(0, 255, 0),
    Colors::GreenDarker   => pixels::Color::RGB(0, 160, 0),
    Colors::Cyan          => pixels::Color::RGB(0, 255, 255),
    Colors::CyanDarker    => pixels::Color::RGB(0, 160, 160),
    Colors::Brown         => pixels::Color::RGB(160, 100, 0),
    Colors::BrownDarker   => pixels::Color::RGB(110, 50, 0),
    Colors::Background    => pixels::Color::RGB(0, 60, 130),
  };

  for y_offset in 0..PIXEL_SCALE {
    for x_offset in 0..PIXEL_SCALE {
      let _ = canvas.pixel(
        x * PIXEL_SCALE + x_offset,
        WINDOW_HEIGHT - (y + 1) * PIXEL_SCALE + y_offset,
        color,
      );
    }
  }
}

fn move_player(controls: &Controls) {
  if controls.move_lock {
    if controls.forward {
      println!("look up");
    }
    if controls.left {
      println!("look left");
    }
    if controls.backward {
      println!("look down");
    }
    if controls.right {
      println!("look right");
    }
  } else {
    if controls.forward {
      println!("forward");
    }
    if controls.left {
      println!("left");
    }
    if controls.backward {
      println!("backward");
    }
    if controls.right {
      println!("right");
    }
  }
  if controls.strafe_left {
    println!("strafe left");
  }
  if controls.strafe_right {
    println!("strafe right");
  }
}

fn clear_background(canvas: &mut Canvas<video::Window>) {
  for y in 0..SCREEN_HEIGHT as i16 {
    for x in 0..SCREEN_WIDTH as i16 {
      draw_pixel(canvas, x, y, Colors::Background);
    }
  }
}

fn draw_3d(canvas: &mut Canvas<video::Window>) {
  let mut c = 0;

  for y in 0..HALF_SCREEN_HEIGHT {
    for x in 0..HALF_SCREEN_WIDTH {
      draw_pixel(canvas, x, y, int_as_color(c));
      
      c += 1;
      if c > 8 {
        c = 0;
      }
    }
  }
}

fn display_screen(
  canvas: &mut Canvas<video::Window>,
  controls: &Controls,
  delta_time: u128,
) {
  clear_background(canvas);
  move_player(controls);
  draw_3d(canvas);
  print!("delta: {}, fps: {}        \r", delta_time, 1_000_000_000 / delta_time);
}

fn keys_down(controls: &mut Controls, keycode: Keycode) {
  match keycode {
    Keycode::W => controls.forward = true,
    Keycode::A => controls.left = true,
    Keycode::S => controls.backward = true,
    Keycode::D => controls.right = true,
    Keycode::Comma => controls.strafe_left = true,
    Keycode::Period => controls.strafe_right = true,
    Keycode::M => controls.move_lock = true,
    _ => {},
  }
}

fn keys_up(controls: &mut Controls, keycode: Keycode) {
  match keycode {
    Keycode::W => controls.forward = false,
    Keycode::A => controls.left = false,
    Keycode::S => controls.backward = false,
    Keycode::D => controls.right = false,
    Keycode::Comma => controls.strafe_left = false,
    Keycode::Period => controls.strafe_right = false,
    Keycode::M => controls.move_lock = false,
    _ => {},
  }
}

fn init() {
  println!("Game start");
}

fn exit() {
  println!("Game exit");
}

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsys = sdl_context.video()?;
  let window = video_subsys
    .window("Doom Clone", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
    .position_centered()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;

  let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

  canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
  canvas.clear();
  canvas.present();
  
  let mut events = sdl_context.event_pump()?;
  let mut controls = Controls::new();
  let mut previous_frame = std::time::SystemTime::now();

  init();
  
  'main: loop {
    let current_frame = std::time::SystemTime::now();
    let delta_time = current_frame.duration_since(previous_frame)
      .unwrap()
      .as_nanos();
    
    for event in events.poll_iter() {
      match event {
        Event::KeyDown {
          keycode: Some(keycode),
          ..
        } => {
          keys_down(&mut controls, keycode)
        },
        Event::KeyUp {
          keycode: Some(keycode),
          ..
        } => {
          keys_up(&mut controls, keycode)
        },
        Event::Quit { .. } => {
          exit();
          break 'main;
        },
        _ => {}
      }
    }
    
    if !ENABLE_TARGET_FPS || delta_time >= 1_000_000_000 / TARGET_FPS as u128 {
      // Draw frame
      display_screen(&mut canvas, &controls, delta_time);
      canvas.present();

      previous_frame = current_frame;
    }
  }

  Ok(())
}
