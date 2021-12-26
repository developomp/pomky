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

use crate::drawing::draw_image_if_dirty;
use crate::image::Image;
use crate::util::get_widget;

pub fn build_bar<F1: 'static, F2: 'static>(
    builder: &gtk::Builder,
    widget_name: &str,
    width: i32,
    height: i32,
    update_interval: u64,
    f1: F1,
    f2: F2,
) where
    F1: Fn() -> sysinfo::System + std::marker::Send,
    F2: Fn(&mut sysinfo::System) -> f64 + std::marker::Send,
{
    let width_f64 = width as f64;
    let height_f64 = height as f64;

    let drawing_area: gtk::DrawingArea = get_widget(widget_name, &builder);

    drawing_area.set_size_request(width, height);

    // This is the channel for sending results from the worker thread to the main thread
    // For every received image, queue the corresponding part of the DrawingArea for redrawing
    let (ready_tx, ready_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    // This is the channel for sending image from the GUI thread to the workers so that the
    // worker can draw the new content into them
    let (tx, rx) = mpsc::channel();

    let initial_image = Image::new(width, height);

    // Send a image to the worker thread for drawing the new content
    tx.send(initial_image.clone()).unwrap();

    let delay = Duration::from_secs(update_interval);

    // Spawn the worker thread
    thread::spawn(glib::clone!(@strong ready_tx => move || {
        let mut sys = f1();

        for mut image in rx.iter() {
            image.with_surface(|surface| {
                let cr = Context::new(surface).expect("Can't create a Cairo context");

                cr.set_source_rgb(1.0, 1.0, 1.0);
                cr.set_line_width(2.0);

                // =====[ bar ]=====

                cr.rectangle(0.0, 0.0, width_f64 * f2(&mut sys), height_f64);
                cr.fill().expect("Failed to fill bar");

                // =====[ border ]=====

                cr.rectangle(0.0, 0.0, width_f64, height_f64);
                cr.stroke().expect("Failed to draw border");

                surface.flush();
            });

            // Send the finished image back to the GUI thread
            ready_tx.send(image).unwrap();

            thread::sleep(delay);
        }
    }));

    // The connect-draw signal and the timeout handler closures have to be 'static, and both need
    // to have access to our images and related state.
    let workspace = Rc::new((RefCell::new(initial_image.clone()), tx));

    // Whenever the drawing area has to be redrawn, render the latest images in the correct
    // locations
    drawing_area.connect_draw(
        glib::clone!(@weak workspace => @default-return gtk::Inhibit(false), move |_, cr| {
            let (ref image, _) = *workspace;

                image.borrow_mut().with_surface(|surface| {
                    draw_image_if_dirty(cr, surface, (width, height));
                });


            gtk::Inhibit(false)
        }),
    );

    ready_rx.attach(None, move |new_image| {
        let (ref image, ref tx) = *workspace;

        // Swap the newly received image with the old stored one and send the old one back to
        // the worker thread
        let tx = &tx;
        let image = image.replace(new_image);
        let _ = tx.send(image);

        drawing_area.queue_draw();

        Continue(true)
    });
}
