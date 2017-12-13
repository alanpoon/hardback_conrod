use cardgame_widgets::custom_widget::arrange_list::{Hoverable, Arrangeable, TimesClicked};
use conrod::{widget, Color, Colorable, Borderable, Positionable, UiCell, Widget, event, input,
             image, Theme, Sizeable};

use conrod::position::{Rect, Scalar, Dimensions, Point};
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
        spinner,
        textedit_background,
        textedit_blinkline,
        textedit_at_toggle
    }
}

/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
    drag: Drag,
    toggle_bool: bool,
    op_str: String,
    blink_line_frame: u16,
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
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            drag: Drag::None,
            toggle_bool: false,
            op_str: "".to_owned(),
            blink_line_frame: 0,
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

        let (interaction, _times_triggered) = interaction_and_times_triggered(id, ui);
        let (_, _, w, h) = rect.x_y_w_h();
        let border = if self.bordered {
            self.style.border(ui.theme())
        } else {
            0.0
        };
        let mut toggle_bool = state.toggle_bool;
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
        let _image = match interaction {
            Interaction::Idle => widget_ih.idle(),
            Interaction::Hover => widget_ih.hover().unwrap_or(widget_ih.idle()),
            Interaction::Press => widget_ih.press().unwrap_or(widget_ih.idle()),
        };
        _image.w_h(w - border, h - border)
            .middle_of(id)
            .parent(id)
            .graphics_for(id)
            .set(state.ids.image, ui);
        if toggle_bool {
            let rect = Rect::from_xy_dim([0.0, 0.0], [80.0, 40.0]);
            rectangle_fill(id,
                           state.ids.textedit_background,
                           rect,
                           self.style.color(&ui.theme),
                           ui);

            for edit in widget::TextEdit::new(&state.op_str)
                    .color(self.style.color(&ui.theme).plain_contrast())
                    .middle_of(state.ids.textedit_background)
                    .padded_wh_of(state.ids.textedit_background, 5.0)
                    .set(state.ids.textedit_at_toggle, ui) {
                if state.op_str.chars().count() < 1 {
                    state.update(|state| state.op_str = edit);
                }
            }

            if state.op_str.chars().count() != 1 {
                state.update(|state| state.blink_line_frame += 1);
                if (state.blink_line_frame / 120) == 0 {
                    let line_l = ui.w_of(state.ids.textedit_background).unwrap();
                    let _style = widget::line::Style {
                        maybe_pattern: None,
                        maybe_color: Some(self.style.color(&ui.theme).plain_contrast()),
                        maybe_thickness: Some(border),
                        maybe_cap: None,
                    };
                    widget::Line::centred_styled([-line_l * 0.5, 0.0], [line_l * 0.5, 0.0], _style)
                        .mid_bottom_of(state.ids.textedit_background)
                        .set(state.ids.textedit_blinkline, ui);
                }
                if state.blink_line_frame > 240 {
                    state.update(|state| state.blink_line_frame = 0);
                }
            } else {
                state.update(|state| state.blink_line_frame = 0);
            }
        }

        let mut drag = state.drag;

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

    }
    fn drag_area(&self, dim: Dimensions, style: &Style, _theme: &Theme) -> Option<Rect> {
        if let Some(_) = style.draggable {
            Some(Rect::from_xy_dim([0.0, 0.0], dim))
        } else {
            None
        }
    }
}
fn interaction_and_times_triggered(button_id: widget::Id, ui: &UiCell) -> (Interaction, u16) {
    let input = ui.widget_input(button_id);
    let interaction = input.mouse().map_or(Interaction::Idle,
                                           |mouse| if mouse.buttons.left().is_down() {
                                               Interaction::Press
                                           } else {
                                               Interaction::Hover
                                           });
    let times_triggered = (input.clicks().left().count() + input.taps().count()) as u16;
    (interaction, times_triggered)
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
                            &mut Drag::None => {
                                *drag = Drag::Selecting(0, point);
                            }
                            _ => {}
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
        let but_w = ui.w_of(button_id).unwrap();
        widget::Image::new(spinner_image)
            .source_rectangle(Rect::from_corners(_rect.0, _rect.1))
            .w_h(but_w * 0.8, but_w * 0.8)
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
#[derive(Copy, Clone,Debug)]
enum Interaction {
    Idle,
    Hover,
    Press,
}
