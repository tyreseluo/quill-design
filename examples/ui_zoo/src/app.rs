use quill_widgets::makepad_widgets::*;

live_design!{
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use quill_widgets::button::*;

    App = {{App}} {
        ui: <Root>{
            main_window = <Window>{
                body = <View>{
                    flow: Right,
                    spacing: 10,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    <View> {
                        height: 100,
                        width: 100,
                        show_bg: true,
                        draw_bg: {
                            color: #f0f0f0,
                        }
                    }
                    <QuillButton> {
                        type_of: Secondary,
                        text: "Secondary Button",
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        quill_widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}