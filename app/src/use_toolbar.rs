use editor::Tool;
use yew::{hook, Callback, MouseEvent, UseStateHandle};

#[hook]
pub fn use_toolbar_callback(
    current_tool: UseStateHandle<Tool>,
    tool: Tool,
) -> Callback<MouseEvent> {
    Callback::from({
        let current_tool = current_tool.clone();
        move |_| current_tool.set(tool)
    })
}
