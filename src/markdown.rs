use pulldown_cmark::{Parser, Event, Tag, TagEnd};
use egui::RichText;
use eframe::egui;

/*
 * TODO:
 * You can extend this as you wish.
 * Maybe move it in different place.
 * Maybe split it...
 *
 * Parse md to RichText using pulldown_cmark and egui
 */ 
pub fn render_md(ui: &mut egui::Ui, md: &str) {
    let parser = Parser::new(md);
    let mut buffer = String::new();

    let mut heading: Option<u32> = None;

    let mut is_bold = false;
    let mut is_italic = false;
    let mut is_code_block = false;

    let mut is_list_item = false;
    let mut ordered_index = 1;
    let mut list_prefix: Option<String> = None;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading {level, .. } => {
                    heading = Some(level as u32);
                    buffer.clear();
                }
                Tag::CodeBlock(_) => {
                    is_code_block = true;
                    buffer.clear();
                }
                Tag::List(Some(start)) => {
                    ordered_index = start;
                }
                Tag::List(None) => {
                    ordered_index = 0;
                }
                Tag::Item => {
                    is_list_item = true;
                    list_prefix = Some(if ordered_index == 0 {
                        "- ".to_string()
                    } else {
                        let prefix = format!("{}. ", ordered_index);
                        ordered_index += 1;
                        prefix
                    });
                }
                Tag::Strong => {
                    is_bold = true;
                }
                Tag::Emphasis => { // italic
                    is_italic = true;
                }
                _ => {}
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::Heading(_) => {
                    if let Some(level) = heading.take() {
                        let rich = match level {
                            1 => RichText::new(&buffer).heading(),
                            2 => RichText::new(&buffer).strong().size(22.0),
                            3 => RichText::new(&buffer).strong().size(18.0),
                            _ => RichText::new(&buffer).size(16.0),
                        };
                        ui.label(rich);
                        buffer.clear();
                    }
                }
                TagEnd::CodeBlock => {
                    is_code_block = false;

                    ui.label(
                        RichText::new(&buffer)
                            .monospace()
                            .background_color(egui::Color32::DARK_GRAY)
                            .color(egui::Color32::LIGHT_GRAY),
                    );
                    buffer.clear();
                }
                TagEnd::Item => {
                    is_list_item = false;
                    list_prefix = None;
                }
                TagEnd::Strong => {
                    is_bold = false;
                }
                TagEnd::Emphasis => {
                    is_italic = false;
                }
                _ => {}
            },
            Event::Rule => {
                ui.separator();
            },
            Event::Text(text) => {
                if heading.is_some() {
                    buffer.push_str(&text);
                } else if is_list_item {
                    if let Some(prefix) = list_prefix.take() {
                        let l = format!("{}{}", prefix, text.as_ref());
                        ui.label(l);
                    }
                } else if is_bold {
                    ui.label(RichText::new(text.as_ref()).strong());
                } else if is_italic {
                    ui.label(RichText::new(text.as_ref()).italics());
                } else {
                    ui.label(RichText::new(text.as_ref()));
                }
            },
            Event::Code(code) => {
                ui.label(
                    RichText::new(code.as_ref())
                        .monospace()
                        .background_color(egui::Color32::DARK_GRAY)
                        .color(egui::Color32::LIGHT_GRAY),
                );
            }
            Event::SoftBreak | Event::HardBreak => {
                if is_code_block {
                    buffer.push('\n');
                } else {
                    ui.separator();
                }
            }
            _ => {}
        }
    }
}
