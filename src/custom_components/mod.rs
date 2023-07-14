pub mod bar;
pub mod graph;

use gdk::cairo::Operator;
use gdk::glib::Receiver;
use gtk;
use gtk::cairo::Context;
use gtk::glib;
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use crate::image::Image;

pub fn build_component<T, F: 'static>(
    drawing_area: gtk::DrawingArea,
    width: i32,
    height: i32,
    draw_rx: Receiver<T>,
    draw_func: F,
) where
    F: Fn(&Context, &f64, &f64, &T) + std::marker::Send,
{
    let width_f64 = width as f64;
    let height_f64 = height as f64;

    let image_ref = Rc::new(RefCell::new(Image::new(width, height)));

    drawing_area.set_size_request(width, height);
    drawing_area.connect_draw(
        glib::clone!(@weak image_ref => @default-return Inhibit(false), move |_, cr| {
            image_ref.borrow_mut().with_surface(|surface| {
                // Capture reference to the surface
                cr.set_source_surface(surface, 0.0, 0.0).unwrap();

                cr.paint().unwrap();

                // Release reference to the surface again (removing this line causes a panic)
                cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
            });

            return Inhibit(false);
        }),
    );

    draw_rx.attach(None, move |data| {
        image_ref.borrow_mut().with_surface(|surface| {
            let cr = Context::new(surface).unwrap();

            // =====[ Clear surface ]=====

            cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
            cr.set_operator(Operator::Clear);
            cr.rectangle(0.0, 0.0, width_f64, height_f64);
            cr.paint_with_alpha(1.0).unwrap();
            cr.set_operator(Operator::Over);

            // Drawing happens here
            draw_func(&cr, &width_f64, &height_f64, &data);

            surface.flush();
        });

        drawing_area.queue_draw();

        return Continue(true);
    });
}
