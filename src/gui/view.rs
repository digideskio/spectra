use luminance::{Dim2, Flat, Mode, RGBA32F};
use luminance_gl::gl33::{Framebuffer, Pipe, Pipeline, RenderCommand, ShadingCommand, Tess, Texture,
                         Uniform};

use id::Id;
use gui::widget::{Color, FillRectWidget, InterpretedWidget, Rect, RootWidget};
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
  quad: Tess,
  // buffer of rectangular area to raw
  fillrect_buffer: Vec<(Rect, Color)>
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
      quad: quad,
      fillrect_buffer: Vec::new()
    }
  }

  fn clear_buffers(&mut self) {
    self.fillrect_buffer.clear();
  }
}

impl<'a> InterpretedWidget<WidgetView<'a>> for RootWidget<WidgetView<'a>> {
  fn redraw(&self, view: &mut WidgetView<'a>) {
    // clear the previous render’s buffers
    view.clear_buffers();

    // fill up the buffers
    for widget in &self.widgets {
      // TODO
    }

    // make the damn render
    Pipeline::new(&view.framebuffer, [0., 0., 0., 1.], &[], &[], vec![
    ]).run();
  }
}

impl<'a> InterpretedWidget<WidgetView<'a>> for FillRectWidget {
  fn redraw(&self, view: &mut WidgetView<'a>) {
    view.fillrect_buffer.push((self.rect.clone(), self.color.clone()));
  }
}
