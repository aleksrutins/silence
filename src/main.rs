use druid::{
    widget::{Align, Flex, Label, LensWrap, Padding},
    AppLauncher, Env, PlatformError, Widget, WidgetExt, WindowDesc,
};
use global_state::GlobalState;
use timeline_display::timeline_hud;

mod global_state;
mod header;
mod timeline;
mod timeline_display;

fn build_ui() -> impl Widget<GlobalState> {
    Flex::column()
        .with_child(Align::centered(timeline_hud().lens(GlobalState::timeline)))
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
