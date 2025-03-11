use makepad_widgets::{
    live_error_origin, live_id, Apply, Cx, LiveApply, LiveErrorOrigin, LiveId, LiveModuleId,
    LiveNew, LiveNode, LiveNodeSliceApi, LiveType, LiveTypeInfo, LiveValue,
};

#[derive(Default, Debug)]
pub enum XButtonType {
    #[default]
    Default,
    Primary,
    Secondary,
    Dashed,
    Text,
    Outline,
}

impl LiveNew for XButtonType {
    fn new(_cx: &mut Cx) -> Self {
        Self::Default
    }

    fn live_type_info(_cx: &mut Cx) -> LiveTypeInfo {
        LiveTypeInfo {
            live_type: LiveType::of::<XButtonType>(),
            type_name: LiveId(0),
            module_id: LiveModuleId::from_str(&module_path!()).unwrap(),
            live_ignore: true,
            fields: Vec::new(),
        }
    }
}

impl LiveApply for XButtonType {
    fn apply(
        &mut self,
        cx: &mut Cx,
        _apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        if let LiveValue::BareEnum(id) = nodes[index].value {
            *self = match id {
                live_id!(Default) => XButtonType::Default,
                live_id!(Primary) => XButtonType::Primary,
                live_id!(Secondary) => XButtonType::Secondary,
                live_id!(Dashed) => XButtonType::Dashed,
                live_id!(Text) => XButtonType::Text,
                live_id!(Outline) => XButtonType::Outline,
                _ => {
                    cx.apply_error(
                        live_error_origin!(),
                        index,
                        nodes,
                        "invalid XButtonType value, only support [Default, Primary, Secondary, Dashed, Text, Outline]".to_string(),
                    );
                    return nodes.skip_node(index);
                }
            };
            return index + 1;
        }
        nodes.skip_node(index)
    }
}

#[derive(Default)]
pub enum XButtonShape {
    Circle,
    Round,
    #[default]
    Square,
}
