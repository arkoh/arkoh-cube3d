use gl;
use gl::*;


pub struct Object
{
  priv vertices: [i32, ..2],
  priv color:    [f32, ..3],


}

impl Object
{
  pub fn new(v1: i32, v2: i32, r: f32, g: f32, b: f32) -> Object
  {
    Object {
      vertices: [v1, v2],
      color:    [r, g, b]
    }
  }


  pub fn draw(&self, color_location: i32)
  {
      gl::Uniform3f(color_location, self.color[0], self.color[1], self.color[2]);
      gl::DrawArrays(gl::TRIANGLES, self.vertices[0], self.vertices[1]);
  }


  pub fn set_color(&mut self, r: f32, g: f32, b: f32)
  {
    self.color[0] = r;
    self.color[1] = g;
    self.color[2] = b;
  }
}


