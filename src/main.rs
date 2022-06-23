use druid::{
    widget::{Align, Flex, Label, LensWrap, Padding},
    AppLauncher, Env, PlatformError, Widget, WidgetExt, WindowDesc,
};
use global_state::GlobalState;
use transport_display::transport_hud;

mod global_state;
mod header;
mod transport;
mod transport_display;

fn build_ui() -> impl Widget<GlobalState> {
    Flex::column()
        .with_child(Align::centered(transport_hud().lens(GlobalState::transport)))
        .padding(10.)
}

fn main() -> Result<(), PlatformError> {
    let state = GlobalState::default();

    AppLauncher::with_window(
        WindowDesc::new(build_ui())
            .title(|data: &GlobalState, _env: &Env| {
                format!(
                    "{} | Silence",
                    data.project
                        .as_ref()
                        .map_or("No Project".to_string(), |p| p.name.to_owned())
                )
            })
            .window_size((1000.0, 500.0)),
    )
    .launch(state)?;
    Ok(())
}
