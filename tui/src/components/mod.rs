/*
    SPDX-License-Identifier: AGPL-3.0-or-later
    SPDX-FileCopyrightText: 2025-2026 Shomy
*/
pub mod blinking_stars;
pub mod card_view;
pub mod description_menu;
pub mod dialog;
pub mod dropdown;
pub mod file_explorer;
pub mod progress_bar;
pub mod selectable_list;
// Re-exports :D

pub use blinking_stars::Stars;
pub use description_menu::{DescriptionMenu, DescriptionMenuItem};
pub use dialog::{DialogBuilder, DialogButton};
pub use dropdown::{Dropdown, DropdownOption};
pub use file_explorer::{ExplorerResult, FileExplorer};
pub use progress_bar::ProgressBar;
use ratatui::buffer::Buffer;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::Rect;

use crate::app::AppCtx;
use crate::themes::Theme;

pub trait Component {
    fn tick(&mut self, ctx: &mut AppCtx) {
        let _ = ctx;
    }

    /// Handle key input. Returns `true` if the event was consumed.
    fn handle_key(&mut self, key: KeyEvent, ctx: &mut AppCtx) -> bool {
        let _ = (key, ctx);
        false
    }

    /// Render component contents into the buffer.
    fn render(&mut self, area: Rect, buf: &mut Buffer, theme: &Theme);

    /// Render on top of other components.
    fn render_overlay(&mut self, area: Rect, buf: &mut Buffer, theme: &Theme) {
        let _ = (area, buf, theme);
    }
}
