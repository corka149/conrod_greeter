use conrod::{self, widget, Colorable, Positionable, Widget};

// REPLACES widget_ids! macro
struct Ids {
    text: conrod::widget::Id, // Add more fields for more IDs
                              // toggles: conrod::widget::id::List <- Can be created by conrod::widget::id::List::new()
}
impl Ids {
    pub fn new(mut generator: conrod::widget::id::Generator) -> Self {
        Ids { text: generator.next() }
    }
}

pub struct WidgetRegister {
    ids: Ids,
}
impl WidgetRegister {
    pub fn new(ui: &mut conrod::Ui) -> WidgetRegister {
        WidgetRegister {
            // Ids::new creates the the widget structure.
            ids: Ids::new(ui.widget_id_generator()),
        }
    }

    pub fn register(&self, ui: &mut conrod::Ui) {
        let ui = &mut ui.set_widgets();
        widget::Text::new("Hello World!")
            .middle_of(ui.window)
            .color(conrod::color::WHITE)
            .font_size(32)
            .set(self.ids.text, ui);
    }
}
