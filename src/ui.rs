use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Gauge};
use tui_widget_list::{ListBuilder, ListState, ListView};
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
            let gauge_style = if context.is_selected {
                Style::default().green().on_black()
            } else {
                Style::default().black().on_green()
            };
            let item = Gauge::default()
                .use_unicode(true)
                .gauge_style(gauge_style)
                .ratio(resource.progress);

            // Return the size of the widget along the main axis.
            let main_axis_size = 1;

            (item, main_axis_size)
        });

        let mut list_state = ListState::default();
        let list = ListView::new(builder, self.resources.len()).block(block);

        list.render(area, buf, &mut list_state);
    }
}
