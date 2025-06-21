use pulldown_cmark::{Parser, Event, Tag, TagEnd};
use egui::RichText;
use eframe::egui;

pub fn render_md(ui: &mut egui::Ui, md: &str) {
    let parser = Parser::new(md);

    let mut heading: Option<u32> = None;
    let mut buffer = String::new();
    let mut is_bold = false;
    let mut is_code_block = false;

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
                Tag::Strong => {
                    is_bold = true;
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
                TagEnd::Strong => {
                    is_bold = true;
                }
                _ => {}
            },
            Event::Rule => {
                ui.separator();
            },
            Event::Text(text) => {
                if heading.is_some() {
                    buffer.push_str(&text);
                } else if is_bold {
                    ui.label(RichText::new(text.as_ref()).strong());
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
