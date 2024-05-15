use std::sync::{Arc, Mutex};

use gpui::{AnyView, AnyWeakView, Axis, Bounds, StyleRefinement, View};
use ui::{div, Element, IntoElement, ParentElement, Pixels, Styled, StyledExt, ViewContext};

use crate::{pane::Pane, AppState, Workspace};

#[derive(Clone)]
pub(crate) struct PaneAxis {
    pub axis: Axis,
    pub members: Vec<Member>,
    pub flexes: Arc<Mutex<Vec<f32>>>,
    pub bounding_boxes: Arc<Mutex<Vec<Option<Bounds<Pixels>>>>>,
}

#[derive(Clone)]
pub(crate) enum Member {
    Axis(PaneAxis),
    Pane(View<Pane>),
}

impl Member {
    fn contains(&self, needle: &View<Pane>) -> bool {
        match self {
            Member::Axis(axis) => axis.members.iter().any(|member| member.contains(needle)),
            Member::Pane(pane) => pane == needle,
        }
    }

    fn first_pane(&self) -> View<Pane> {
        match self {
            Member::Axis(axis) => axis.members[0].first_pane(),
            Member::Pane(pane) => pane.clone(),
        }
    }

    pub fn render(
        &self,
        basis: usize,
        active_pane: &View<Pane>,
        zoomed: Option<&AnyWeakView>,
        app_state: &Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) -> impl IntoElement {
        match self {
            Member::Pane(pane) => div()
                .relative()
                .flex_1()
                .size_full()
                .child(
                    AnyView::from(pane.clone())
                        /* .cached(StyleRefinement::default().v_flex().size_full()), */
                )
                .into_any(),
            Member::Axis(_) => div().into_any(),
        }
    }
}

#[derive(Clone)]
pub struct PaneGroup {
    pub(crate) root: Member,
}

impl PaneGroup {
    pub(crate) fn with_root(root: Member) -> Self {
        Self { root }
    }

    pub fn new(pane: View<Pane>) -> Self {
        Self {
            root: Member::Pane(pane),
        }
    }

    pub(crate) fn render(
        &self,
        active_pane: &View<Pane>,
        zoomed: Option<&AnyWeakView>,
        app_state: &Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) -> impl IntoElement {
        self.root.render(0, active_pane, zoomed, app_state, cx)
    }
}
