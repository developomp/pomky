/// Bar graph
/// taken from [gtk3-rs cairo thread example](https://github.com/gtk-rs/gtk3-rs/tree/67f3a1833d303ef292def8d341880f4a92445a5c/examples/cairo_threads)
use gtk;
use gtk::cairo::{Context, Operator};
use gtk::glib;
use gtk::prelude::{Continue, Inhibit, WidgetExt};

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
    let delay = Duration::from_millis(update_interval);

    let drawing_area = get_widget::<gtk::DrawingArea>(widget_name, &builder);
    drawing_area.set_size_request(width, height);

    let (main_thread_tx, main_thread_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (worker_thread_tx, worker_thread_rx) = mpsc::channel();

    // initialize images to be worked by the threads
    worker_thread_tx.send(Image::new(width, height)).unwrap();
    let workspace = Rc::new((RefCell::new(Image::new(width, height)), worker_thread_tx));

    // apply changes to the image
    drawing_area.connect_draw(
        glib::clone!(@weak workspace => @default-return Inhibit(false), move |_, cr| {
            let (ref current_image, _) = *workspace;

            current_image.borrow_mut().with_surface(|surface| {
                // Capture reference to the surface
                cr.set_source_surface(surface, 0.0, 0.0).unwrap();

                cr.paint().unwrap();

                // Release reference to the surface again (removing this line causes an error)
                cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
            });

            return Inhibit(false);
        }),
    );

    // worker thread
    // does the actual drawing
    thread::spawn(glib::clone!(@strong main_thread_tx => move || {
        let mut data = f1();

        for mut received_image in worker_thread_rx.iter() {
            received_image.with_surface(|surface| {
                let cr = Context::new(surface).unwrap();

                // =====[ blear surface ]=====

                cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
                cr.set_operator(Operator::Clear);
                cr.rectangle(0.0, 0.0, width_f64, height_f64);
                cr.paint_with_alpha(1.0).unwrap();
                cr.set_operator(Operator::Over);

                // =====[ border ]=====

                cr.set_source_rgb(1.0, 1.0, 1.0);
                cr.set_line_width(2.0);

                cr.rectangle(0.0, 0.0, width_f64, height_f64);
                cr.stroke().unwrap();

                // =====[ bar ]=====

                cr.rectangle(0.0, 0.0, width_f64 * f2(&mut data), height_f64);
                cr.fill().unwrap();

                surface.flush();
            });

            main_thread_tx.send(received_image).unwrap();

            thread::sleep(delay);
        }
    }));

    // main thread
    // swap images from a double-buffer-like memory
    main_thread_rx.attach(None, move |received_image| {
        let (ref current_image, ref worker_thread_tx) = *workspace;

        // replace image with a new one
        let old_image = current_image.replace(received_image);
        // and send the old one back to the worker thread
        worker_thread_tx.send(old_image).unwrap();

        drawing_area.queue_draw();

        return Continue(true);
    });
}
