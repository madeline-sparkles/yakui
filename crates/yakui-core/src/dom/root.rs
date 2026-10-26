use crate::geometry::Constraints;
use crate::widget::{LayoutContext, Widget};

#[derive(Debug)]
pub struct RootWidget;

impl Widget for RootWidget {
    type Props = ();
    type Response = ();

    fn new(_props: &Self::Props) -> Self {
        Self
    }

    fn update(&mut self, _props: &mut Self::Props) -> Self::Response {}

    fn layout(
        &self,
        mut ctx: LayoutContext<'_, Self::Props>,
        constraints: Constraints,
    ) -> glam::Vec2 {
        ctx.layout.new_layer(ctx.dom);

        let node = ctx.dom.get_current();

        for &child in &node.children {
            ctx.calculate_layout(child, constraints);
        }

        constraints.max
    }
}
