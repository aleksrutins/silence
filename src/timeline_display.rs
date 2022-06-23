use druid::{Widget, Size, kurbo::RoundedRect, RenderContext, Color};

use crate::timeline::Timeline;

pub struct TimelineDisplay;

impl Widget<Timeline> for TimelineDisplay {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut Timeline, env: &druid::Env) {
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &Timeline, env: &druid::Env) {
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &Timeline, data: &Timeline, env: &druid::Env) {
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &Timeline, env: &druid::Env) -> druid::Size {
        bc.constrain(Size::new(200.0, 200.0))
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Timeline, env: &druid::Env) {
        let rounded = RoundedRect::from_rect(ctx.size().to_rect(), 10.0);
        ctx.fill(rounded, &Color::BLACK);
    }
}