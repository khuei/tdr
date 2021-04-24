use crossterm::event::Event;

use crate::widget;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Mode {
    AddItem,
    DisplayItem,
}

pub struct App {
    pub mode: Mode,
    pub items: Vec<widget::ItemState>,
    pub add_item: widget::AddItemState,
    pub current_item: usize,
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
