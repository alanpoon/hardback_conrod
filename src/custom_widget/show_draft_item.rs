use cardgame_widgets::custom_widget::image_hover::TimesClicked;
use conrod_core::{widget, Color, Colorable, Borderable, Positionable, UiCell, Widget, event, input,
             image, Theme, Sizeable, text, FontSize, color};
use conrod_core::position::{Rect, Scalar, Dimensions, Point};
use cardgame_widgets::text::get_font_size_hn;
use conrod_core::widget::Rectangle;

/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ItemWidget<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub timeless: bool,
    pub cost_rect: Rect,
    pub top_left_rect: Rect,
    pub alphabet: &'a str,
    pub timelesstext: &'a str,
    pub cloudy_image: Option<image::Id>,
    pub coin_info: Option<image::Id>,
    pub coin_info270: Option<image::Id>,
    pub game_icon: Option<image::Id>,
    /// See the Style struct below.
    style: Style,
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
        textedit_background,
        textedit_blinkline,
        textedit_at_toggle,
        top_lefticon
    }
}

/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
}

impl<'a> ItemWidget<'a> {
    /// Create a button context to be built upon.
    pub fn new(timeless: bool,
               alphabet: &'a str,
               cost_rect: Rect,
               top_left_rect: Rect,
               timelesstext: &'a str)
               -> Self {
        ItemWidget {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            alphabet: alphabet,
            timeless: timeless,
            timelesstext: timelesstext,
            cost_rect: cost_rect,
            top_left_rect: top_left_rect,
            cloudy_image: None,
            coin_info: None,
            coin_info270: None,
            game_icon: None,
        }
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
    pub fn game_icon(mut self, game_icon: image::Id) -> Self {
        self.game_icon = Some(game_icon);
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
impl<'a> Widget for ItemWidget<'a> {
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// `Some` when clicked, otherwise `None`.
    type Event = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State { ids: Ids::new(id_gen) }
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
        rectangle_fill(id,
                       state.ids.background,
                       rect,
                       self.style.color(&ui.theme),
                       ui);

        if let (Some(_cloudy), Some(_coin_info), Some(_coin_info270)) =
            (self.cloudy_image, self.coin_info, self.coin_info270) {
            widget::Image::new(_cloudy)
                .w_h(w, h)
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
                if let Some(_game_icon) = self.game_icon {
                    widget::Image::new(_game_icon)
                        .source_rectangle(self.top_left_rect)
                        .wh([h * 0.2, h * 0.2])
                        .mid_left_of(state.ids.coin_info)
                        .set(state.ids.top_lefticon, ui);
                }
                let fontsize = get_font_size_hn(h * 0.2, 1.0);
                let timeless_font_id =
                    self.style.timeless_font_id(&ui.theme).or(ui.fonts.ids().next());
                widget::Text::new(self.timelesstext)
                    .mid_left_with_margin_on(state.ids.coin_info, 2.0)
                    .color(color::WHITE)
                    .font_size(fontsize)
                    .and_then(timeless_font_id, widget::Text::font_id)
                    .set(state.ids.coin_info_timeless, ui);
            } else {
                widget::Image::new(_coin_info)
                    .source_rectangle(self.cost_rect)
                    .wh([w * 0.2, h])
                    .mid_left_of(id)
                    .parent(id)
                    .set(state.ids.coin_info, ui);
                if let Some(_game_icon) = self.game_icon {
                    widget::Image::new(_game_icon)
                        .source_rectangle(self.top_left_rect)
                        .wh([w * 0.2, w * 0.2])
                        .mid_top_of(state.ids.coin_info)
                        .set(state.ids.top_lefticon, ui);
                }
            }
        }

        let fontsize = get_font_size_hn(h, 2.0);
        let alphabet_font_id = self.style.alphabet_font_id(&ui.theme).or(ui.fonts.ids().next());
        let j = self.alphabet.to_uppercase();
        let fontsize1 = if (self.alphabet == "m") | (self.alphabet == "w") {
            (fontsize as f64 * 0.6) as u32
        } else {
            fontsize
        };
        widget::Text::new(&j)
            .mid_right_with_margin_on(id, 0.3 * w)
            .parent(id)
            .font_size(fontsize1)
            .and_then(alphabet_font_id, widget::Text::font_id)
            .graphics_for(id)
            .set(state.ids.alphabet, ui);
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


impl<'a> Colorable for ItemWidget<'a> {
    builder_method!(color { style.color = Some(Color) });
}
impl<'a> Borderable for ItemWidget<'a> {
    builder_methods!{
        border { style.border = Some(Scalar) }
        border_color { style.border_color = Some(Color) }
    }
}
