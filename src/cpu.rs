use gdk::glib;
use gtk::prelude::LabelExt;
use sysinfo::{ProcessorExt, RefreshKind, System, SystemExt};

use crate::bar::build_bar;
use crate::util::{get_widget, msec_2_duration};

const CPU_UPDATE_INTERVAL: u32 = 1000;

pub fn setup(builder: &gtk::Builder) {
    let mut sys = System::new_with_specifics(RefreshKind::new());
    let label_cpu_percent = get_widget("label_cpu_percent", &builder);

    update(&mut sys, &label_cpu_percent);

    glib::timeout_add_local(msec_2_duration(CPU_UPDATE_INTERVAL), move || {
        update(&mut sys, &label_cpu_percent);

        return glib::Continue(true);
    });

    build_bar(
        builder,
        "drawing_area_cpu_percent_bar",
        500,
        6,
        CPU_UPDATE_INTERVAL as u64,
        || {
            return System::new_with_specifics(RefreshKind::new());
        },
        |sys| {
            sys.refresh_cpu();

            let mut total_percent = 0.0;

            for processor in sys.processors().into_iter() {
                total_percent += processor.cpu_usage();
            }

            return (total_percent / 800.0) as f64;
        },
    );

    for i in 0..8 {
        build_bar(
            builder,
            &format!("drawing_area_cpu{}_percent", i),
            120,
            6,
            CPU_UPDATE_INTERVAL as u64,
            || {
                return System::new_with_specifics(RefreshKind::new());
            },
            move |sys| {
                sys.refresh_cpu();

                return sys.processors()[i].cpu_usage() as f64 / 100.0;
            },
        );
    }
}

fn update(sys: &mut System, label_cpu_percent: &gtk::Label) {
    sys.refresh_cpu();

    let mut total_percent = 0.0;

    for processor in sys.processors().into_iter() {
        total_percent += processor.cpu_usage();
    }

    label_cpu_percent.set_text(&format!("{:.1}%", total_percent / 8.0));
}
