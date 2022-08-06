/*
//------------------------------------------------------------------------------
//--------------------------Code By: 3DSage-------------------------------------
//----------------Video tutorial on YouTube-3DSage------------------------------
//------------------------------------------------------------------------------

#include <math.h>
#include <stdio.h>
#include <GL/glut.h> 

#define res        1                        //0=160x120 1=360x240 4=640x480
#define SW         160*res                  //screen width
#define SH         120*res                  //screen height
#define SW2        (SW/2)                   //half of screen width
#define SH2        (SH/2)                   //half of screen height
#define pixelScale 4/res                    //OpenGL pixel scale
#define GLSW       (SW*pixelScale)          //OpenGL window width
#define GLSH       (SH*pixelScale)          //OpenGL window height
//------------------------------------------------------------------------------
typedef struct 
{
 int fr1,fr2;           //frame 1 frame 2, to create constant frame rate
}time; time T;

typedef struct 
{
 int w,s,a,d;           //move up, down, left, right
 int sl,sr;             //strafe left, right 
 int m;                 //move up, down, look up, down
}keys; keys K;
//------------------------------------------------------------------------------

void pixel(int x,int y, int c)                  //draw a pixel at x/y with rgb
{int rgb[3];
 if(c==0){ rgb[0]=255; rgb[1]=255; rgb[2]=  0;} //Yellow	
 if(c==1){ rgb[0]=160; rgb[1]=160; rgb[2]=  0;} //Yellow darker	
 if(c==2){ rgb[0]=  0; rgb[1]=255; rgb[2]=  0;} //Green	
 if(c==3){ rgb[0]=  0; rgb[1]=160; rgb[2]=  0;} //Green darker	
 if(c==4){ rgb[0]=  0; rgb[1]=255; rgb[2]=255;} //Cyan	
 if(c==5){ rgb[0]=  0; rgb[1]=160; rgb[2]=160;} //Cyan darker
 if(c==6){ rgb[0]=160; rgb[1]=100; rgb[2]=  0;} //brown	
 if(c==7){ rgb[0]=110; rgb[1]= 50; rgb[2]=  0;} //brown darker
 if(c==8){ rgb[0]=  0; rgb[1]= 60; rgb[2]=130;} //background 
 glColor3ub(rgb[0],rgb[1],rgb[2]); 
 glBegin(GL_POINTS);
 glVertex2i(x*pixelScale+2,y*pixelScale+2);
 glEnd();
}

void movePlayer()
{
 //move up, down, left, right
 if(K.a ==1 && K.m==0){ printf("left\n");}  
 if(K.d ==1 && K.m==0){ printf("right\n");}
 if(K.w ==1 && K.m==0){ printf("up\n");}
 if(K.s ==1 && K.m==0){ printf("down\n");}
 //strafe left, right
 if(K.sr==1){ printf("strafe left\n");}
 if(K.sl==1){ printf("strafe right\n");}
 //move up, down, look up, look down
 if(K.a==1 && K.m==1){ printf("look up\n");}
 if(K.d==1 && K.m==1){ printf("look down\n");}
 if(K.w==1 && K.m==1){ printf("move up\n");}
 if(K.s==1 && K.m==1){ printf("move down\n");}
}

void clearBackground() 
{int x,y;
 for(y=0;y<SH;y++)
 { 
  for(x=0;x<SW;x++){ pixel(x,y,8);} //clear background color
 }	
}

int tick;
void draw3D()
{int x,y,c=0;
 for(y=0;y<SH2;y++)
 {
  for(x=0;x<SW2;x++)
  {
   pixel(x,y,c); 
   c+=1; if(c>8){ c=0;}
  }
 }
 //frame rate
 tick+=1; if(tick>20){ tick=0;} pixel(SW2,SH2+tick,0); 
}

void display() 
{int x,y;
 if(T.fr1-T.fr2>=50)                        //only draw 20 frames/second
 { 
  clearBackground();
  movePlayer();
  draw3D(); 

  T.fr2=T.fr1;   
  glutSwapBuffers(); 
  glutReshapeWindow(GLSW,GLSH);             //prevent window scaling
 }

 T.fr1=glutGet(GLUT_ELAPSED_TIME);          //1000 Milliseconds per second
 glutPostRedisplay();
} 

void KeysDown(unsigned char key,int x,int y)   
{ 
 if(key=='w'==1){ K.w =1;} 
 if(key=='s'==1){ K.s =1;} 
 if(key=='a'==1){ K.a =1;} 
 if(key=='d'==1){ K.d =1;} 
 if(key=='m'==1){ K.m =1;} 
 if(key==','==1){ K.sr=1;} 
 if(key=='.'==1){ K.sl=1;} 
}
void KeysUp(unsigned char key,int x,int y)
{ 
 if(key=='w'==1){ K.w =0;}
 if(key=='s'==1){ K.s =0;}
 if(key=='a'==1){ K.a =0;}
 if(key=='d'==1){ K.d =0;}
 if(key=='m'==1){ K.m =0;}
 if(key==','==1){ K.sr=0;} 
 if(key=='.'==1){ K.sl=0;}
}

void init()
{       
}

int main(int argc, char* argv[])
{
 glutInit(&argc, argv);
 glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGB);
 glutInitWindowPosition(GLSW/2,GLSH/2);
 glutInitWindowSize(GLSW,GLSH);
 glutCreateWindow(""); 
 glPointSize(pixelScale);                        //pixel size
 gluOrtho2D(0,GLSW,0,GLSH);                      //origin bottom left
 init();
 glutDisplayFunc(display);
 glutKeyboardFunc(KeysDown);
 glutKeyboardUpFunc(KeysUp);
 glutMainLoop();
 return 0;
}
*/

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{pixels, video};

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::Canvas;

//const TARGET_FPS: u64 = 20; No longer works
const RESOLUTION_MULTIPLIER: u32 = 8;                   //0=160x120 1=360x240 4=640x480
const SCREEN_WIDTH: u32 = 160 * RESOLUTION_MULTIPLIER;  //screen width
const SCREEN_HEIGHT: u32 = 120 * RESOLUTION_MULTIPLIER; //screen height
const HALF_SCREEN_WIDTH: u32 = SCREEN_WIDTH / 2;        //half of screen width
const HALF_SCREEN_HEIGHT: u32 = SCREEN_HEIGHT / 2;      //half of screen height
const PIXEL_SCALE: u32 = 4 / RESOLUTION_MULTIPLIER;     //OpenGL pixel scale
const WINDOW_WIDTH: u32 = SCREEN_WIDTH * PIXEL_SCALE;   //OpenGL window width
const WINDOW_HEIGHT: u32 = SCREEN_HEIGHT * PIXEL_SCALE; //OpenGL window height

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

  let _ = canvas.pixel(x, y, color);
}

fn move_player() {}

fn clear_background(canvas: &mut Canvas<video::Window>) {
  for x in 0..SCREEN_WIDTH as i16 {
    for y in 0..SCREEN_HEIGHT as i16 {
      draw_pixel(canvas, x, y, Colors::Background);
    }
  }
}

fn draw_3d(canvas: &mut Canvas<video::Window>) {}

fn display_screen(canvas: &mut Canvas<video::Window>) {
  clear_background(canvas);
  move_player();
  draw_3d(canvas);
}

fn keys_down() {}

fn keys_up() {}

fn init() {
  println!("Game start");
}

fn exit() {}

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsys = sdl_context.video()?;
  let window = video_subsys
    .window("Doom Clone", SCREEN_WIDTH, SCREEN_HEIGHT)
    .position_centered()
    .opengl()
    .build()
    .map_err(|e| e.to_string())?;

  let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

  canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
  canvas.clear();
  canvas.present();

  let mut events = sdl_context.event_pump()?;

  init();

  'main: loop {
    for event in events.poll_iter() {
      match event {
        Event::Quit { .. } => break 'main,
        _ => {}
      }
    }

    display_screen(&mut canvas);
    canvas.present();
  }

  Ok(())
}
