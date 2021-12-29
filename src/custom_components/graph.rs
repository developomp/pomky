use gdk::glib;
use gdk::glib::Receiver;
use gdk::prelude::Continue;

use super::build_component;

pub fn build_graph(
    drawing_area: gtk::DrawingArea,
    width: i32,
    height: i32,
    graph_rx: Receiver<u64>,
) {
    let (draw_tx, draw_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    graph_rx.attach(None, move |_| {
        draw_tx.send((0_usize, 0_u64, [0.0; 500])).unwrap();
        return Continue(true);
    });

    build_component(
        drawing_area,
        width,
        height,
        draw_rx,
        |cr, &width_f64, &height_f64, &(index, max, values)| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.set_line_width(2.0);

            // =====[ Border ]=====

            cr.rectangle(0.0, 0.0, width_f64, height_f64);
            cr.stroke().unwrap();

            // =====[ Graph ]=====

            let mut val_i = index.clone();
            let mut pos_i = 0.0;

            loop {
                let height_scale = values[val_i] as f64 / max as f64;
                // println!("{}", height_scale);

                cr.move_to(width_f64 - pos_i as f64, height_f64);
                cr.line_to(width_f64 - pos_i as f64, height_f64 * height_scale);

                val_i += 1;
                pos_i += 1.0;

                if pos_i >= 500.0 {
                    break;
                }

                if val_i > 499 {
                    val_i = 0;
                }
            }

            cr.stroke().unwrap();
        },
    )
}
