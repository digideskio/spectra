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

/// Rectangular area in 2D. The origin of the space coordinates is at the upper-left corner.
///
///  ·----> (x)
///  |
///  |   L---------A
///  v   |         |
/// (y)  |         |
///      |         |
///      B---------U
///
/// Here, the point *U* is called the *lower* point and the point *U* is called the *upper* point.
/// `(A - L).x = width` and `(B - L).y = height`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rect {
  pub lower: Pos,
  pub upper: Pos
}

impl Rect {
  pub fn new(lower: Pos, upper: Pos) -> Self {
    let mut out_lower: Pos;
    let mut out_upper: Pos;

    if lower.x <= upper.x {
      out_lower.x = lower.x;
      out_upper.x = upper.x;
    } else {
      out_lower.x = upper.x;
      out_upper.x = lower.x;
    }

    if lower.y <= upper.y {
      out_lower.y = lower.y;
      out_upper.y = upper.y;
    } else {
      out_lower.y = upper.y;
      out_upper.y = lower.y;
    }

    Rect {
      lower: out_lower,
      upper: out_upper
    }
  }

  pub fn new_wh(upper_left: Pos, w: Px, h: Px) -> Self {
    Rect {
      lower: Pos::new(upper_left.x, upper_left.y + h),
      upper: Pos::new(upper_left.x + w, upper_left.y)
    }
  }
}

/// Class of widgets.
pub trait Widget {
  fn name(&self) -> String;
  fn rect(&self) -> Rect;
}

/// Class of widgets that can contain other widgets.
///
/// In order to contain other widgets, those widgets must be interpreted.
pub trait WidgetContainer<V>: Widget {
  fn add_widget(&mut self, widget: Box<InterpretedWidget<V>>);
}

/// Class of widget interpretors.
///
/// As-is, a widget is not a usable object because it lacks interpretation. For instance, having a
/// *slider* without a way to render it or treat events is plain useless. This trait solves that
/// by providing interpretation (`V`) to a widget.
pub trait InterpretedWidget<V>: Widget {
  fn redraw(&self, computed_rect: Rect, view: &mut V);
}

/// A simple widget representing a colored rectangular area.
#[derive(Debug)]
pub struct FillRectWidget {
  name: String,
  pub color: Color,
  pub rect: Rect
}

impl FillRectWidget {
  pub fn new(name: &str, rect: Rect, color: Color) -> Self {
    FillRectWidget {
      name: name.to_owned(),
      color: color,
      rect: rect
    }
  }
}

impl Widget for FillRectWidget {
  fn name(&self) -> String {
    self.name.clone()
  }

  fn rect(&self) -> Rect {
    self.rect.clone()
  }
}

/// “Entry point” of widgets.
pub struct RootWidget<V> {
  pub rect: Rect,
  pub layout: Layout,
  pub widgets: Vec<Box<InterpretedWidget<V>>>,
}

impl<V> RootWidget<V> {
  pub fn new(w: Px, h: Px, layout: Layout) -> Self {
    RootWidget {
      rect: Rect::new_wh(Pos::new(0, 0), w, h),
      layout: layout,
      widgets: Vec::new()
    }
  }
}

impl<V> Widget for RootWidget<V> {
  fn name(&self) -> String {
    "".to_owned()
  }

  fn rect(&self) -> Rect {
    self.rect.clone()
  }
}

impl<V> WidgetContainer<V> for RootWidget<V> {
  fn add_widget(&mut self, widget: Box<InterpretedWidget<V>>) {
    let _ = self.widgets.push(widget);
  }
}

