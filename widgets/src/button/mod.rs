pub mod api;

use api::{ButtonType, ButtonShape};
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::styles::colors::*;

    ButtonBase = {{XButton}} {}

    pub XButton = <ButtonBase> {
        width: Fit, height: 32,
        align: {x: 0.5, y: 0.5},
        label_walk: { width: Fit, height: Fit },
        padding: { left: 15.0, right: 15.0 },
        draw_text: {
            color: #fff
            text_style: <THEME_FONT_REGULAR> {
                font_size: 10.5,
            }
        }
        draw_bg: {
            instance radius: 2.0
            instance border_width: 1.0
            instance border_color: #0000
            instance bg_color: (ORANGE_500)
            instance inset: vec4(0.0, 0.0, 0.0, 0.0)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.inset.x + self.border_width,
                    self.inset.y + self.border_width,
                    self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
                    self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
                    max(1.0, self.radius)
                )
                return sdf.fill(self.bg_color);
            }
        },
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct XButton {
    #[redraw] #[live]   draw_bg: DrawQuad,
    #[layout]           layout: Layout,
    #[walk]             walk: Walk,

    #[live]             draw_text: DrawText,
    #[live]             icon_walk: Walk,
    #[live]             label_walk: Walk,

    #[live(true)]       disabled: bool,
    #[live]             shape: ButtonShape,
    #[live]             loading: bool,
    #[live]             type_of: ButtonType,

    #[live]         pub text: ArcStringMut,
}

impl Widget for XButton {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        self._type(cx);
        self._shape(cx);
        self.draw_text
            .draw_walk(cx, self.label_walk, Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        DrawStep::done()
    }

    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }

    fn set_text(&mut self, cx:&mut Cx, v: &str) {
        self.text.as_mut_empty().push_str(v);
        self.redraw(cx);
    }
}

impl XButton {
    fn _shape(&mut self, cx: &mut Cx) {
        match self.shape {
            ButtonShape::Square => self.__shape_square(cx),
            ButtonShape::Round => self.__shape_round(cx),
            ButtonShape::Circle => self.__shape_circle(cx),
        }
    }

    fn _type(&mut self, cx: &mut Cx) {
        match self.type_of {
            ButtonType::Primary => self.__type_primary(cx),
            ButtonType::Secondary => self.__type_secondary(cx),
            ButtonType::Text => self.__type_text(cx),
            ButtonType::Outline => self.__type_outline(cx),
        }
    }
}

/// ButtonType impls
impl XButton {
    fn __type_primary(&mut self, _cx: &mut Cx) {
        log!("primary");
    }

    fn __type_secondary(&mut self, cx: &mut Cx) {
        self.draw_text.apply_over(cx, live!{
            color: (vec4(0.0, 0.0, 0.0, 1.0)),
        });
        self.draw_bg.apply_over(cx, live! {
            bg_color: (vec4(1.0, 1.0, 1.0, 1.0)),
        });
    }

    fn __type_text(&mut self, _cx: &mut Cx) {
        log!("text");
    }

    fn __type_outline(&mut self, _cx: &mut Cx) {
        log!("outline");
    }
}


/// ButtonShape impls
impl XButton {
    fn __shape_square(&mut self, _cx: &mut Cx) {
        log!("square");
    }

    fn __shape_round(&mut self, cx: &mut Cx) {
        log!("round");
        let mut value = [0.0];
        self.draw_bg.get_instance(cx, id!(radius), &mut value);
        let radius = value[0];
        // if radius is 0.0, user is not setting own radius ,apply default radius
        if radius == 0.0 {
            self.draw_bg.apply_over(cx, live!(
                radius: 8.5,
            ));
        }
    }

    fn __shape_circle(&mut self, _cx: &mut Cx) {
        log!("circle");
    }
}
