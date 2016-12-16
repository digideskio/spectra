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

/// Class of usable widgets.
///
/// As-is, a widget is not a usable object because it lacks interpretation. For instance, having a
/// *slider* without a way to render it or treat events is plain useless. This trait solves that
/// by providing interpretation (`V`) to a widget.
pub trait Widget<V> {
  fn redraw(&self, view: &mut V);
}

/// A simple widget representing a colored rectangular area.
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

/// “Entry point” of widgets.
pub struct RootWidget<V> {
  rect: Rect,
  layout: Layout,
  widgets: Vec<Box<Widget<V>>>,
}

impl<V> RootWidget<V> {
  pub fn new(w: Px, h: Px, layout: Layout) -> Self {
    RootWidget {
      rect: Rect::new(Pos::new(0, 0), w, h),
      layout: layout,
      widgets: Vec::new()
    }
  }

  pub fn add_fill_rect(&mut self, widget: Box<Widget<V>>) {
    self.widgets.push(widget)
  }
}

// TESTS ONLY
pub struct ConsoleView;

impl Widget<ConsoleView> for RootWidget<ConsoleView> {
  fn redraw(&self, view: &mut ConsoleView) {
    deb!("{:#?} {:#?}", self.rect, self.layout);
    
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
