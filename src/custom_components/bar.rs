use gdk::glib::Receiver;

use super::build_component;

pub fn build_bar(drawing_area: gtk::DrawingArea, width: i32, height: i32, draw_rx: Receiver<f64>) {
    build_component(
        drawing_area,
        width,
        height,
        draw_rx,
        |cr, &width_f64, &height_f64, &value| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.set_line_width(2.0);

            // =====[ Border ]=====

            cr.rectangle(0.0, 0.0, width_f64, height_f64);
            cr.stroke().unwrap();

            // =====[ Bar ]=====

            cr.rectangle(0.0, 0.0, width_f64 * value, height_f64);
            cr.fill().unwrap();
        },
    )
}
