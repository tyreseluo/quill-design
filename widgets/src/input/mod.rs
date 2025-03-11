use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub QInput = <View> {
        width: 300,
        height: 100,
        show_bg: true,
        draw_bg: {
            color: #0D3B77
        }
    }
}