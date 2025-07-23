// Button API
// disabled: Whether the button is disabled
// size: The size of the button
// type: The type of the button (primary, secondary, etc.)

use makepad_widgets::{
    live_error_origin, live_id, Apply, Cx, LiveApply, LiveErrorOrigin, LiveId, LiveModuleId,
    LiveNew, LiveNode, LiveNodeSliceApi, LiveType, LiveTypeInfo, LiveValue,
};

#[derive(Default, Debug)]
pub enum ButtonType {
    #[default]
    Primary,
    Secondary,
    Tertiary,
    Warning,
    Danger,
}

impl LiveNew for ButtonType  {
    fn new(_cx: &mut Cx) -> Self {
        Self::Primary
    }

    fn live_type_info(_cx: &mut Cx) -> makepad_widgets::LiveTypeInfo {
        LiveTypeInfo {
            live_type: LiveType::of::<ButtonType>(),
            type_name: LiveId(0),
            module_id: LiveModuleId::from_str(&module_path!()).unwrap(),
            live_ignore: true,
            fields: Vec::new(),
        }
    }
}

impl LiveApply for ButtonType {
    fn apply(
        &mut self,
        cx: &mut Cx,
        _apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        if let LiveValue::BareEnum(id) = nodes[index].value {
            *self = match id {
                live_id!(Primary) => ButtonType::Primary,
                live_id!(Secondary) => ButtonType::Secondary,
                live_id!(Tertiary) => ButtonType::Tertiary,
                live_id!(Warning) => ButtonType::Warning,
                live_id!(Danger) => ButtonType::Danger,
                _ => {
                    cx.apply_error(
                        live_error_origin!(),
                        index,
                        nodes,
                        "Invalid ButtonType value, only support [Default, Primary, Secondary, Tertiary, Warning, Danger]".to_string(),
                    );
                    return nodes.skip_node(index);
                }
            };
            return index + 1;
        }
        nodes.skip_node(index)
    }
}