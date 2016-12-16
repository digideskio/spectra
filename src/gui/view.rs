use luminance::{Dim2, Flat, Mode, RGBA32F};
use luminance_gl::gl33::{Framebuffer, Tess, Texture, Uniform};

use id::Id;
use gui::widget::FillRectWidget;
use scene::Scene;
use shader::Program;

const WIDGET_VIEW_RESOLUTION: Uniform<[u32; 2]> = Uniform::new(0);
const WIDGET_VIEW_WIDGET_RECT: Uniform<[i32; 4]> = Uniform::new(1);
const WIDGET_VIEW_WIDGET_COLOR: Uniform<[f32; 3]> = Uniform::new(2);

/// Default widget interpretor.
pub struct WidgetView<'a> {
  // available framebuffer for the whole GUI
  framebuffer: Framebuffer<Flat, Dim2, Texture<Flat, Dim2, RGBA32F>, ()>,
  // program used to render widgets
  program: Id<'a, Program>,
  // used to render rectangular area
  quad: Tess
}

impl<'a> WidgetView<'a> {
  pub fn new(w: u32, h: u32, scene: &mut Scene<'a>) -> Self {
    let framebuffer = Framebuffer::new((w, h), 0).unwrap();
    let program = scene.get_id("spectra/gui/default.glsl", vec![
      Uniform::<[u32; 2]>::sem("resolution"),
      Uniform::<[i32; 4]>::sem("widget_rect"),
      Uniform::<[f32; 3]>::sem("widget_color")
    ]).unwrap();
    let quad = Tess::attributeless(Mode::TriangleStrip, 4);

    WidgetView {
      framebuffer: framebuffer,
      program: program,
      quad: quad
    }
  }
}

