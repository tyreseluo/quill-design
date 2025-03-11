pub mod api;

use api::XButtonType;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::styles::colors::*;

    ButtonBase = {{XButton}} {}

    pub XButton = <ButtonBase> {
        width: 100,
        height: 50,
        draw_bg: {
            fn pixel(self) -> vec4 {
                return (SKY_500)
            }
        }
        button_type: A,
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct XButton {
    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[live]
    draw_text: DrawText,
    #[live]
    draw_icon: DrawIcon,
    #[live]
    icon_walk: Walk,
    // #[live(true)] disabled: bool,
    // #[live] shape: ButtonShape,
    // #[live] loading: bool,
    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,
    #[live]
    button_type: XButtonType,
}

impl Widget for XButton {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        println!("button_type: {:?}", self.button_type);
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}