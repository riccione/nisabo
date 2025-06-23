use pulldown_cmark::{LinkType, Parser, Event, Tag, TagEnd};
use egui::{Color32, Context, OpenUrl, RichText, FontId, TextStyle};
use egui::text::{LayoutJob, TextFormat};
use eframe::egui;

/*
 * TODO:
 * You can refactor this as you wish.
 * Parse md to RichText using pulldown_cmark and egui
 */ 
#[derive(Default)]
struct RenderState {
    is_paragraph: bool,
    is_list_item: bool,
    is_blockquote: bool,
    is_code: bool,
    is_bold: bool,
    is_italic: bool,
    list_prefix: Option<String>,
    ordered_index: usize,
    heading_level: Option<u32>,
    link: Option<String>,
}

pub fn render_md(ui: &mut egui::Ui, ctx: &egui::Context, md: &str) {
    let parser = Parser::new(md);
    let mut buffer = String::new();

    let mut state = RenderState {
        is_paragraph: false,
        is_list_item: false,
        is_blockquote: false,
        is_code: false,
        is_bold: false,
        is_italic: false,
        list_prefix: None,
        ordered_index: 1,
        heading_level: None,
        link: None,
    };

    let mut layout_job = LayoutJob::default();
    
    for event in parser {
        match event {
            Event::Start(tag) => start_tag(tag, &mut state, &mut buffer, &mut layout_job),
            Event::End(tag) => end_tag(tag, &mut state, ui, &mut buffer, &layout_job),
            Event::Text(text) => event_text(&text, &mut state, ui, ctx, &mut buffer, &mut layout_job),
            Event::Code(code) => {
                ui.label(
                    RichText::new(code.as_ref())
                        .monospace()
                        .background_color(egui::Color32::DARK_GRAY)
                        .color(egui::Color32::LIGHT_GRAY),
                );
            },
            Event::Rule => { ui.separator(); },
            Event::SoftBreak | Event::HardBreak if state.is_paragraph => {
                //ui.allocate_exact_size(vec2(0.0, 12.0), Sense::hover());
                //ui.end_row();
                layout_job.append("\n", 0.0, TextFormat::default());
            },
            _ => {}
        }
    }
}

fn start_tag(tag: Tag, state: &mut RenderState, buffer: &mut String, layout_job: &mut LayoutJob) {
    match tag {
        Tag::Paragraph => {
            state.is_paragraph = true;
            *layout_job = LayoutJob::default();
        }
        Tag::BlockQuote(_) => {
            state.is_blockquote = true;
            buffer.clear();
        }
        Tag::Heading {level, .. } => {
            state.heading_level = Some(level as u32);
            buffer.clear();
        }
        Tag::CodeBlock(_) => {
            state.is_code = true;
            buffer.clear();
        }
        Tag::List(Some(start)) => {
            state.ordered_index = start as usize;
        }
        Tag::List(None) => {
            state.ordered_index = 0;
        }
        Tag::Item => {
            state.is_list_item = true;
            state.list_prefix = Some(if state.ordered_index == 0 {
                "- ".to_string()
            } else {
                let prefix = format!("{}. ", state.ordered_index);
                state.ordered_index += 1;
                prefix
            });
        }
        Tag::Strong => {
            state.is_bold = true;
        }
        Tag::Emphasis => { // italic
            state.is_italic = true;
        }
        Tag::Link {link_type: LinkType::Inline, dest_url, ..} => {
            state.link = Some(dest_url.to_string());
        }
        _ => {}
    }
}

fn end_tag(
    tag: TagEnd,
    state: &mut RenderState,
    ui: &mut egui::Ui,
    buffer: &mut String,
    layout_job: &LayoutJob) {
    let b = buffer.as_str();
    match tag {
        TagEnd::Paragraph => {
            state.is_paragraph = false;
            ui.label(layout_job.clone());
            ui.add_space(7.0);
        }
        TagEnd::Heading(_) => {
            if let Some(level) = state.heading_level.take() {
                let rt = match level {
                    1 => RichText::new(b).heading(),
                    2 => RichText::new(b).strong().size(22.0),
                    3 => RichText::new(b).strong().size(18.0),
                    _ => RichText::new(b).size(16.0),
                };
                ui.label(rt);
                buffer.clear();
            }
        }
        TagEnd::BlockQuote(_) => {
            state.is_blockquote = false;
            ui.add_space(7.0);
            egui::Frame::new()
                .fill(Color32::LIGHT_GRAY)
                .inner_margin(7.0)
                .show(ui, |ui| {
                    ui.label(RichText::new(buffer.as_str().trim_end())
                             .italics()
                             .color(Color32::DARK_GRAY));
                });
            buffer.clear();
        }
        TagEnd::CodeBlock => {
            ui.label(
                RichText::new(b)
                    .monospace()
                    .background_color(egui::Color32::DARK_GRAY)
                    .color(egui::Color32::LIGHT_GRAY),
            );
            buffer.clear();
            state.is_code = false;
        }
        TagEnd::Item => {
            state.is_list_item = false;
            state.list_prefix = None;
        }
        TagEnd::Strong => { state.is_bold = false; }
        TagEnd::Emphasis => { state.is_italic = false; }
        TagEnd::Link => { state.link = None; }
        _ => {}
    }
}

fn event_text(
    text: &str,
    state: &mut RenderState,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    buffer: &mut String,
    layout_job: &mut LayoutJob) {
        if let Some(link) = &state.link {
            if link.starts_with('#') {
                let anchor = &link[1..];

                if ui.link(text).clicked() {
                    println!("Clicked on anchor link to: {}", anchor);
                }
            } else {
                if ui.link(text).clicked() {
                    ctx.open_url(egui::OpenUrl {
                        url: link.to_string(),
                        new_tab: true,
                    });
                    println!("Clicked on external link to: {}", link);
                }
            }
            
            return;
        }

        if state.is_blockquote {
            buffer.push_str(&text);
            buffer.push('\n');
        } else if state.is_code {
            buffer.push_str(&text);
        } else if state.heading_level.is_some() {
            buffer.push_str(&text);
        } else if state.is_paragraph {
            let style = ui.style();
            let font_id = if state.is_bold {
                style.text_styles.get(&TextStyle::Heading).cloned()
            } else {
                style.text_styles.get(&TextStyle::Body).cloned()
            }
            .unwrap_or_else(|| FontId::proportional(14.0)); // fallback

            let mut format = TextFormat {
                font_id,
                italics: state.is_italic || state.is_blockquote,
                color: if state.is_blockquote {
                    Color32::DARK_GRAY
                } else {
                    ui.style().visuals.text_color()
                },
                background: if state.is_blockquote {
                    Color32::LIGHT_GRAY
                } else {
                    Color32::TRANSPARENT
                },
                ..Default::default()
            };

            layout_job.append(&text, 0.0, format);
        } else if state.is_list_item {
            if let Some(prefix) = state.list_prefix.take() {
                let l = format!("{}{}", prefix, text);
                ui.label(l);
            }
        } else {
            let mut rt = RichText::new(text);
            if state.is_bold {
                rt = rt.strong();
            } 
            if state.is_italic {
                rt = rt.italics();
            } 
            ui.label(rt);
        }
}
