/// Bar graph
use gtk;
use gtk::cairo::Context;
use gtk::glib;
use gtk::prelude::{Continue, WidgetExt};

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::image::Image;
use crate::util::get_widget;

pub fn build_bar<F1: 'static, F2: 'static, T: 'static>(
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
    let width_f64 = width as f64;
    let height_f64 = height as f64;
    let delay = Duration::from_secs(update_interval);
    let initial_image = Image::new(width, height);

    let drawing_area = get_widget::<gtk::DrawingArea>(widget_name, &builder);
    drawing_area.set_size_request(width, height);

    let (to_main_tx, to_main_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (to_worker_tx, to_worker_rx) = mpsc::channel();

    to_worker_tx.send(initial_image.clone()).unwrap();

    let workspace = Rc::new((RefCell::new(initial_image.clone()), to_worker_tx));

    // worker thread
    thread::spawn(glib::clone!(@strong to_main_tx => move || {
        let mut data = f1();

        for mut received_image in to_worker_rx.iter() {
            // draw the bar
            received_image.with_surface(|surface| {
                let cr = Context::new(surface).expect("Can't create a Cairo context");

                cr.set_source_rgb(1.0, 1.0, 1.0);
                cr.set_line_width(2.0);

                // =====[ border ]=====

                cr.rectangle(0.0, 0.0, width_f64, height_f64);
                cr.stroke().expect("Failed to draw bar outline");

                // =====[ bar ]=====

                cr.rectangle(0.0, 0.0, width_f64 * f2(&mut data), height_f64);
                cr.fill().expect("Failed to fill bar");

                surface.flush();
            });

            to_main_tx.send(received_image).unwrap();

            thread::sleep(delay);
        }
    }));

    // GUI thread
    drawing_area.connect_draw(
        glib::clone!(@weak workspace => @default-return gtk::Inhibit(false), move |_, cr| {
            let (ref current_image, _) = *workspace;

            current_image.borrow_mut().with_surface(|surface| {
                cr.set_source_surface(surface, 0.0, 0.0).expect("The surface has an invalid state");
                cr.paint().expect("Invalid cairo surface state");

                // Release the reference to the surface again
                cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
            });

            return gtk::Inhibit(false);
        }),
    );

    // main thread
    to_main_rx.attach(None, move |received_image| {
        let (ref old_image, ref to_worker_tx) = *workspace;

        // Swap the newly received image with the old stored one
        let current_image = old_image.replace(received_image);
        // and send the old one back to the worker thread
        to_worker_tx.send(current_image).unwrap();

        drawing_area.queue_draw();

        return Continue(true);
    });
}
