use linear::Vector2;

pub use color::Color;

pub type Px = i32;
pub type Pos = Vector2<Px>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Layout {
  Horizontal(Positioning),
  Vertical(Positioning),
  Floating
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Positioning {
  First,
  Last,
  Tiling
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

pub trait Widget<V> {
  fn redraw(&self, view: &mut V);
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

pub struct GUI<V> {
  rect: Rect,
  layout: Layout,
  color: Color,
  widgets: Vec<Box<Widget<V>>>,
}

impl<V> GUI<V> {
  pub fn new(w: Px, h: Px, layout: Layout, color: Color) -> Self {
    GUI {
      rect: Rect::new(Pos::new(0, 0), w, h),
      layout: layout,
      color: color,
      widgets: Vec::new()
    }
  }

  pub fn add_fill_rect(&mut self, widget: Box<Widget<V>>) {
    self.widgets.push(widget)
  }
}

// TESTS ONLY
pub struct ConsoleView;

impl Widget<ConsoleView> for GUI<ConsoleView> {
  fn redraw(&self, view: &mut ConsoleView) {
    deb!("{:#?} {:#?} {:#?}", self.rect, self.layout, self.color);
    
    for widget in &self.widgets {
      widget.redraw(view);
    }
  }
}

impl Widget<ConsoleView> for FillRectWidget {
  fn redraw(&self, _: &mut ConsoleView) {
    deb!("{:#?}", self);
  }
}
