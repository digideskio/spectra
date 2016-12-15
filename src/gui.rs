use std::marker::PhantomData;
use linear::Vector2;

pub use std::rc::Rc;
pub use color::Color;

pub trait Widget {
  fn get_rect(&self) -> Rect;
  fn redraw<V>(&self, view: &mut V) where V: WidgetView<Self> {
    view.redraw(self)
  }
}

pub trait WidgetView<W: ?Sized> {
  fn redraw(&mut self, widget: &W);
}

pub type Px = u32;
pub type Pos = Vector2<Px>;

pub mod layout {
  pub struct Horizontal;
  pub struct Vertical;
  pub struct Floating;
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

pub struct FillRectWidget<L> {
  color: Color,
  rect: Rect,
  _l: PhantomData<L>
}

impl<L> FillRectWidget<L> {
  pub fn new(rect: Rect, color: Color) -> Self {
    FillRectWidget {
      color: color,
      rect: rect,
      _l: PhantomData
    }
  }
}

impl<L> Widget for FillRectWidget<L> {
  fn get_rect(&self) -> Rect {
    self.rect.clone()
  }
}

pub struct GUI<L> {
  rect: Rect,
  widgets: Vec<Rc<Widget>>,
  _l: PhantomData<L>
}

impl<L> GUI<L> {
  pub fn new(w: Px, h: Px) -> Self {
    GUI {
      rect: Rect::new(Pos::new(0, 0), w, h),
      _l: PhantomData
    }
  }
}

impl<L> Widget for GUI<L> {
  fn get_rect(&self) -> Rect {
    self.rect.clone()
  }
}

// TESTS ONLY
pub struct ConsoleView;

impl<L> WidgetView<GUI<L>> for ConsoleView {
  fn redraw(&mut self, widget: &GUI<L>) {
    deb!("redrawing GUI: {:#?}", widget.rect);
  }
}
