use gdk::glib;
use gdk::glib::Receiver;
use gdk::prelude::Continue;

use super::build_component;

pub fn build_graph(
    drawing_area: gtk::DrawingArea,
    width: i32,
    height: i32,
    graph_rx: Receiver<u64>,
    max_value: Option<u64>,
) {
    let (draw_tx, draw_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    // values used to draw the graph
    let mut values = [0; 500];

    // index of the starting point of circular array
    let mut i_start: usize = 0;

    // fill circular array
    graph_rx.attach(None, move |value| {
        values[i_start] = value;

        let max = if max_value.is_none() {
            highest_in_array(&values)
        } else {
            max_value.unwrap()
        };

        draw_tx.send((i_start, max, values)).unwrap();

        if i_start >= width as usize {
            i_start = 0;
        }

        i_start += 1;

        return Continue(true);
    });

    build_component(
        drawing_area,
        width,
        height,
        draw_rx,
        move |cr, &width_f64, &height_f64, &(index, max, values)| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.set_line_width(2.0);

            // =====[ Border ]=====

            cr.rectangle(0.0, 0.0, width_f64, height_f64);
            cr.stroke().unwrap();

            // =====[ Graph ]=====

            let mut i_val = index.clone();
            let mut i_pos = 0; // position of the current line being drawn (from right to left)

            loop {
                let height_scale = values[i_val] as f64 / max as f64;
                let height_scale = if height_scale.is_nan() {
                    0.0
                } else {
                    height_scale
                };

                cr.move_to(width_f64 - 1.0 - i_pos as f64, height_f64);
                cr.line_to(
                    width_f64 - 1.0 - i_pos as f64,
                    height_f64 - height_f64 * height_scale,
                );

                if i_val == 0 {
                    i_val = width as usize;
                }

                i_val -= 1;
                i_pos += 1;

                if i_pos >= width - 2 {
                    break;
                }
            }

            cr.stroke().unwrap();
        },
    )
}

fn highest_in_array(array: &[u64]) -> u64 {
    let mut max = 0;

    for elem in array {
        max = *elem.max(&max);
    }

    return max;
}
