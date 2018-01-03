use cardgame_widgets::custom_widget::arrange_list::{Hoverable, Arrangeable, TimesClicked};
use conrod::{widget, Color, Colorable, Borderable, Positionable, UiCell, Widget, event, input,
             image, Theme, Sizeable, text, FontSize, color};

use conrod::position::{Rect, Scalar, Dimensions, Point};
use cardgame_widgets::sprite::{Spriteable, spriteable_rect};
use cardgame_widgets::text::get_font_size_hn;
use conrod::widget::Rectangle;

/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ItemWidget<'a, S>
    where S: Spriteable
{
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub bordered: bool,
    /// See the Style struct below.
    style: Style,
    pub toggle_image: image::Id,
    pub spinner_image_id: Option<(image::Id, S)>,
    pub timeless: bool,
    pub cost_rect: Rect,
    pub alphabet: &'a str,
    pub timelesstext: &'a str,
    pub cloudy_image: Option<image::Id>,
    pub coin_info: Option<image::Id>,
    pub coin_info270: Option<image::Id>,
}

#[derive( Clone, Debug, Default, PartialEq, WidgetStyle)]
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
    /// Alphabet Font Id
    #[conrod(default="theme.font_id")]
    pub alphabet_font_id: Option<Option<text::font::Id>>,
    /// Timeless Font Id
    #[conrod(default="theme.font_id")]
    pub timeless_font_id: Option<Option<text::font::Id>>,
}

widget_ids! {
    struct Ids {
        background,
        rect,
        cloudy,
        alphabet,
        coin_info,
        coin_info_timeless,
        spinner,
        textedit_background,
        textedit_blinkline,
        textedit_at_toggle,
        
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

impl<'a, S> ItemWidget<'a, S>
    where S: Spriteable
{
    /// Create a button context to be built upon.
    pub fn new(toggle_image: image::Id,
               timeless: bool,
               alphabet: &'a str,
               cost_rect: Rect,
               timelesstext: &'a str)
               -> Self {
        ItemWidget {
            toggle_image: toggle_image,
            spinner_image_id: None,
            common: widget::CommonBuilder::default(),
            bordered: false,
            style: Style::default(),
            alphabet: alphabet,
            timeless: timeless,
            timelesstext: timelesstext,
            cost_rect: cost_rect,
            cloudy_image: None,
            coin_info: None,
            coin_info270: None,
        }
    }

    /// The spinner image overlay displayed when the button is held for 2 seconds.
    pub fn spinner_image(mut self, id: image::Id, sprite: S) -> Self {
        self.spinner_image_id = Some((id, sprite));
        self
    }

    pub fn cloudy_image(mut self, image: image::Id) -> Self {
        self.cloudy_image = Some(image);
        self
    }
    pub fn coin_info(mut self, coin_info: image::Id) -> Self {
        self.coin_info = Some(coin_info);
        self
    }
    pub fn coin_info270(mut self, coin_info270: image::Id) -> Self {
        self.coin_info270 = Some(coin_info270);
        self
    }

    pub fn alphabet_font_id(mut self, font_id: text::font::Id) -> Self {
        self.style.alphabet_font_id = Some(Some(font_id));
        self
    }
    pub fn timeless_font_id(mut self, font_id: text::font::Id) -> Self {
        self.style.timeless_font_id = Some(Some(font_id));
        self
    }
}

/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a, S> Widget for ItemWidget<'a, S>
    where S: Spriteable
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

        let (_interaction, _times_triggered) = interaction_and_times_triggered(id, ui);
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
            Rectangle::outline_styled([w, h], _style).middle_of(id).parent(id).set(state.ids.rect,
                                                                                   ui);
        }
        // Instantiate the image.
        if toggle_bool {
            widget::Image::new(self.toggle_image)
                .w_h(w - border, h - border)
                .middle_of(id)
                .parent(id)
                .graphics_for(id)
                .set(state.ids.cloudy, ui);
        } else {
            if let (Some(_cloudy), Some(_coin_info), Some(_coin_info270)) =
                (self.cloudy_image, self.coin_info, self.coin_info270) {
                widget::Image::new(_cloudy)
                    .w_h(w - border, h - border)
                    .middle_of(id)
                    .parent(id)
                    .graphics_for(id)
                    .set(state.ids.cloudy, ui);
                if self.timeless {
                    widget::Image::new(_coin_info270)
                        .source_rectangle(self.cost_rect)
                        .wh([w, h * 0.2])
                        .mid_bottom_of(id)
                        .parent(id)
                        .set(state.ids.coin_info, ui);
                    let fontsize = get_font_size_hn(h * 0.2, 1.0);
                    let timeless_font_id =
                        self.style.timeless_font_id(&ui.theme).or(ui.fonts.ids().next());
                    widget::Text::new(self.timelesstext)
                        .middle_of(state.ids.coin_info)
                        .font_size(fontsize)
                        .color(color::WHITE)
                        .and_then(timeless_font_id, widget::Text::font_id)
                        .set(state.ids.coin_info_timeless, ui);
                } else {
                    widget::Image::new(_coin_info)
                        .source_rectangle(self.cost_rect)
                        .wh([w * 0.2, h])
                        .mid_left_of(id)
                        .parent(id)
                        .set(state.ids.coin_info, ui);
                }
            }
        }

        let fontsize = get_font_size_hn(h, 2.0);
        let alphabet_font_id = self.style.alphabet_font_id(&ui.theme).or(ui.fonts.ids().next());
        let fontsize1 = if (self.alphabet == "m") | (self.alphabet == "w") {
            (fontsize as f64 * 0.7) as u32
        } else {
            fontsize
        };
        let j = self.alphabet.to_uppercase();

        widget::Text::new(&j)
            .mid_right_with_margin_on(id, 0.2 * w)
            .parent(id)
            .font_size(fontsize1)
            .and_then(alphabet_font_id, widget::Text::font_id)
            .graphics_for(id)
            .set(state.ids.alphabet, ui);

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
impl<'a, S> Arrangeable for ItemWidget<'a, S>
    where S: Spriteable
{
    fn selectable(mut self) -> Self {
        self.bordered = true;
        self
    }
}
impl<'a, S> Colorable for ItemWidget<'a, S>
    where S: Spriteable
{
    builder_method!(color { style.color = Some(Color) });
}
impl<'a, S> Borderable for ItemWidget<'a, S>
    where S: Spriteable
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
