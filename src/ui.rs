use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Gauge};
use tui_widget_list::{ListBuilder, ListView};
impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("incremental-tui")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let builder = ListBuilder::new(|context| {
            let resource = self.resources[context.index].clone();
            let resource_label = format!("{} (Lvl {}): {}", resource.resource_type, resource.level, resource.amount);
            let resource_block = if context.is_selected {
                let upgrade_str = "Press <Enter> to upgrade";
                Block::bordered().title(resource_label).title_bottom(upgrade_str).on_black().border_type(BorderType::Rounded)
            } else {
                Block::default().title(resource_label)
            };
            let gauge_style = if context.is_selected {
                Style::default().fg(Color::Green).bg(Color::Black)
            } else {
                Style::default().fg(Color::Blue).bg(Color::Reset)
            };
            let item = Gauge::default()
                .gauge_style(gauge_style)
                .ratio(resource.progress)
                .block(resource_block);

            // Return the size of the widget along the main axis.
            let main_axis_size = if context.is_selected {
                3
            } else {
                2
            };

            (item, main_axis_size)
        });

        let list = ListView::new(builder, self.resources.len())
            .block(block);

        list.render(area, buf, &mut self.list_state.borrow_mut());
    }
}
