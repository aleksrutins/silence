use druid::{PlatformError, Widget, widget::{Label, Flex, Padding, Align}, AppLauncher, WindowDesc};
use global_state::GlobalState;

mod header;
mod timeline;
mod global_state;
mod timeline_display;

fn build_ui() -> impl Widget<GlobalState> {
    Flex::column()
        .with_child(
            Align::centered(
                Label::new("Hello World!")
            )
        )
}

fn main() -> Result<(), PlatformError> {
    let state = GlobalState::default();
    AppLauncher::with_window(WindowDesc::new(build_ui())).launch(state)?;
    Ok(())
}