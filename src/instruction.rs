use cardgame_widgets::custom_widget::instructionset::Instructable;
use conrod::{Sizeable, Positionable, widget, color};
use conrod::widget::primitive::shape::oval::Full;
use conrod::widget::button::{Button, Flat};
use conrod::widget::{Oval, Rectangle};
pub struct Instruction<'a>(pub &'a str, pub &'a [f64; 4], pub &'a Option<[f64; 4]>, pub widget::Id);
impl<'a> Instructable<'a> for Instruction<'a> {
    fn label(&self) -> &'a str {
        self.0
    }
    fn rect(&self, wh: [f64; 2]) -> Rectangle {
        widget::Rectangle::fill_with([self.1[2].clone() * wh[0], self.1[3].clone() * wh[1]],
                                     color::BLACK.with_alpha(0.3))
                .top_left_with_margins_on(self.3, self.1[1] * wh[1], self.1[0] * wh[0])
    }
    fn button(&self, _wh: [f64; 2]) -> Button<Flat> {
        widget::Button::new().w_h(100.0, 50.0).mid_bottom()
    }
    fn oval_one(&self, wh: [f64; 2]) -> Option<Oval<Full>> {
        if let Some(_dim) = self.2.clone() {
            Some(widget::Oval::outline_styled([_dim[2] * wh[0], _dim[3] * wh[1]],
                                              widget::line::Style::new().thickness(5.0))
                         .top_left_with_margins_on(self.3,
                                                   _dim[1] * wh[1],
                                                   _dim[0] * wh[0]))
        } else {
            None
        }

    }
    fn oval_two(&self, wh: [f64; 2]) -> Option<Oval<Full>> {
        if let Some(_dim) = self.2.clone() {
            Some(widget::Oval::outline_styled([_dim[2] * wh[0] * 1.2, _dim[3] * wh[1]],
                                              widget::line::Style::new().thickness(5.0))
                         .top_left_with_margins_on(self.3,
                                                   _dim[1] * wh[1],
                                                   _dim[0] * wh[0]))
        } else {
            None
        }
    }
}
