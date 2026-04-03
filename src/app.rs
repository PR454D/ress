use eframe::{
    CreationContext, Frame, Storage,
    egui::{
        CentralPanel, ComboBox, Context, FontId, MenuBar, Panel, ScrollArea,
        TextStyle, Ui, ViewportCommand, WidgetText, widgets,
    },
};
use rss::Channel;
use std::collections::BTreeMap;

type FeedItem = (String, String);
#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct App {
    subs: Vec<FeedItem>,
    name_input: String,
    url_input: String,
    selected_value: Option<usize>,
    #[serde(skip)]
    channel: Option<Channel>,
}

impl eframe::App for App {
    fn save(&mut self, _storage: &mut dyn Storage) {}
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        self.set_styles(ui.ctx());
        self.show_top_bar(ui);

        CentralPanel::default().show_inside(ui, |ui| {
            self.rss_form(ui);
            ui.separator();

            self.combo_box(ui);
            if self.channel.is_some() && ui.button("Clear Feed").clicked() {
                self.channel = None;
            }
            self.feed_view(ui);
        });
    }
}

impl App {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn set_styles(&mut self, ctx: &Context) {
        let mut style_map = BTreeMap::new();
        style_map.insert(TextStyle::Heading, FontId::monospace(30.));
        style_map.insert(TextStyle::Body, FontId::monospace(18.));
        style_map.insert(TextStyle::Button, FontId::monospace(22.));
        style_map.insert(TextStyle::Small, FontId::monospace(14.));
        let mut style = (*ctx.global_style()).clone();
        style.text_styles = style_map;
        ctx.set_global_style(style);
    }

    fn clear_inputs(&mut self) {
        self.name_input.clear();
        self.url_input.clear();
    }

    fn show_top_bar(&self, ui: &mut Ui) {
        Panel::top("menu_bar").show_inside(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        ui.send_viewport_cmd(ViewportCommand::Close);
                    }
                });

                ui.menu_button(
                    "Theme",
                    widgets::global_theme_preference_buttons,
                );
            })
        });
    }

    fn rss_form(&mut self, ui: &mut Ui) {
        ui.collapsing("New Rss", |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.name_input);
                ui.label("URL");
                ui.text_edit_singleline(&mut self.url_input);
                ui.horizontal(|ui| {
                    if ui.button("Submit").clicked() {
                        let item: FeedItem =
                            (self.name_input.clone(), self.url_input.clone());
                        self.subs.push(item);
                        self.clear_inputs();
                    }
                    if ui.button("Clear").clicked() {
                        self.clear_inputs();
                    }
                })
            })
        });
    }

    fn combo_box(&mut self, ui: &mut Ui) {
        ComboBox::from_label("Select RSS")
            .selected_text(if let Some(index) = self.selected_value {
                if let Some(item) = self.subs.get(index) {
                    item.0.clone()
                } else {
                    "Select me".to_string()
                }
            } else {
                "Select me".to_string()
            })
            .show_ui(ui, |ui| {
                for (idx, item) in self.subs.iter().enumerate() {
                    if ui
                        .selectable_value(
                            &mut self.selected_value,
                            Some(idx),
                            item.0.clone(),
                        )
                        .clicked()
                        && let Some(sub) = self.subs.get(idx)
                    {
                        match self.get_feed(&sub.1) {
                            Ok(channel) => self.channel = Some(channel),
                            Err(e) => println!("{}", e),
                        }
                    }
                }
            });
    }

    fn get_feed(
        &self,
        url: &str,
    ) -> Result<Channel, Box<dyn std::error::Error>> {
        let content = reqwest::blocking::get(url)?.bytes()?;
        let channel = Channel::read_from(&content[..])?;
        Ok(channel)
    }

    fn feed_view(&mut self, ui: &mut Ui) {
        if let Some(channel) = &self.channel {
            ui.separator();
            ui.heading(channel.title());
            ui.label(channel.description());
            ui.separator();

            ScrollArea::vertical().show(ui, |ui| {
                for item in channel.items() {
                    if let Some(link) = item.link() {
                        ui.hyperlink_to(
                            item.title().unwrap_or("Default title"),
                            link,
                        )
                        .on_hover_text(WidgetText::Text(link.to_string()));
                    } else {
                        ui.heading(item.title().unwrap_or("No title"));
                    }
                    ui.label(item.description().unwrap_or("No description"));
                    ui.separator();
                }
            });
        }
    }
}
