use crate::widget;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Mode {
    AddItem,
    DisplayItem,
    DisplayHelp,
}

pub struct App {
    pub mode: Mode,
    pub previous_mode: Mode,
    pub items: Vec<widget::ItemState>,
    pub add_item: widget::AddItemState,
    pub current_item: usize,
    pub summary_scroll_state: SummaryScrollState,
    pub help: widget::HelpWidget,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SummaryScrollState {
    pub offset: usize,
    pub queued_scroll: Option<ScrollDirection>,
}

#[derive(Debug, Clone, Copy)]
pub enum ScrollDirection {
    Up,
    Down,
}
