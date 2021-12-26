use gdk::glib;
use gtk::prelude::LabelExt;
use sysinfo::{ProcessorExt, RefreshKind, System, SystemExt};

use crate::{bar::build_bar, util::get_widget};

const CPU_UPDATE_INTERVAL: u32 = 1;

pub fn setup(builder: &gtk::Builder) {
    // todo: there's probably a better way of doing this...
    let cpu_percent_labels: [gtk::Label; 8] = [
        get_widget("label_cpu0_percent", &builder),
        get_widget("label_cpu1_percent", &builder),
        get_widget("label_cpu2_percent", &builder),
        get_widget("label_cpu3_percent", &builder),
        get_widget("label_cpu4_percent", &builder),
        get_widget("label_cpu5_percent", &builder),
        get_widget("label_cpu6_percent", &builder),
        get_widget("label_cpu7_percent", &builder),
    ];

    let mut sys = System::new_with_specifics(RefreshKind::new());
    update(&mut sys, &cpu_percent_labels);

    for i in 0..8 {
        build_bar(
            builder,
            format!("drawing_area_cpu{}_percent", i).as_str(),
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

    glib::timeout_add_seconds_local(CPU_UPDATE_INTERVAL, move || {
        update(&mut sys, &cpu_percent_labels);

        return glib::Continue(true);
    });
}

fn update(sys: &mut System, cpu_percent_labels: &[gtk::Label; 8]) {
    sys.refresh_cpu();

    for (i, processor) in sys.processors().into_iter().enumerate() {
        cpu_percent_labels[i].set_text(format!("{:.1}%", processor.cpu_usage()).as_str());
    }
}
