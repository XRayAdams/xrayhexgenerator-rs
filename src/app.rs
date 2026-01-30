// (c) 2025,26 Konstantin Adamov, licensed under MIT

use libadwaita as adw;
use gtk4::prelude::*;
use gtk4::{Align, PolicyType, TextBuffer, TextView, glib};
use adw::ToastOverlay;
use relm4::actions::RelmActionGroup;
use relm4::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::generators::base_generator::BaseGenerator;
use crate::generators::byte_sequence::ByteSequenceGenerator;
use crate::generators::custom_generator::CustomGenerator;
use crate::generators::guid_generator::GUIDGenerator;
use crate::generators::hex_color::HexColorGenerator;
use crate::generators::hex_color_with_alpha::HexColorWithAlphaGenerator;
use crate::generators::mac_address::MacAddressGenerator;
use crate::generators::prefixed_hex::PrefixedHexGenerator;
use crate::helpers::actions::{AboutAction, WindowActionGroup, create_about_action};
use crate::helpers::constants::{APP_NAME, SPACING_LARGE, SPACING_MEDIUM};
use crate::helpers::number_editor::{CounterOutput, NumberEditor, NumberEditorInput};

#[derive(Serialize, Deserialize)]
struct AppSettings {
    selected_index: usize,
    digits: usize,
    lines: usize,
    output_uppercase: bool,
}
impl Default for AppSettings {
    fn default() -> Self {
        Self {
            selected_index: 0,
            digits: 16,
            lines: 10,
            output_uppercase: true,
        }
    }
}

pub struct App {
    selected_index: usize,
    digits: usize,
    lines: usize,
    output_uppercase: bool,
    result_text: String,
    generators: Vec<Box<dyn BaseGenerator>>,
    gen_names: gtk4::StringList,
    digits_widget: Controller<NumberEditor>,
    lines_widget: Controller<NumberEditor>,
    toast_overlay: Option<ToastOverlay>,
    text_view: Option<TextView>,
}



#[derive(Debug)]
pub enum Messages {
    Generate,
    CopyToClipboard,
    UpdateSelectedIndex(usize),
    UpdateDigits(usize),
    UpdateLines(usize),
    UpdateOutputUppercase(bool),
}

impl App {
    fn get_config_path() -> PathBuf {
        let mut path = gtk4::glib::user_config_dir();
        path.push("xrayhexgenerator");
        std::fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }

    fn load_config() -> AppSettings {
        let path = Self::get_config_path();
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(settings) = serde_json::from_str::<AppSettings>(&content) {
                return settings;
            }
        }
        AppSettings::default()
    }
    fn save_config(&self) {
        let path = Self::get_config_path();
        let settings = AppSettings {
            selected_index: self.selected_index,
            digits: self.digits,
            lines: self.lines,
            output_uppercase: self.output_uppercase,
        };
        if let Ok(content) = serde_json::to_string_pretty(&settings) {
            let _ = fs::write(path, content);
        }
    }

    fn get_def_generators() -> Vec<Box<dyn BaseGenerator>> {
        vec![
            Box::new(CustomGenerator {}),
            Box::new(MacAddressGenerator {}),
            Box::new(ByteSequenceGenerator {}),
            Box::new(GUIDGenerator {}),
            Box::new(HexColorGenerator {}),
            Box::new(HexColorWithAlphaGenerator {}),
            Box::new(PrefixedHexGenerator {}),
        ]
    }

    fn get_app_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = Messages;
    type Output = ();

    menu! {
        main_menu: {
            section! {
                "_About" => AboutAction,
            }
        }
    }
    view! {
        #[root]
        main_window = adw::ApplicationWindow {
            set_visible: true,
            set_title: Some(APP_NAME),
            set_default_size: (900, 600),

            #[name = "toast_overlay"]
            adw::ToastOverlay {
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    adw::HeaderBar {
                        pack_end = &gtk::MenuButton {
                            set_icon_name: "open-menu-symbolic",
                            set_menu_model: Some(&main_menu),
                            set_direction: gtk::ArrowType::Down,
                            set_can_focus: false,
                        }
                    },

                    gtk::Box{
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: SPACING_MEDIUM,
                        set_margin_top: SPACING_LARGE,
                        set_margin_bottom: SPACING_LARGE,
                        set_margin_start: SPACING_LARGE,
                        set_margin_end: SPACING_LARGE,

                        gtk::Box{
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: SPACING_MEDIUM,

                            gtk::Label {
                                set_label: "Generator :",
                                set_halign: Align::Start,
                            },
                            gtk::DropDown {
                                set_halign: Align::Start,
                                set_model: Some(&model.gen_names),
                                set_selected :model.selected_index as u32,

                                connect_selected_item_notify[sender] => move |dropdown| {
                                    sender.input(Messages::UpdateSelectedIndex(dropdown.selected() as usize));
                                },
                            }
                        },

                        gtk::Box{
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: SPACING_MEDIUM,

                            model.digits_widget.widget(),
                            model.lines_widget.widget(),

                            gtk::Box{
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 5,
                                set_halign: Align::Center,
                                    gtk::Label{
                                    set_label: "Output in Uppercase",
                                },
                                gtk::Switch{
                                    set_active: model.output_uppercase,
                                    set_halign: Align::Center,
                                    connect_state_set[sender] => move |_, state| {
                                        let _ = sender.input(Messages::UpdateOutputUppercase(state));
                                        glib::Propagation::Proceed
                                    },
                                }
                            },

                        },
                        gtk::Box{
                            set_orientation: gtk::Orientation::Vertical,
                            set_margin_top: SPACING_MEDIUM,

                            gtk::ScrolledWindow {
                                set_hscrollbar_policy: PolicyType::Never,
                                set_vexpand: true,
                                set_hexpand: true,
                                #[name = "result_text_view"]
                                gtk::TextView {
                                    set_editable: false,
                                    set_wrap_mode: gtk::WrapMode::Word,
                                    set_vexpand: true,
                                    add_css_class: "card",
                                    set_monospace: true,
                                    set_left_margin: SPACING_MEDIUM,
                                    set_right_margin: SPACING_MEDIUM,
                                    set_top_margin: SPACING_MEDIUM,
                                    #[watch]
                                    set_buffer: Some(&TextBuffer::builder()
                                        .text(model.result_text.as_str())
                                        .build()
                                    ),
                                },
                            },
                        },
                        gtk::Box{
                            set_halign: Align::Start,
                            set_spacing: SPACING_LARGE,
                            set_margin_vertical: SPACING_MEDIUM,
                            gtk::Button::with_mnemonic("_Generate") {
                                set_halign: Align::Start,
                                connect_clicked[sender] => move |_| {
                                    sender.input(Messages::Generate);
                                },
                            },
                            gtk::Button::with_mnemonic("_Copy to Clipboard") {
                                #[watch]
                                set_sensitive: !model.result_text.is_empty(),
                                set_halign: Align::Start,
                                connect_clicked[sender] => move|_| {
                                    sender.input(Messages::CopyToClipboard);
                                },
                            },
                        },
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = Self::load_config();

        let generators = Self::get_def_generators();

        let def_digits = generators[settings.selected_index].default_digits();
        let digits_are_editable = generators[settings.selected_index].digist_are_editable();
            
        let mut model = Self {
            selected_index: settings.selected_index,
            digits: settings.digits,
            lines: settings.lines,
            result_text: String::new(),
            toast_overlay: None,
            text_view: None,
            output_uppercase: settings.output_uppercase,
            generators: generators,
            gen_names: gtk4::StringList::new(&[]),
            digits_widget: NumberEditor::builder()
                .launch(("Digits".to_string(), 1, 100, def_digits, digits_are_editable))
                .forward(sender.input_sender(), |output| match output {
                    CounterOutput::ValueChanged(value) => Messages::UpdateDigits(value),
                }),
            lines_widget: NumberEditor::builder()
                .launch(("Lines".to_string(), 1, 1000, settings.lines, true))
                .forward(sender.input_sender(), |output| match output {
                    CounterOutput::ValueChanged(value) => Messages::UpdateLines(value),
                }),
        };

        for generator in model.generators.iter() {
            model.gen_names.append(&generator.name());
        }

        let widgets = view_output!();

        model.toast_overlay = Some(widgets.toast_overlay.clone());
        model.text_view = Some(widgets.result_text_view.clone());

        let about_action =
            create_about_action(widgets.main_window.clone(), Self::get_app_version());

        let mut window_actions = RelmActionGroup::<WindowActionGroup>::new();
        window_actions.add_action(about_action);
        window_actions.register_for_widget(&widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Messages::Generate => {
                self.generators
                    .get(self.selected_index)
                    .map(|generator| {
                        self.result_text = generator.generate(
                            self.digits,
                            self.lines,
                            self.output_uppercase,
                        );
                    });
            }
            Messages::CopyToClipboard => {
                self.text_view
                    .clone()
                    .unwrap()
                    .clipboard()
                    .set_text(self.result_text.as_str());

                self.toast_overlay
                    .clone()
                    .unwrap()
                    .add_toast(adw::Toast::new("Copied to clipboard"));
            }
            Messages::UpdateSelectedIndex(index) => {
                self.selected_index = index;


                self.generators.get(index)
                    .map(|generator| {
                        let enabled = generator.digist_are_editable();
                        let def_digits = generator.default_digits();
                        
                        self.digits_widget
                            .emit(NumberEditorInput::SetEnabled(enabled));
                        self.digits_widget
                            .emit(NumberEditorInput::SetValue(def_digits));
                    });

                self.save_config();
            }
            Messages::UpdateDigits(digits) => {
                self.digits = digits;
                self.save_config();
            }
            Messages::UpdateLines(lines) => {
                self.lines = lines;
                self.save_config();
            }
            Messages::UpdateOutputUppercase(uppercase) => {
                self.output_uppercase = uppercase;
                self.save_config();
            }
        }
    }
}
