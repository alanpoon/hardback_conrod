use cardgame_widgets::custom_widget::arrange_list::{Hoverable, ImageHover, Arrangeable, TimesClicked};
use conrod::{widget, Color, Colorable, Borderable, Positionable, UiCell, Widget, event, input,
             image, Theme, Sizeable};
use conrod::position::{Rect, Scalar, Dimensions, Point};
use cardgame_widgets::custom_widget::dragdrop_list::Draggable;
use cardgame_widgets::sprite::{Spriteable, spriteable_rect};
use conrod::widget::Rectangle;

/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ItemWidget<H, S>
    where H: Hoverable,
          S: Spriteable
{
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub image: H,
    pub bordered: bool,
    /// See the Style struct below.
    style: Style,
    pub toggle_image: H,
    pub spinner_image_id: Option<(image::Id, S)>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the Button's pressable area.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<Color>,
    /// Width of the border surrounding the Image
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the border.
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
    /// Dragable
    #[conrod(default="false")]
    pub draggable: Option<bool>,
}

widget_ids! {
    struct Ids {
        background,
        rect,
        image,
        spinner
    }
}

/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
    drag: Drag,
    toggle_bool: bool,
}

impl<H, S> ItemWidget<H, S>
    where H: Hoverable,
          S: Spriteable
{
    /// Create a button context to be built upon.
    pub fn new(image: H, toggle_image: H) -> Self {
        ItemWidget {
            image: image,
            toggle_image: toggle_image,
            spinner_image_id: None,
            common: widget::CommonBuilder::default(),
            bordered: false,
            style: Style::default(),
        }
    }

    /// The spinner image overlay displayed when the button is held for 2 seconds.
    pub fn spinner_image(mut self, id: image::Id, sprite: S) -> Self {
        self.spinner_image_id = Some((id, sprite));
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<H, S> Widget for ItemWidget<H, S>
    where H: Hoverable,
          S: Spriteable
{
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = TimesClicked;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            drag: Drag::None,
            toggle_bool: false,
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, ui, .. } = args;
        // Finally, we'll describe how we want our widget drawn by simply instantiating the
        // necessary primitive graphics widgets.
        //
        let (_, _, w, h) = rect.x_y_w_h();
        let border = if self.bordered {
            self.style.border(ui.theme())
        } else {
            0.0
        };
        let toggle_bool = state.toggle_bool;
        rectangle_fill(id,
                       state.ids.background,
                       rect,
                       self.style.color(&ui.theme),
                       ui);
        if self.bordered {
            let border_color = self.style.border_color(ui.theme());
            let _style = widget::line::Style {
                maybe_pattern: None,
                maybe_color: Some(border_color),
                maybe_thickness: Some(border),
                maybe_cap: None,
            };
            Rectangle::outline_styled([w, h],_style).middle_of(id)
            .parent(id)
            //.graphics_for(id)
            .set(state.ids.rect, ui);
        }
        // Instantiate the image.
        let widget_ih = if toggle_bool {
            self.toggle_image
        } else {
            self.image
        };
        let t = ImageHover::new(widget_ih)
            .middle_of(id)
            .padded_wh_of(id, border)
            .parent(id)
            .graphics_for(id)
            .set(state.ids.image, ui);
        let mut drag = state.drag;
        let mut toggle_bool = state.toggle_bool;
        update_drag(id, &mut drag, ui);
        let draw_spinner_index = update_toggle_bool_spinner_index(&mut drag, &mut toggle_bool);
        state.update(|state| {
                         state.drag = drag;
                         state.toggle_bool = toggle_bool
                     });

        if let Some(spinner_index) = draw_spinner_index {
            draw_spinner_op(id,
                            state.ids.spinner,
                            self.spinner_image_id,
                            spinner_index,
                            ui);
        }
        t
    }
    fn drag_area(&self, dim: Dimensions, style: &Style, _theme: &Theme) -> Option<Rect> {
        if let Some(_) = style.draggable {
            Some(Rect::from_xy_dim([0.0, 0.0], dim))
        } else {
            None
        }
    }
}
fn rectangle_fill(button_id: widget::Id,
                  rectangle_id: widget::Id,
                  rect: Rect,
                  color: Color,
                  ui: &mut UiCell) {
    // BorderedRectangle widget.
    let dim = rect.dim();
    widget::Rectangle::fill_with(dim, color)
        .middle_of(button_id)
        .graphics_for(button_id)
        .set(rectangle_id, ui);
}
// Track whether some sort of dragging is currently occurring.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Drag {
    /// The drag is currently selecting a range of text.
    Selecting(u16, Point),
    None,
    Terminate,
}
fn update_drag(button_id: widget::Id, drag: &mut Drag, ui: &UiCell) {
    for widget_event in ui.widget_input(button_id).events() {
        match widget_event {
            event::Widget::Press(press) => {
                match press.button {
                    event::Button::Mouse(input::MouseButton::Left, point) => {
                        match drag {
                            &mut Drag::Selecting(_, _) => {}
                            &mut Drag::None => {
                                *drag = Drag::Selecting(0, point);
                            }
                            &mut Drag::Terminate => {}
                        }
                    }
                    _ => {}
                }
            }
            event::Widget::Click(click) => {
                match (click, drag.clone()) {
                    (event::Click { button: input::MouseButton::Left, .. }, Drag::Terminate) => {
                        *drag = Drag::None;
                    }
                    _ => {}
                }
            }
            event::Widget::Release(release) => {
                if let event::Button::Mouse(input::MouseButton::Left, _) = release.button {
                    match drag {
                        &mut Drag::Selecting(_, _) => {
                            *drag = Drag::Terminate;
                        }
                        _ => {}
                    }
                }
            }
            event::Widget::Drag(drag_event) if drag_event.button == input::MouseButton::Left => {
                match drag {
                    &mut Drag::Selecting(_, ref mut point) => {
                        *point = drag_event.to;
                    }
                    _ => {}
                }
            }
            _ => if let Drag::None = *drag {},
        }
    }

}

fn draw_spinner_op<H: Spriteable>(button_id: widget::Id,
                                  spinner_id: widget::Id,
                                  spinner_image: Option<(image::Id, H)>,
                                  spinner_index: u16,
                                  ui: &mut UiCell) {
    if let Some((spinner_image, _sprite)) = spinner_image {
        let _rect = spriteable_rect(_sprite, spinner_index as f64);
        widget::Image::new(spinner_image)
            .source_rectangle(Rect::from_corners(_rect.0, _rect.1))
            .w_h(40.0, 40.0)
            .middle_of(button_id)
            .set(spinner_id, ui);
    }

}
fn update_toggle_bool_spinner_index(drag: &mut Drag, toggle_bool: &mut bool) -> Option<u16> {
    match drag {
        &mut Drag::Selecting(ref mut spinner_index, _) => {
            if *spinner_index >= 60 {
                if *toggle_bool {
                    *toggle_bool = false;
                } else {
                    *toggle_bool = true;
                }

                *spinner_index = 0;
                None
            } else {

                *spinner_index += 1;
                Some(spinner_index.clone())
            }
        }
        _ => None,
    }
}
impl<H, S> Arrangeable for ItemWidget<H, S>
    where H: Hoverable,
          S: Spriteable
{
    fn selectable(mut self) -> Self {
        self.bordered = true;
        self
    }
}
impl<H, S> Colorable for ItemWidget<H, S>
    where H: Hoverable,
          S: Spriteable
{
    builder_method!(color { style.color = Some(Color) });
}
impl<H, S> Borderable for ItemWidget<H, S>
    where H: Hoverable,
          S: Spriteable
{
    builder_methods!{
        border { style.border = Some(Scalar) }
        border_color { style.border_color = Some(Color) }
    }
}
