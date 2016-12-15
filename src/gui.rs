use std::marker::PhantomData;
use linear::Vector2;

pub use std::rc::Rc;
pub use color::Color;

pub trait Widget {
  fn get_rect(&self) -> Rect;
}

pub type Px = u32;
pub type Pos = Vector2<Px>;

pub enum Layout {
  Horizontal,
  Vertical,
  Floating
}

// Upper-left is origin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rect {
  lower: Pos,
  upper: Pos
}

impl Rect {
  pub fn new(upper_left: Pos, w: Px, h: Px) -> Self {
    Rect {
      lower: Pos::new(upper_left.x, upper_left.y + h),
      upper: Pos::new(upper_left.x + w, upper_left.y)
    }
  }
}

#[derive(Debug)]
pub struct FillRectWidget {
  color: Color,
  rect: Rect
}

impl FillRectWidget {
  pub fn new(rect: Rect, color: Color) -> Self {
    FillRectWidget {
      color: color,
      rect: rect
    }
  }
}

impl Widget for FillRectWidget {
  fn get_rect(&self) -> Rect {
    self.rect.clone()
  }
}

pub struct GUI {
  rect: Rect,
  layout: Layout,
  color: Color,
  widgets: Vec<Rc<FillRectWidget>>,
}

impl GUI {
  pub fn new(w: Px, h: Px, layout: Layout, color: Color) -> Self {
    GUI {
      rect: Rect::new(Pos::new(0, 0), w, h),
      layout: layout,
      color: color,
      widgets: Vec::new()
    }
  }

  pub fn add_fill_rect(&mut self, widget: Rc<FillRectWidget>) {
    self.widgets.push(widget)
  }
}

impl Widget for GUI {
  fn get_rect(&self) -> Rect {
    self.rect.clone()
  }
}

pub trait WidgetView<W> {
  fn redraw(&mut self, widget: &W);
}

// TESTS ONLY
pub struct ConsoleView;

impl WidgetView<GUI> for ConsoleView {
  fn redraw(&mut self, widget: &GUI) {
    deb!("redrawing GUI: {:#?}", widget.rect);

    for child in &widget.widgets {
      <ConsoleView as WidgetView<FillRectWidget>>::redraw(self, child);
    }
  }
}

impl WidgetView<FillRectWidget> for ConsoleView {
  fn redraw(&mut self, widget: &FillRectWidget) {
    deb!("redrawing FillRectWidget: {:#?}", widget);
  }
}
