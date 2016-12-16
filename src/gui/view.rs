use luminance::{Dim2, Flat, Mode, RGBA32F};
use luminance_gl::gl33::{Framebuffer, Tess, Texture};

use id::Id;
use gui::widget::FillRectWidget;
use scene::Scene;
use shader::Program;

/// Default widget interpretor.
pub struct WidgetView<'a> {
  // available framebuffer for the all GUI
  framebuffer: Framebuffer<Flat, Dim2, Texture<Flat, Dim2, RGBA32F>, ()>,
  // program used to render widgets
  program: Id<'a, Program>,
  // used to render rectangular area
  quad: Tess
}

impl<'a> WidgetView<'a> {
  pub fn new(w: u32, h: u32, scene: &mut Scene<'a>) -> Self {
    let framebuffer = Framebuffer::new((w, h), 0).unwrap();
    let program = scene.get_id("spectra/gui/default.glsl", vec![]).unwrap();
    let quad = Tess::attributeless(Mode::TriangleStrip, 4);

    WidgetView {
      framebuffer: framebuffer,
      program: program,
      quad: quad
    }
  }
}
