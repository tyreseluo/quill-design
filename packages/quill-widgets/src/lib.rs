/// Externally use the makepad version that is fully compatible with Quill Design.
pub extern crate makepad_widgets;

#[cfg(feature = "Button")]
#[path ="./button/button.rs"]
pub mod button;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    makepad_widgets::live_design(cx);
    #[cfg(feature = "Button")]
    button::live_design(cx);
}