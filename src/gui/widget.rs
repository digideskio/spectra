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

/// Class of widgets.
pub trait Widget {
  fn get_rect(&self) -> Rect;
}

/// Class of widgets that can contain other widgets.
///
/// In order to contain other widgets, those widgets must be interpreted.
pub trait WidgetContainer<V>: Widget {
  fn get_widgets(&self) -> &[Box<InterpretedWidget<V>>];
  fn add_widget(&mut self, widget: Box<InterpretedWidget<V>>);
}

/// Class of widget interpretors.
///
/// As-is, a widget is not a usable object because it lacks interpretation. For instance, having a
/// *slider* without a way to render it or treat events is plain useless. This trait solves that
/// by providing interpretation (`V`) to a widget.
pub trait InterpretedWidget<V>: Widget {
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

impl Widget for FillRectWidget {
  fn get_rect(&self) -> Rect {
    self.rect.clone()
  }
}

/// “Entry point” of widgets.
pub struct RootWidget<V> {
  rect: Rect,
  layout: Layout,
  widgets: Vec<Box<InterpretedWidget<V>>>,
}

impl<V> RootWidget<V> {
  pub fn new(w: Px, h: Px, layout: Layout) -> Self {
    RootWidget {
      rect: Rect::new(Pos::new(0, 0), w, h),
      layout: layout,
      widgets: Vec::new()
    }
  }
}

impl<V> Widget for RootWidget<V> {
  fn get_rect(&self) -> Rect {
    self.rect.clone()
  }
}

impl<V> WidgetContainer<V> for RootWidget<V> {
  fn get_widgets(&self) -> &[Box<InterpretedWidget<V>>] {
    &self.widgets
  }

  fn add_widget(&mut self, widget: Box<InterpretedWidget<V>>) {
    self.widgets.push(widget)
  }
}

