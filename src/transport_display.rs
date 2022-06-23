use druid::{
    kurbo::RoundedRect,
    widget::{Button, Flex, Label},
    Color, Env, RenderContext, Size, Widget, WidgetExt,
};

use crate::transport::Transport;

fn control_buttons() -> impl Widget<Transport> {
    Flex::row()
        .with_child(
            Button::new("▶").on_click(|_ctx, transport: &mut Transport, _env| {
                transport.play();
            }),
        )
        .with_default_spacer()
        .with_child(
            Button::new("⏸︎").on_click(|_ctx, transport: &mut Transport, _env| {
                transport.pause();
            }),
        )
        .with_default_spacer()
        .with_child(
            Button::new("⏹︎").on_click(|_ctx, transport: &mut Transport, _env| {
                transport.stop();
            }),
        )
        .padding(5.)
}

pub fn transport_hud() -> impl Widget<Transport> {
    Flex::column()
        .with_child(
            Label::new(|data: &Transport, _env: &Env| data.current_time().to_string())
                .with_text_size(20.),
        )
        .with_child(Label::new(|data: &Transport, _env: &Env| {
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
