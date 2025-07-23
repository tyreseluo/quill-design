pub mod api;

use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    ButtonBase = {{QuillButton}} {}

    pub QuillButton = <ButtonBase> {
        height: 100,
        width: 100,
        show_bg: true,
        draw_bg: {
            color: #ff6900,
        }
    }
}


#[derive(Live, LiveHook, Widget)]
pub struct QuillButton {
    #[deref] view: View,
}

impl Widget for QuillButton {
   fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
       self.view.draw_walk(cx, scope, walk)
   }
}