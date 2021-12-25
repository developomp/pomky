/// Bar graph
use gtk;
use gtk::prelude::WidgetExt;

pub fn draw_bar(drawing_area: &gtk::DrawingArea, width: i32, height: i32, value: f64) {
    let width_f64 = width as f64;
    let height_f64 = height as f64;

    drawing_area.set_size_request(width, height);

    drawing_area.connect_draw(move |_, cr| {
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.set_line_width(2.0);

        // =====[ border ]=====

        cr.rectangle(0.0, 0.0, width_f64, height_f64);
        cr.stroke().expect("Failed to draw border");

        // =====[ bar ]=====

        cr.rectangle(0.0, 0.0, width_f64 * value, height_f64);
        cr.fill().expect("Failed to fill bar");

        return gtk::Inhibit(false);
    });
}
