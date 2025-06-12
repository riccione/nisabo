use eframe::egui::{
    Response,
    Sense,
    Ui,
    Vec2,
    Align2,
    TextStyle,
};

pub fn left_aligned_button(ui: &mut Ui, text: &str, width: f32, height: f32) -> Response {
    let size = Vec2::new(width, height);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    let visuals = ui.style().interact(&response);

    if ui.is_rect_visible(rect) {
        ui.painter().rect_filled(rect, 4.0, visuals.bg_fill);

        let text_pos = rect.left_center() + Vec2::new(6.0, 0.0);

        ui.painter().text(
            text_pos,
            Align2::LEFT_CENTER,
            text,
            TextStyle::Button.resolve(ui.style()),
            visuals.text_color(),
        );
    }
    response
}
