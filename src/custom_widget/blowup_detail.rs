use cardgame_widgets::custom_widget::image_hover::TimesClicked;
use conrod::{widget, Color, Colorable, Borderable, Positionable, UiCell, Widget, event, input,
             image, Theme, Sizeable, text, FontSize, color};
use conrod::position::{Rect, Scalar, Dimensions, Point};
use cardgame_widgets::text::get_font_size_hn;
use conrod::widget::Rectangle;
use backend::codec_lib::cards::GIVEABLE;
use graphics_match::gameicons_rect;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ItemWidget<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub key: &'a str,
    pub giveable: GIVEABLE,
    pub details: Option<String>,
    pub icon_image: image::Id,
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
}

widget_ids! {
    struct Ids {
        background,
        key,
        giveable_text,
        giveable_icon,
        giveable_text2,
        giveable_icon2,
        giveable_icon3,
        details,
    }
}

/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
}

impl<'a> ItemWidget<'a> {
    /// Create a button context to be built upon.
    pub fn new(key: &'a str,
               giveable: GIVEABLE,
               details: Option<String>,
               icon_image: image::Id)
               -> Self {
        ItemWidget {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
            key: key,
            giveable: giveable,
            details: details,
            icon_image: icon_image,
        }
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
        /* rectangle_fill(id,
                       state.ids.background,
                       rect,
                       self.style.color(&ui.theme),
                       ui);
                       */
        let fontsize = get_font_size_hn(h * 0.3, 1.0);
        let text_dim = [w * 0.2, h * 0.3];
        let icon_dim = [w * 0.2, h * 0.3];
        widget::Text::new(self.key)
            .color(color::BLACK)
            .font_size(fontsize)
            .graphics_for(id)
            .w(190.0)
            .mid_left_of(id)
            .set(state.ids.key, ui);
        match self.giveable {
            GIVEABLE::NONE => {}
            GIVEABLE::VP(_j) => {
                widget::Text::new(&_j.to_string())
                    .w(50.0)
                    .right_from(state.ids.keys, 5.0)
                    .set(state.ids.giveable_text, ui);
                let j_rect = gameicons_rect(5.0);
                widget::Image::new(self.icon_image)
                    .wh(icon_dim)
                    .right_from(state.ids.giveable_text, 0.0)
                    .source_rectangle(j_rect)
                    .set(state.ids.giveable_icon, ui);
            }
            GIVEABLE::COIN(_j) => {
                widget::Text::new(&_j.to_string())
                    .w(50.0)
                    .right_from(state.ids.keys, 5.0)
                    .set(state.ids.giveable_text, ui);
                let j_rect = gameicons_rect(3.0);
                widget::Image::new(self.icon_image)
                    .right_from(state.ids.giveable_text, 0.0)
                    .wh(icon_dim)
                    .source_rectangle(j_rect)
                    .set(state.ids.giveable_icon, ui);
            }
            GIVEABLE::VPCOIN(_v, _c) => {
                let mut v_str = _v.to_string();
                v_str.push_str(" + ");
                widget::Text::new(&v_str)
                    .w(50.0)
                    .right_from(state.ids.keys, 5.0)
                    .set(state.ids.giveable_text, ui);
                let v_rect = gameicons_rect(5.0);
                widget::Image::new(self.icon_image)
                    .right_from(state.ids.giveable_text, 0.0)
                    .source_rectangle(v_rect)
                    .wh(icon_dim)
                    .set(state.ids.giveable_icon, ui);
                widget::Text::new(&_c.to_string())
                    .right_from(state.ids.giveable_icon, 0.0)
                    .w(50.0)
                    .set(state.ids.giveable_text2, ui);
                let c_rect = gameicons_rect(3.0);
                widget::Image::new(self.icon_image)
                    .source_rectangle(c_rect)
                    .wh(icon_dim)
                    .right_from(state.ids.giveable_text2, 0.0)
                    .set(state.ids.giveable_icon2, ui);
            }
            GIVEABLE::COININK(_v) => {
                let mut v_str = _v.to_string();
                v_str.push_str(" + ");
                widget::Text::new(&v_str)
                    .wh(text_dim)
                    .right_from(state.ids.keys, 5.0)
                    .set(state.ids.giveable_text, ui);
                let v_rect = gameicons_rect(3.0);
                widget::Image::new(self.icon_image)
                    .right_from(state.ids.giveable_text, 0.0)
                    .wh(icon_dim)
                    .source_rectangle(v_rect)
                    .set(state.ids.giveable_icon, ui);
                widget::Text::new(&_v.to_string())
                    .right_from(state.ids.giveable_icon, 0.0)
                    .w(50.0)
                    .right_from(state.ids.keys, 5.0)
                    .set(state.ids.giveable_text2, ui);
                let c_rect = gameicons_rect(1.0);
                widget::Image::new(self.icon_image)
                    .source_rectangle(c_rect)
                    .wh(icon_dim)
                    .right_from(state.ids.giveable_text2, 0.0)
                    .set(state.ids.giveable_icon2, ui);
            }
            GIVEABLE::VPINK(_v) => {
                let mut v_str = _v.to_string();
                v_str.push_str(" + ");
                widget::Text::new(&v_str)
                    .wh(text_dim)
                    .right_from(state.ids.keys, 5.0)
                    .set(state.ids.giveable_text, ui);
                let v_rect = gameicons_rect(5.0);
                widget::Image::new(self.icon_image)
                    .right_from(state.ids.giveable_text, 0.0)
                    .wh(icon_dim)
                    .source_rectangle(v_rect)
                    .set(state.ids.giveable_icon, ui);
                widget::Text::new(&_v.to_string())
                    .right_from(state.ids.giveable_icon, 0.0)
                    .w(50.0)
                    .set(state.ids.giveable_text2, ui);
                let c_rect = gameicons_rect(1.0);
                widget::Image::new(self.icon_image)
                    .source_rectangle(c_rect)
                    .wh(icon_dim)
                    .right_from(state.ids.giveable_text2, 0.0)
                    .set(state.ids.giveable_icon2, ui);
            }
            GIVEABLE::VPORCOIN(_v) => {
                let mut v_str = _v.to_string();
                v_str.push_str(" / ");
                widget::Text::new(&v_str).wh(text_dim).mid_left_of(id).set(state.ids.giveable_text,
                                                                           ui);
                let v_rect = gameicons_rect(5.0);
                widget::Image::new(self.icon_image)
                    .right_from(state.ids.giveable_text, 0.0)
                    .wh(icon_dim)
                    .source_rectangle(v_rect)
                    .set(state.ids.giveable_icon, ui);
                widget::Text::new(&_v.to_string())
                    .right_from(state.ids.giveable_icon, 0.0)
                    .w(50.0)
                    .set(state.ids.giveable_text2, ui);
                let c_rect = gameicons_rect(3.0);
                widget::Image::new(self.icon_image)
                    .source_rectangle(c_rect)
                    .wh(icon_dim)
                    .right_from(state.ids.giveable_text2, 0.0)
                    .set(state.ids.giveable_icon2, ui);
            }
            GIVEABLE::VPORCOININK(_v) => {
                let mut v_str = _v.to_string();
                v_str.push_str(" / ");
                widget::Text::new(&v_str).wh(text_dim).mid_left_of(id).set(state.ids.giveable_text,
                                                                           ui);
                let v_rect = gameicons_rect(5.0);
                widget::Image::new(self.icon_image)
                    .right_from(state.ids.giveable_text, 0.0)
                    .wh(icon_dim)
                    .source_rectangle(v_rect)
                    .set(state.ids.giveable_icon, ui);
                widget::Text::new(&_v.to_string())
                    .right_from(state.ids.giveable_icon, 0.0)
                    .w(50.0)
                    .set(state.ids.giveable_text2, ui);
                let c_rect = gameicons_rect(3.0);
                widget::Image::new(self.icon_image)
                    .source_rectangle(c_rect)
                    .wh(icon_dim)
                    .right_from(state.ids.giveable_text2, 0.0)
                    .set(state.ids.giveable_icon2, ui);
                let rect3 = gameicons_rect(1.0);
                widget::Image::new(self.icon_image)
                    .source_rectangle(rect3)
                    .wh(text_dim)
                    .right_from(state.ids.giveable_icon2, 0.0)
                    .set(state.ids.giveable_icon3, ui);
            }
            GIVEABLE::INK => {
                widget::Text::new("1").wh(text_dim).mid_left_of(id).set(state.ids.giveable_text,
                                                                        ui);
                let v_rect = gameicons_rect(1.0);
                widget::Image::new(self.icon_image)
                    .right_from(state.ids.giveable_text, 0.0)
                    .wh(icon_dim)
                    .source_rectangle(v_rect)
                    .set(state.ids.giveable_icon, ui);
            }
        }
        if let Some(_detail) = self.details {
            widget::Text::new(&_detail)
                .down_from(state.ids.giveable_text, 0.0)
                .h(h * 0.6)
                .w(w * 0.4)
                .set(state.ids.details, ui);
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

    widget::BorderedRectangle::new(dim)
        //.with_style(_style)
        .border_color(color::BLACK)
            .border(2.0)
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
