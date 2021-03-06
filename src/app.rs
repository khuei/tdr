use crate::widget;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Mode {
    AddWorkspace,
    EditWorkspace,
    DisplayWorkspace,
    AddItem,
    EditItem,
    DisplayItem,
    DisplayHelp,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum HelpMode {
    ItemHelp,
    WorkspaceHelp,
}

pub struct App {
    pub mode: Mode,
    pub previous_mode: Mode,
    pub workspaces: Vec<widget::WorkspaceState>,
    pub add_workspace: widget::AddWorkspaceState,
    pub edit_workspace: widget::EditWorkspaceState,
    pub current_workspace: usize,
    pub items: Vec<Vec<widget::ItemState>>,
    pub add_item: widget::AddItemState,
    pub edit_item: widget::EditItemState,
    pub current_item: usize,
    pub summary_scroll_state: SummaryScrollState,
    pub is_modified: bool,
    pub help_mode: HelpMode,
    pub help_item: widget::HelpItemWidget,
    pub help_workspace: widget::HelpWorkspaceWidget,
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
