#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color { pub r:f32, pub g:f32, pub b:f32, pub a:f32 }
impl Color { pub const fn new(r:f32,g:f32,b:f32,a:f32)->Self{Self{r,g,b,a}} }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 { pub x:f32, pub y:f32 }
impl Vec2 { pub const fn new(x:f32,y:f32)->Self{Self{x,y}} }
