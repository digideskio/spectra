use luminance::{Dim2, Flat, Mode, RGBA32F};
use luminance_gl::gl33::{Framebuffer, Pipe, Pipeline, RenderCommand, ShadingCommand, Tess, Texture,
                         Uniform};

use id::Id;
use gui::widget::{Color, FillRectWidget, InterpretedWidget, Layout, Pos, Positioning, Rect, RootWidget};
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
  fn redraw(&self, _: Rect, view: &mut WidgetView<'a>) {
    // clear the previous render’s buffers
    view.clear_buffers();

    // redraw all the children!
    match self.layout {
      Layout::Horizontal(positioning) => {
        match positioning {
          Positioning::First => {
            let mut lower = self.rect.lower.clone();

            for widget in &self.widgets {
              let widget_rect = widget.rect();
              let w = widget_rect.upper.x - widget_rect.lower.x;
              let rect = Rect::new(lower.clone(), Pos::new(lower.x + w, self.rect.upper.y));

              widget.redraw(rect, view);

              lower.x += w;
            }
          },

          Positioning::Last => {
            let mut upper = self.rect.upper.clone();

            for widget in &self.widgets {
              let widget_rect = widget.rect();
              let w = widget_rect.upper.x - widget_rect.lower.x;
              let rect = Rect::new(Pos::new(upper.x - w, self.rect.lower.y), upper.clone());

              widget.redraw(rect, view);

              upper.x -= w;
            }
          },

          Positioning::Tiling => {
            let widget_w = (((self.rect.upper.x - self.rect.lower.x) as f32) / self.widgets.len() as f32) as i32;
            let mut lower = self.rect.lower;

            for widget in &self.widgets {
              let rect = Rect::new(lower.clone(), Pos::new(widget_w, lower.y));
            }
          }
        }
      },

      Layout::Vertical(positioning) => {
        match positioning {
          Positioning::First => {
            let mut upper = self.rect.upper.clone();

            for widget in &self.widgets {
              let widget_rect = widget.rect();
              let h = widget_rect.lower.y - widget_rect.upper.y;
              let rect = Rect::new(Pos::new(self.rect.lower.x, upper.y + h), upper.clone);
            }
          }
        }
      }
    }


    // make the damn render
    Pipeline::new(&view.framebuffer, [0., 0., 0., 1.], &[], &[], vec![
    ]).run();
  }
}

impl<'a> InterpretedWidget<WidgetView<'a>> for FillRectWidget {
  fn redraw(&self, computed_rect: Rect, view: &mut WidgetView<'a>) {
    view.fillrect_buffer.push((computed_rect.clone(), self.color.clone()));
  }
}
