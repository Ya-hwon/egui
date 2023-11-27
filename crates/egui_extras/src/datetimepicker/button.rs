use chrono::{DateTime, Local};
use egui::{Area, Button, Frame, InnerResponse, Key, Order, RichText, Ui, Widget};

type DateType = DateTime<Local>;

#[derive(Default, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct DateTimePickerButtonState {
    pub picker_visible: bool,
}

pub struct DateTimePickerButton<'a> {
    selection: &'a mut DateType,
    id_source: Option<&'a str>,
    combo_boxes: bool,
    arrows: bool,
    calendar: bool,
    calendar_week: bool,
}

impl<'a> DateTimePickerButton<'a> {
    pub fn new(selection: &'a mut DateType) -> Self {
        Self {
            selection,
            id_source: None,
            combo_boxes: true,
            arrows: true,
            calendar: true,
            calendar_week: true,
        }
    }

    /// Add id source.
    /// Must be set if multiple date picker buttons are in the same Ui.
    #[inline]
    pub fn id_source(mut self, id_source: &'a str) -> Self {
        self.id_source = Some(id_source);
        self
    }

    /// Show combo boxes in date picker popup. (Default: true)
    #[inline]
    pub fn combo_boxes(mut self, combo_boxes: bool) -> Self {
        self.combo_boxes = combo_boxes;
        self
    }

    /// Show arrows in date picker popup. (Default: true)
    #[inline]
    pub fn arrows(mut self, arrows: bool) -> Self {
        self.arrows = arrows;
        self
    }

    /// Show calendar in date picker popup. (Default: true)
    #[inline]
    pub fn calendar(mut self, calendar: bool) -> Self {
        self.calendar = calendar;
        self
    }

    /// Show calendar week in date picker popup. (Default: true)
    #[inline]
    pub fn calendar_week(mut self, week: bool) -> Self {
        self.calendar_week = week;
        self
    }
}

impl<'a> Widget for DateTimePickerButton<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let id = ui.make_persistent_id(self.id_source);
        let mut button_state = ui
            .memory_mut(|mem| mem.data.get_persisted::<DateTimePickerButtonState>(id))
            .unwrap_or_default();

        let mut text = RichText::new(format!("{}", self.selection.format("%+")));
        let visuals = ui.visuals().widgets.open;
        if button_state.picker_visible {
            text = text.color(visuals.text_color());
        }
        let mut button = Button::new(text);
        if button_state.picker_visible {
            button = button.fill(visuals.weak_bg_fill).stroke(visuals.bg_stroke);
        }
        let mut button_response = ui.add(button);
        if button_response.clicked() {
            button_state.picker_visible = true;
            ui.memory_mut(|mem| mem.data.insert_persisted(id, button_state.clone()));
        }

        if button_state.picker_visible {
            let width = 333.0;
            let mut pos = button_response.rect.left_bottom();
            let width_with_padding = width
                + ui.style().spacing.item_spacing.x
                + ui.style().spacing.window_margin.left
                + ui.style().spacing.window_margin.right;
            if pos.x + width_with_padding > ui.clip_rect().right() {
                pos.x = button_response.rect.right() - width_with_padding;
            }

            // Check to make sure the calendar never is displayed out of window
            pos.x = pos.x.max(ui.style().spacing.window_margin.left);

            //TODO(elwerene): Better positioning

            let InnerResponse {
                inner: saved,
                response: area_response,
            } = Area::new(ui.make_persistent_id(self.id_source))
                .order(Order::Foreground)
                .fixed_pos(pos)
                .constrain_to(ui.ctx().screen_rect())
                .show(ui.ctx(), |ui| {
                    let frame = Frame::popup(ui.style());
                    frame
                        .show(ui, |ui| {
                            ui.set_min_width(width);
                            ui.set_max_width(width);

                            ui.label(format!("{}", self.selection.timestamp()));
                            ui.label(format!("{}", self.selection.format("%Y")));

                            if ui.button("+1Y").clicked() {
                                *self.selection = self
                                    .selection
                                    .checked_add_months(chrono::Months::new(12))
                                    .unwrap();
                            }

                            ui.button("Save").clicked()
                        })
                        .inner
                });

            if saved {
                button_response.mark_changed();
            }

            if !button_response.clicked()
                && (ui.input(|i| i.key_pressed(Key::Escape)) || area_response.clicked_elsewhere())
            {
                button_state.picker_visible = false;
                ui.memory_mut(|mem| mem.data.insert_persisted(id, button_state));
            }
        }

        button_response
    }
}
