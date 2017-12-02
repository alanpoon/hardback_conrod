use conrod::{widget, Color, Colorable, Positionable, UiCell, Widget, image, Theme, Sizeable, Rect,
             Labelable};
use cardgame_widgets::custom_widget::list_select::{ListItem, ListSelect};
use conrod::widget::Rectangle;
use std::iter::once;
//Player_info list all player's item, at the end, there is some arrow animation that opens another overlay
/// The type upon which we'll implement the `Widget` trait.
#[derive(WidgetCommon)]
pub struct ItemWidget<'a> {
    /// An object that handles some of the dirty work of rendering a GUI. We don't
    /// really have to worry about it.
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    pub icon_vec: Vec<ListItem>,
    /// See the Style struct below.
    style: Style,
    pub overlay: &'a mut bool,
}

#[derive(Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the button's label.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<Color>,
}

widget_ids! {
    struct Ids {
        rect,
        icon_vec,
        arrow1,
        arrow2,
        arrow3
    }
}

/// Represents the unique, cached state for our widget.
pub struct State {
    ids: Ids,
    frame: u16,
}

impl<'a> ItemWidget<'a> {
    /// Create a button context to be built upon.
    pub fn new(icon_vec: Vec<ListItem>, overlay: &'a mut bool) -> Self {
        ItemWidget {
            icon_vec: icon_vec,
            common: widget::CommonBuilder::default(),
            overlay: overlay,
            style: Style::default(),
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
        State {
            ids: Ids::new(id_gen),
            frame: 0,
        }
    }

    fn style(&self) -> Self::Style {
        self.style.clone()
    }

    /// Update the state of the button by handling any input that has occurred since the last
    /// update.
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs { id, state, rect, ui, .. } = args;
        let (interaction, _times_triggered) = interaction_and_times_triggered(id, ui);
        let _dim = rect.dim();
        let default_color = self.style.color(&ui.theme);
        let rect_c = match interaction {
            Interaction::Idle => default_color,
            Interaction::Hover => default_color.highlighted(),
            Interaction::Press => default_color.highlighted(),
        };
        rectangle_fill(id, state.ids.rect, rect, rect_c, ui);
        ListSelect::new(self.icon_vec)
            .label("Player info")
            .label_color(default_color.plain_contrast())
            .mid_left_of(id)
            .w(_dim[0] * 0.7)
            .set(state.ids.icon_vec, ui);
        let (left, top, right) = if self.overlay {
            ([0.0, -_dim[1] * 0.8], [-_dim[0] * 0.07, 0.0], [0.0, _dim[1] * 0.8])
        } else {
            ([_dim[0] * 0.07, -_dim[1] * 0.8], [0.0, 0.0], [_dim[0] * 0.07, _dim[1] * 0.8])
        };

        let points = once(left).chain(once(top)).chain(once(right));
        if state.frame / 60 == 0 {
            widget::PointPath::centred(points.clone())
                .right_from(state.ids.icon_vec, _dim[0] * 0.05)
                .set(state.ids.arrow1, ui);
        } else if state.frame / 60 == 1 {
            widget::PointPath::centred(points.clone())
                .right_from(state.ids.arrow1, _dim[0] * 0.05)
                .set(state.ids.arrow2, ui);
        } else if state.frame / 60 == 2 {
            widget::PointPath::centred(points)
                .right_from(state.ids.arrow2, _dim[0] * 0.05)
                .set(state.ids.arrow3, ui);
        }
        state.update(|state| state.frame += 1);
        if state.frame > 180 {
            state.update(|state| state.frame = 0);
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
impl<'a> Colorable for ItemWidget<'a> {
    fn color(mut self, color: Color) -> Self {
        self.style.color = Some(color);
        self
    }
}

#[derive(Copy, Clone,Debug)]
enum Interaction {
    Idle,
    Hover,
    Press,
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
