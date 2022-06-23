use druid::{
    kurbo::RoundedRect,
    widget::{Button, Flex, Label},
    Color, Env, RenderContext, Size, Widget, WidgetExt,
};

use crate::timeline::Timeline;

fn control_buttons() -> impl Widget<Timeline> {
    Flex::row()
        .with_child(
            Button::new("▶").on_click(|_ctx, timeline: &mut Timeline, _env| {
                timeline.play();
            }),
        )
        .with_default_spacer()
        .with_child(
            Button::new("⏸︎").on_click(|_ctx, timeline: &mut Timeline, _env| {
                timeline.pause();
            }),
        )
        .with_default_spacer()
        .with_child(
            Button::new("⏹︎").on_click(|_ctx, timeline: &mut Timeline, _env| {
                timeline.stop();
            }),
        )
        .padding(5.)
}

pub fn timeline_hud() -> impl Widget<Timeline> {
    Flex::column()
        .with_child(
            Label::new(|data: &Timeline, _env: &Env| data.current_time().to_string())
                .with_text_size(20.),
        )
        .with_child(Label::new(|data: &Timeline, _env: &Env| {
            if data.is_playing() {
                "Playing"
            } else {
                "Stopped"
            }
        }))
        .with_child(Flex::row().with_child(control_buttons()))
        .padding(10.)
        .background(Color::BLACK)
        .rounded(10.)
}
