use luminance::{Dim2, Flat, Mode, RGBA32F};
use luminance_gl::gl33::{Framebuffer, Pipe, Pipeline, RenderCommand, ShadingCommand, Tess, Texture,
                         Uniform};
use std::rc::Rc;

use id::Id;
use gui::widget::{Color, FillRectWidget, InterpretedWidget, Layout, Pos, Positioning, Rect, RootWidget};
use scene::Scene;
use shader::Program;

const WIDGET_VIEW_RESOLUTION: Uniform<[u32; 2]> = Uniform::new(0);
const WIDGET_VIEW_WIDGET_RECT: Uniform<[i32; 4]> = Uniform::new(1);
const WIDGET_VIEW_WIDGET_COLOR: Uniform<[f32; 3]> = Uniform::new(2);

/// Default widget interpretor state.
pub struct WidgetViewSt<'a> {
  // available framebuffer for the whole GUI
  framebuffer: Framebuffer<Flat, Dim2, Texture<Flat, Dim2, RGBA32F>, ()>,
  // program used to render widgets
  program: Id<'a, Program>,
  // used to render rectangular area
  quad: Tess,
  // buffer of rectangular area to raw
  fillrect_buffer: Vec<(Rect, Color)>
}

impl<'a> WidgetViewSt<'a> {
  pub fn new(w: u32, h: u32, scene: &mut Scene<'a>) -> Self {
    let framebuffer = Framebuffer::new((w, h), 0).unwrap();
    let program = scene.get_id("spectra/gui/default.glsl", vec![
      Uniform::<[u32; 2]>::sem("resolution"),
      Uniform::<[i32; 4]>::sem("widget_rect"),
      Uniform::<[f32; 3]>::sem("widget_color")
    ]).unwrap();
    let quad = Tess::attributeless(Mode::TriangleStrip, 4);

    WidgetViewSt {
      framebuffer: framebuffer,
      program: program,
      quad: quad,
      fillrect_buffer: Vec::new()
    }
  }
}

/// Default widget interpretor.
pub struct WidgetView<'a> {
  state: &'a mut WidgetViewSt<'a>,
  program: Rc<Program>
}

impl<'a> WidgetView<'a> {
  pub fn new(st: &'a mut WidgetViewSt<'a>, scene: &mut Scene<'a>) -> Self {
    let program = scene.get_by_id(&st.program).unwrap();

    WidgetView {
      state: st,
      program: program
    }
  }

  fn clear_buffers(&mut self) {
    self.state.fillrect_buffer.clear();
  }
}

impl<'a> InterpretedWidget<WidgetView<'a>> for RootWidget<WidgetView<'a>> {
  fn redraw(&self, _: Rect, view: &mut WidgetView<'a>) {
    // clear the previous render’s buffers
    view.clear_buffers();

    redraw_children(self.layout.clone(), self.rect.clone(), &self.widgets, view);

    // make the damn render
    Pipeline::new(&view.state.framebuffer, [0., 0., 0., 1.], &[], &[], vec![
      Pipe::new(|program| {
          program.update(&WIDGET_VIEW_RESOLUTION, [self.rect.width() as u32, self.rect.height() as u32]);
        },
        ShadingCommand::new(&view.program, vec![
        ]))
    ]).run();
  }
}

impl<'a> InterpretedWidget<WidgetView<'a>> for FillRectWidget {
  fn redraw(&self, computed_rect: Rect, view: &mut WidgetView<'a>) {
    view.state.fillrect_buffer.push((computed_rect.clone(), self.color.clone()));
  }
}

/// Redraw the children widgets of a widget.
fn redraw_children<V>(parent_layout: Layout,
                      parent_rect: Rect,
                      widgets: &[Box<InterpretedWidget<V>>],
                      view: &mut V) {
  match parent_layout {
    Layout::Horizontal(ref positioning) => {
      match *positioning {
        Positioning::First => {
          let mut lower = parent_rect.lower.clone();

          for widget in widgets.iter() {
            let widget_rect = widget.rect();
            let w = widget_rect.width();
            let rect = Rect::new(lower.clone(), Pos::new(lower.x + w, parent_rect.upper.y));

            widget.redraw(rect, view);

            lower.x += w;
          }
        },

        Positioning::Last => {
          let mut upper = parent_rect.upper.clone();

          for widget in widgets.iter() {
            let widget_rect = widget.rect();
            let w = widget_rect.width();
            let rect = Rect::new(Pos::new(upper.x - w, parent_rect.lower.y), upper.clone());

            widget.redraw(rect, view);

            upper.x -= w;
          }
        },

        Positioning::Tiling => {
          let w = ((parent_rect.width() as f32) / widgets.len() as f32) as i32;
          let mut lower = parent_rect.lower;

          for widget in widgets.iter() {
            let rect = Rect::new(lower.clone(), Pos::new(lower.x + w, parent_rect.upper.y));

            widget.redraw(rect, view);

            lower.x += w;
          }
        }
      }
    },

    Layout::Vertical(ref positioning) => {
      match *positioning {
        Positioning::First => {
          let mut lower = parent_rect.lower.clone();

          for widget in widgets.iter() {
            let widget_rect = widget.rect();
            let h = widget_rect.height();
            let rect = Rect::new(lower.clone(), Pos::new(parent_rect.upper.x, lower.y + h));

            widget.redraw(rect, view);

            lower.y += h;
          }
        },

        Positioning::Last => {
          let mut upper = parent_rect.upper.clone();

          for widget in widgets.iter() {
            let widget_rect = widget.rect();
            let h = widget_rect.height();
            let rect = Rect::new(Pos::new(parent_rect.lower.x, upper.y - h), upper.clone());

            widget.redraw(rect, view);

            upper.y -= h;
          }
        },

        Positioning::Tiling => {
          let h = ((parent_rect.height() as f32) / widgets.len() as f32) as i32;
          let mut lower = parent_rect.lower;

          for widget in widgets.iter() {
            let rect = Rect::new(lower.clone(), Pos::new(parent_rect.upper.x, lower.y + h));

            widget.redraw(rect, view);

            lower.y += h;
          }
        }
      }
    },

    Layout::Floating => {
      for widget in widgets.iter() {
        let widget_rect = widget.rect();
        let lower = parent_rect.lower + widget_rect.lower;
        let dim = Pos::new(widget_rect.width(), widget_rect.height());
        let rect = Rect::new(lower, lower + dim);

        widget.redraw(rect, view);
      }
    }
  }
}
