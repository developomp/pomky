use gdk::glib;
use gtk::prelude::LabelExt;
use sysinfo::{ProcessorExt, RefreshKind, System, SystemExt};

use crate::{bar::draw_bar, util::get_widget};

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

    let cpu_percent_bars: [gtk::DrawingArea; 8] = [
        get_widget("drawing_area_cpu0_percent", &builder),
        get_widget("drawing_area_cpu1_percent", &builder),
        get_widget("drawing_area_cpu2_percent", &builder),
        get_widget("drawing_area_cpu3_percent", &builder),
        get_widget("drawing_area_cpu4_percent", &builder),
        get_widget("drawing_area_cpu5_percent", &builder),
        get_widget("drawing_area_cpu6_percent", &builder),
        get_widget("drawing_area_cpu7_percent", &builder),
    ];

    let mut sys = System::new_with_specifics(RefreshKind::new());
    update(&mut sys, &cpu_percent_labels, &cpu_percent_bars);

    glib::timeout_add_seconds_local(CPU_UPDATE_INTERVAL, move || {
        update(&mut sys, &cpu_percent_labels, &cpu_percent_bars);

        return glib::Continue(true);
    });
}

fn update(
    sys: &mut System,
    cpu_percent_labels: &[gtk::Label; 8],
    cpu_percent_bars: &[gtk::DrawingArea; 8],
) {
    sys.refresh_cpu();

    let processors = sys.processors();

    for (i, processor) in processors.into_iter().enumerate() {
        let percent = processor.cpu_usage() as f64;

        cpu_percent_labels[i].set_text(format!("{:.1}%", percent).as_str());

        // draw_bar(&cpu_percent_bars[i], 120, 6, percent / 100.0);
    }
}
