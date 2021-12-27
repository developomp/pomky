pub mod bar;
pub mod graph;

use gtk;
use gtk::cairo::Context;
use gtk::glib;
use gtk::prelude::{Continue, Inhibit, WidgetExt};

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::image::Image;
use crate::util::get_widget;

pub fn build_component<T: 'static, F1: 'static, F2: 'static, F3: 'static>(
    builder: &gtk::Builder,
    widget_name: &str,
    width: i32,
    height: i32,
    update_interval: u64,
    f1: F1,
    f2: F2,
    f3: F3,
) where
    F1: FnOnce() -> T + std::marker::Send,
    F2: Fn(&mut T) -> f64 + std::marker::Send,
    F3: Fn(&Context, &f64, &f64, &f64) + std::marker::Send,
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

                f3(&cr, &width_f64, &height_f64, &f2(&mut data));

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
