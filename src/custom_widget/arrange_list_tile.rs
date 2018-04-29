use cardgame_widgets::custom_widget::arrange_list::{WidgetMut,Hoverable, Arrangeable, TimesClicked};
use conrod::{widget, Color, Colorable, Borderable, Positionable, UiCell, Widget, event, input,
             image, Theme, Sizeable, text, FontSize, color};
use conrod::position::{Rect, Scalar, Dimensions, Point};
use conrod::widget::list::{Right,Fixed};
use cardgame_widgets::sprite::{Spriteable, spriteable_rect};
use cardgame_widgets::text::get_font_size_hn;
use conrod::widget::Rectangle;
use backend::meta::app::{AppData,ResourceEnum};
use backend::SupportIdType;
use std::collections::HashMap;
use support;
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ItemWidget<'a,S>
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
    pub tuple:ArrangeTuple,
    pub timelesstext: String,
    pub cloudy_image: Option<image::Id>,
    pub coin_info: Option<image::Id>,
    pub coin_info270: Option<image::Id>,
    pub game_icon: Option<image::Id>,
    pub used_for_keypad:Option<(&'a AppData,&'a HashMap<ResourceEnum, SupportIdType>,bool,widget::Id)>,
    pub toggle:bool
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
        top_lefticon
    }
}

/// Represents the unique, cached state for our CardViewPartial widget.
pub struct State {
    ids: Ids,
    drag: Drag,
    blink_line_frame: u16,
}

impl<'a,S> ItemWidget<'a,S>
    where S: Spriteable
{
    /// Create a button context to be built upon.
    pub fn new(toggle_image: image::Id,
               tuple:ArrangeTuple,
               timelesstext: String,
               used_for_keypad:Option<(&'a AppData,&'a HashMap<ResourceEnum, SupportIdType>, bool,widget::Id)>)
               -> Self {
        ItemWidget {
            toggle_image: toggle_image,
            spinner_image_id: None,
            common: widget::CommonBuilder::default(),
            bordered: false,
            style: Style::default(),
            timelesstext: timelesstext,
            tuple:tuple,
            cloudy_image: None,
            game_icon: None,
            coin_info: None,
            coin_info270: None,
            used_for_keypad:used_for_keypad,
            toggle:false
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
    pub fn game_icon(mut self, game_icon: image::Id) -> Self {
        self.game_icon = Some(game_icon);
        self
    }

    pub fn toggle(mut self,toggle_bool: bool)->Self{
        self.toggle = toggle_bool;
        self
    }
}
impl<'a,S> WidgetMut<ArrangeTuple> for ItemWidget<'a,S> where S:Spriteable{
    fn set_mut<'c,'d>(self,widget_list_item:widget::list::Item<Right,Fixed>,ui:&'c mut UiCell<'d>)->(ArrangeTuple,bool){
        widget_list_item.set(self,ui)
    }
}
/// A custom Conrod widget must implement the Widget trait. See the **Widget** trait
/// documentation for more details.
impl<'a,S> Widget for ItemWidget<'a,S>
    where S: Spriteable
{
    /// The State struct that we defined above.
    type State = State;
    /// The Style struct that we defined using the `widget_style!` macro.
    type Style = Style;
    /// The event produced by instantiating the widget.
    ///
    /// bool is keypad
    type Event = (ArrangeTuple,bool);

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State {
        State {
            ids: Ids::new(id_gen),
            drag: Drag::None,
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
        
        let (q_cardindex,q_timeless,q_alphabet,mut q_op_str,q_color,q_text_id,q_cost_rect,q_top_left_rect,q_ink) = self.tuple.clone();
        let (_interaction, _times_triggered) = interaction_and_times_triggered(id, ui);
        let (_, _, w, h) = rect.x_y_w_h();
        let border = if self.bordered {
            self.style.border(ui.theme())
        } else {
            0.0
        };
        
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
        if let  Some(_) = q_op_str {
            widget::Image::new(self.toggle_image)
                .w_h(w - border, h - border)
                .middle_of(id)
                .parent(id)
                .graphics_for(id)
                .set(state.ids.cloudy, ui);
        } else {
            if let (Some(_cloudy), Some(_coin_info), Some(_coin_info270), Some(_game_icon)) =
                (self.cloudy_image, self.coin_info, self.coin_info270, self.game_icon) {
                widget::Image::new(_cloudy)
                    .w_h(w - border, h - border)
                    .middle_of(id)
                    .parent(id)
                    .graphics_for(id)
                    .set(state.ids.cloudy, ui);
                if q_timeless {
                    widget::Image::new(_coin_info270)
                        .source_rectangle(q_cost_rect)
                        .wh([w, h * 0.2])
                        .mid_bottom_of(id)
                        .parent(id)
                        .set(state.ids.coin_info, ui);
                    widget::Image::new(_game_icon)
                        .source_rectangle(q_top_left_rect)
                        .wh([h * 0.2, h * 0.2])
                        .mid_left_of(state.ids.coin_info)
                        .set(state.ids.top_lefticon, ui);
                    let fontsize = get_font_size_hn(h * 0.18, 1.0);
                    let timeless_font_id =q_text_id;
                    widget::Text::new(&self.timelesstext)
                        .mid_left_with_margin_on(state.ids.coin_info, 20.0)
                        .font_size(fontsize)
                        .color(color::WHITE)
                        .and_then(Some(timeless_font_id), widget::Text::font_id)
                        .set(state.ids.coin_info_timeless, ui);
                } else {
                    widget::Image::new(_coin_info)
                        .source_rectangle(q_cost_rect)
                        .wh([w * 0.2, h])
                        .mid_left_of(id)
                        .parent(id)
                        .set(state.ids.coin_info, ui);
                    widget::Image::new(_game_icon)
                        .source_rectangle(q_top_left_rect)
                        .wh([w * 0.2, w * 0.2])
                        .mid_top_of(state.ids.coin_info)
                        .set(state.ids.top_lefticon, ui);
                }
            }
        }

        let fontsize = get_font_size_hn(h, 2.0);
        let fontsize1 = if (q_alphabet== "m") | (q_alphabet== "w") {
            (fontsize as f64 * 0.6) as u32
        } else {
            fontsize
        };
        let j = q_alphabet.to_uppercase();

        widget::Text::new(&j)
            .mid_right_with_margin_on(id, 0.3 * w)
            .parent(id)
            .font_size(fontsize1)
            .and_then(Some(q_text_id), widget::Text::font_id)
            .graphics_for(id)
            .set(state.ids.alphabet, ui);
        let mut keypad_new=false;
        
        if let  (&mut Some(ref mut _str),Some((appdata,result_map,mut gamedata_keypad_on,id_master))) = (&mut q_op_str,self.used_for_keypad){
            let rect = Rect::from_xy_dim([0.0, 0.0], [80.0, 40.0]);
            rectangle_fill(id,
                           state.ids.textedit_background,
                           rect,
                           self.style.color(&ui.theme),
                           ui);
/*
            support::textedit(&mut _str,
                                state.ids.textedit_at_toggle,
                                appdata,
                                result_map,
                                [30.0, 50.0],
                                Some(1),
                                &mut gamedata_keypad_on,
                                state.ids.textedit_background,
                                15.0,
                                id_master,
                                ui);
*/            
            keypad_new = gamedata_keypad_on;
            if _str.chars().count() != 1 {
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
        if self.toggle{
            update_drag(id, &mut drag, ui);
       // let draw_spinner_index = update_toggle_bool_spinner_index(&mut drag,&mut q_op_str,j);
       let draw_spinner_index = Some(16u16);
 state.update(|state| {
                         state.drag = drag;
                     });

        if let Some(spinner_index) = draw_spinner_index {
            draw_spinner_op(id,
                            state.ids.spinner,
                            self.spinner_image_id,
                            spinner_index,
                            ui);
        }
        }
         ((q_cardindex,q_timeless,q_alphabet,q_op_str,q_color,q_text_id,q_cost_rect,q_top_left_rect,q_ink),keypad_new)
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
fn update_toggle_bool_spinner_index(drag: &mut Drag, op_str:&mut Option<String>,alphabet:String) -> Option<u16> {
    match drag {
        &mut Drag::Selecting(ref mut spinner_index, _) => {
            if *spinner_index >= 60 {
                if op_str.is_some() {
                    *op_str = None;
                } else {
                    *op_str = Some(alphabet.clone());
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
impl<'a,S> Arrangeable for ItemWidget<'a,S>
    where S: Spriteable
{
    fn selectable(mut self) -> Self {
        self.bordered = true;
        self
    }
}
impl<'a,S> Colorable for ItemWidget<'a,S>
    where S: Spriteable
{
    builder_method!(color { style.color = Some(Color) });
}
impl<'a,S> Borderable for ItemWidget<'a,S>
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
pub type ArrangeTuple = (usize, bool, String,Option<String>, Color, text::font::Id, Rect, Rect,bool);
//(*x.clone(), _timeless, _string,None, _color, _app_font, _rect, _top_left_rect,inked
