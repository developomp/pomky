use gdk::glib::{self, Receiver, Sender};
use gtk::prelude::LabelExt;
use sysinfo::{ProcessorExt, RefreshKind, System, SystemExt};

use crate::custom_components::bar::build_bar;
use crate::util::{get_widget, msec_2_duration};

const CPU_UPDATE_INTERVAL: u32 = 1000;

pub fn setup(builder: &gtk::Builder) {
    let mut sys = System::new_with_specifics(RefreshKind::new());
    let label_cpu_percent = get_widget("label_cpu_percent", &builder);

    let (cpu_percent_tx, cpu_percent_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let (cpu0_percent_tx, cpu0_percent_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu1_percent_tx, cpu1_percent_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu2_percent_tx, cpu2_percent_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu3_percent_tx, cpu3_percent_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu4_percent_tx, cpu4_percent_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu5_percent_tx, cpu5_percent_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu6_percent_tx, cpu6_percent_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu7_percent_tx, cpu7_percent_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let cpu_percent_tx_channels: [Sender<f64>; 8] = [
        cpu0_percent_tx,
        cpu1_percent_tx,
        cpu2_percent_tx,
        cpu3_percent_tx,
        cpu4_percent_tx,
        cpu5_percent_tx,
        cpu6_percent_tx,
        cpu7_percent_tx,
    ];

    build_bar(
        get_widget("drawing_area_cpu_percent_bar", &builder),
        500,
        6,
        cpu_percent_rx,
    );

    build_cpu_core_bars(
        &builder,
        cpu0_percent_rx,
        cpu1_percent_rx,
        cpu2_percent_rx,
        cpu3_percent_rx,
        cpu4_percent_rx,
        cpu5_percent_rx,
        cpu6_percent_rx,
        cpu7_percent_rx,
    );

    update(
        &mut sys,
        &label_cpu_percent,
        &cpu_percent_tx,
        &cpu_percent_tx_channels,
    );
    glib::timeout_add_local(msec_2_duration(CPU_UPDATE_INTERVAL), move || {
        update(
            &mut sys,
            &label_cpu_percent,
            &cpu_percent_tx,
            &cpu_percent_tx_channels,
        );

        return glib::Continue(true);
    });
}

fn update(
    sys: &mut System,
    label_cpu_percent: &gtk::Label,
    cpu_percent_tx: &Sender<f64>,
    cpu_percent_tx_channels: &[Sender<f64>; 8],
) {
    sys.refresh_cpu();

    let mut total_percent = 0.0;

    for (i, processor) in sys.processors().into_iter().enumerate() {
        let usage = processor.cpu_usage();
        total_percent += usage;
        cpu_percent_tx_channels[i]
            .send(usage as f64 / 100.0)
            .unwrap();
    }

    cpu_percent_tx.send(total_percent as f64 / 800.0).unwrap();
    label_cpu_percent.set_text(&format!("{:.1}%", total_percent / 8.0));
}

fn build_cpu_core_bars(
    builder: &gtk::Builder,
    cpu0_percent_rx: Receiver<f64>,
    cpu1_percent_rx: Receiver<f64>,
    cpu2_percent_rx: Receiver<f64>,
    cpu3_percent_rx: Receiver<f64>,
    cpu4_percent_rx: Receiver<f64>,
    cpu5_percent_rx: Receiver<f64>,
    cpu6_percent_rx: Receiver<f64>,
    cpu7_percent_rx: Receiver<f64>,
) {
    build_bar(
        get_widget("drawing_area_cpu0_percent", builder),
        120,
        6,
        cpu0_percent_rx,
    );

    build_bar(
        get_widget("drawing_area_cpu1_percent", builder),
        120,
        6,
        cpu1_percent_rx,
    );

    build_bar(
        get_widget("drawing_area_cpu2_percent", builder),
        120,
        6,
        cpu2_percent_rx,
    );

    build_bar(
        get_widget("drawing_area_cpu3_percent", builder),
        120,
        6,
        cpu3_percent_rx,
    );

    build_bar(
        get_widget("drawing_area_cpu4_percent", builder),
        120,
        6,
        cpu4_percent_rx,
    );

    build_bar(
        get_widget("drawing_area_cpu5_percent", builder),
        120,
        6,
        cpu5_percent_rx,
    );

    build_bar(
        get_widget("drawing_area_cpu6_percent", builder),
        120,
        6,
        cpu6_percent_rx,
    );

    build_bar(
        get_widget("drawing_area_cpu7_percent", builder),
        120,
        6,
        cpu7_percent_rx,
    );
}
