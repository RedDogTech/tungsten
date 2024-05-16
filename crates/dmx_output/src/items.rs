use gpui::{IntoElement, Render, ViewContext, WeakView};
use ui::{
    h_flex, rems, ButtonLike, Color, Icon, IconName, IconSize, Label, LabelCommon, LabelSize,
    ParentElement, Styled,
};
use workspace::{item::ItemHandle, StatusItemView, Workspace};

pub struct DmxIndicator {
    workspace: WeakView<Workspace>,
}

impl DmxIndicator {
    pub fn new(workspace: &Workspace, _cx: &mut ViewContext<Self>) -> Self {
        Self {
            workspace: workspace.weak_handle(),
        }
    }
}

impl Render for DmxIndicator {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex().h(rems(1.375)).gap_2().child(
            ButtonLike::new("diagnostic-indicator").child(
                h_flex()
                    .gap_1()
                    .child(
                        Icon::new(IconName::XCircle)
                            .size(IconSize::Small)
                            .color(Color::Error),
                    )
                    .child(Label::new("0".to_string()).size(LabelSize::Small))
                    .child(
                        Icon::new(IconName::ExclamationTriangle)
                            .size(IconSize::Small)
                            .color(Color::Warning),
                    )
                    .child(Label::new("0".to_string()).size(LabelSize::Small)),
            ),
        )
    }
}

impl StatusItemView for DmxIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        // no-op
    }
}
