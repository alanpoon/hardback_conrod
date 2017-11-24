//! The `WildCard` widget and related items.

use conrod::{Color, Colorable, Borderable, Positionable, UiCell, Widget, event, input, image, Theme,
             Sizeable};
use conrod::position::{self, Rect, Scalar, Dimensions, Point};
use conrod::widget;
use cardgame_widgets::custom_widget::dragdrop_list::Draggable;
use cardgame_widgets::sprite::{Spriteable, spriteable_rect};
use conrod_keypad::custom_widget::text_edit;

/// The `WildCard` displays an `Image` on top.

pub struct Image<'a, H>
    where H: Spriteable
{
    /// The id of the `Image` to be used.
    pub image_id: image::Id,
    /// The image displayed when the button is held for 1 seconds.
    pub toggle_image_id: Option<image::Id>,
    /// The image overlay on the mouse while is held for more than 1/16 seconds
    pub spinner_image_id: Option<(image::Id, H)>,
    /// Alphaphet
    pub wild: Option<&'a mut String>,
}
/// A pressable button widget whose reaction is triggered upon release.
#[derive(WidgetCommon)]
pub struct WildCard<S> {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    /// Whether the `WildCard` is a `Flat` color or an `Image`.
    pub show: S,
    /// Unique styling parameters for the WildCard.
    pub style: Style,
}

/// Unique styling for the WildCard.
#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the WildCard's pressable area.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<Color>,
    /// Width of the border surrounding the button
    #[conrod(default = "theme.border_width")]
    pub border: Option<Scalar>,
    /// The color of the border.
    #[conrod(default = "theme.border_color")]
    pub border_color: Option<Color>,
    /// Dragable
    #[conrod(default="false")]
    pub draggable: Option<bool>,
}

/// The State of the WildCard widget that will be cached within the Ui.
pub struct ImageState {
    /// Track whether some sort of dragging is currently occurring.
    drag: Drag,
    ids: ImageIds,
    toggle_bool: bool,
}
/// Track whether some sort of dragging is currently occurring.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Drag {
    /// The drag is currently selecting a range of text.
    Selecting(usize, Point),
    None,
    Terminate,
}

widget_ids! {
    /// Identifiers for an image button.
    #[allow(missing_docs, missing_copy_implementations)]
    pub struct ImageIds {
        image,
        label,
        rectangle,
        spinner
    }
}



impl<S> WildCard<S> {
    /// Create a button context to be built upon.
    fn new_internal(show: S) -> Self {
        WildCard {
            common: widget::CommonBuilder::default(),
            show: show,
            style: Style::default(),
        }
    }
}
impl<'a, H> WildCard<Image<'a, H>>
    where H: Spriteable
{
    /// Begin building a button displaying the given `Image` on top.
    pub fn image(img: image::Id, wild: Option<&'a mut String>) -> Self {
        let image = Image {
            image_id: img,
            toggle_image_id: None,
            spinner_image_id: None,
            wild: wild,
        };
        Self::new_internal(image)
    }
    /// The image displayed when the button is held for 4 seconds.
    pub fn toggle_image(mut self, id: image::Id) -> Self {
        self.show.toggle_image_id = Some(id);
        self
    }
    /// The spinner image overlay displayed when the button is held for 2 seconds.
    pub fn spinner_image(mut self, id: image::Id, sprite: H) -> Self {
        self.show.spinner_image_id = Some((id, sprite));
        self
    }
}
impl<'a, H> Widget for WildCard<Image<'a, H>>
    where H: Spriteable
{
    type State = ImageState;
    type Style = Style;
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        ImageState {
            drag: Drag::None,
            toggle_bool: false,
            ids: ImageIds::new(id_gen),
        }
    }

    fn style(&self) -> Style {
        self.style.clone()
    }

    /// Update the state of the WildCard.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, style, rect, ui, .. } = args;
        let WildCard { show, .. } = self;
        let mut drag = state.drag;
        let mut toggle_bool = state.toggle_bool;
        update_drag(id, &mut drag, ui);
        bordered_rectangle(id,
                           state.ids.rectangle,
                           rect,
                           style.color(&ui.theme),
                           style,
                           ui);
        let draw_spinner_index = update_toggle_bool_spinner_index(&mut drag, &mut toggle_bool);
        state.update(|state| {
                         state.drag = drag;
                         state.toggle_bool = toggle_bool
                     });
        // Instantiate the image.
        let widget_image = if toggle_bool {
            show.toggle_image_id.unwrap()
        } else {
            show.image_id
        };
        let (x, y, w, h) = rect.x_y_w_h();
        let image = widget::Image::new(widget_image)
            .x_y(x, y)
            .top_right_with_margins_on(id, h * 0.2, w * 0.2)
            .w_h(w * 0.6, h * 0.6)
            .parent(id)
            .graphics_for(id);

        image.set(state.ids.image, ui);
        if let Some(spinner_index) = draw_spinner_index {
            draw_spinner_op(id,
                            state.ids.spinner,
                            show.spinner_image_id,
                            spinner_index,
                            ui);
        }
        ()
    }
    fn drag_area(&self, dim: Dimensions, style: &Style, _theme: &Theme) -> Option<Rect> {
        if let Some(_) = style.draggable {
            Some(Rect::from_xy_dim([0.0, 0.0], dim))
        } else {
            None
        }
    }
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

fn bordered_rectangle(button_id: widget::Id,
                      rectangle_id: widget::Id,
                      rect: Rect,
                      color: Color,
                      style: &Style,
                      ui: &mut UiCell) {
    // BorderedRectangle widget.
    let dim = rect.dim();
    let border = style.border(&ui.theme);
    let border_color = style.border_color(&ui.theme);
    widget::BorderedRectangle::new(dim)
        .middle_of(button_id)
        .graphics_for(button_id)
        .color(color)
        .border(border)
        .border_color(border_color)
        .set(rectangle_id, ui);
}

fn draw_spinner_op<H: Spriteable>(button_id: widget::Id,
                                  spinner_id: widget::Id,
                                  spinner_image: Option<(image::Id, H)>,
                                  spinner_index: usize,
                                  ui: &mut UiCell) {
    if let Some((spinner_image, sprite)) = spinner_image {
        let r = spriteable_rect(sprite, spinner_index as f64);
        widget::Image::new(spinner_image)
            .source_rectangle(Rect::from_corners(r.0, r.1))
            .w_h(40.0, 40.0)
            .middle_of(button_id)
            .set(spinner_id, ui);
    }

}
fn update_toggle_bool_spinner_index(drag: &mut Drag, toggle_bool: &mut bool) -> Option<usize> {
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
impl<S> Colorable for WildCard<S> {
    builder_method!(color { style.color = Some(Color) });
}

impl<S> Borderable for WildCard<S> {
    builder_methods!{
        border { style.border = Some(Scalar) }
        border_color { style.border_color = Some(Color) }
    }
}
impl<S> Draggable for WildCard<S> {
    builder_methods!{
        draggable { style.draggable = Some(bool) }
    }
}
