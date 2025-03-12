use makepad_widgets::{
    live_error_origin, live_id, Apply, Cx, LiveApply, LiveErrorOrigin, LiveId, LiveModuleId,
    LiveNew, LiveNode, LiveNodeSliceApi, LiveType, LiveTypeInfo, LiveValue,
};

pub enum XButtonAction {
    None,
}

#[derive(Default, Debug)]
pub enum ButtonType {
    #[default]
    Primary,
    Secondary,
    Text,
    Outline,
}

impl LiveNew for ButtonType {
    fn new(_cx: &mut Cx) -> Self {
        Self::Primary
    }

    fn live_type_info(_cx: &mut Cx) -> LiveTypeInfo {
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
                live_id!(Text) => ButtonType::Text,
                live_id!(Outline) => ButtonType::Outline,
                _ => {
                    cx.apply_error(
                        live_error_origin!(),
                        index,
                        nodes,
                        "Invalid ButtonType value, only support [Default, Primary, Secondary, Dashed, Text, Outline]".to_string(),
                    );
                    return nodes.skip_node(index);
                }
            };
            return index + 1;
        }
        nodes.skip_node(index)
    }
}

#[derive(Default, Debug)]
pub enum ButtonShape {
    #[default]
    Square,
    Circle,
    Round,
}

impl LiveNew for ButtonShape {
    fn new(_cx: &mut Cx) -> Self {
        Self::Square
    }

    fn live_type_info(_cx: &mut Cx) -> LiveTypeInfo {
        LiveTypeInfo {
            live_type: LiveType::of::<ButtonShape>(),
            type_name: LiveId(0),
            module_id: LiveModuleId::from_str(&module_path!()).unwrap(),
            live_ignore: true,
            fields: Vec::new(),
        }
    }
}

impl LiveApply for ButtonShape {
    fn apply(
        &mut self,
        cx: &mut Cx,
        _apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        if let LiveValue::BareEnum(id) = nodes[index].value {
            *self = match id {
                live_id!(Circle) => ButtonShape::Circle,
                live_id!(Round) => ButtonShape::Round,
                live_id!(Square) => ButtonShape::Square,
                _ => {
                    cx.apply_error(
                        live_error_origin!(),
                        index,
                        nodes,
                        "Invalid ButtonShape value, only support [Circle, Round, Square]".to_string(),
                    );
                    return nodes.skip_node(index);
                }
            };
            return index + 1;
        }
        nodes.skip_node(index)
    }
}