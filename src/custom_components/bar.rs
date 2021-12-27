use gtk::cairo::Operator;

use super::build_component;

pub fn build_bar<T: 'static, F1: 'static, F2: 'static>(
    builder: &gtk::Builder,
    widget_name: &str,
    width: i32,
    height: i32,
    update_interval: u64,
    f1: F1,
    f2: F2,
) where
    F1: FnOnce() -> T + std::marker::Send,
    F2: Fn(&mut T) -> f64 + std::marker::Send,
{
    build_component(
        builder,
        widget_name,
        width,
        height,
        update_interval,
        f1,
        f2,
        |cr, width_f64, height_f64, value| {
            // =====[ blear surface ]=====

            cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
            cr.set_operator(Operator::Clear);
            cr.rectangle(0.0, 0.0, *width_f64, *height_f64);
            cr.paint_with_alpha(1.0).unwrap();
            cr.set_operator(Operator::Over);

            // =====[ border ]=====

            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.set_line_width(2.0);

            cr.rectangle(0.0, 0.0, *width_f64, *height_f64);
            cr.stroke().unwrap();

            // =====[ bar ]=====

            cr.rectangle(0.0, 0.0, width_f64 * value, *height_f64);
            cr.fill().unwrap();
        },
    )
}
