use gdk::glib::{self, Receiver, Sender};
use gtk::prelude::*;
use sysinfo::{CpuExt, RefreshKind, System, SystemExt};

use crate::custom_components::bar::build_bar;
use crate::custom_components::graph::build_graph;
use crate::util::get_widget;

const UPDATE_INTERVAL_CPU: u32 = 1;

pub fn setup(builder: &gtk::Builder) {
    let mut sys = System::new_with_specifics(RefreshKind::new());
    let label_cpu_percent = get_widget("label_cpu_percent", &builder);

    let (cpu_bar_tx, cpu_bar_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu0_bar_tx, cpu0_bar_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu1_bar_tx, cpu1_bar_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu2_bar_tx, cpu2_bar_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu3_bar_tx, cpu3_bar_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu4_bar_tx, cpu4_bar_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu5_bar_tx, cpu5_bar_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu6_bar_tx, cpu6_bar_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu7_bar_tx, cpu7_bar_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let cpu_percent_tx_channels: [Sender<f64>; 8] = [
        cpu0_bar_tx,
        cpu1_bar_tx,
        cpu2_bar_tx,
        cpu3_bar_tx,
        cpu4_bar_tx,
        cpu5_bar_tx,
        cpu6_bar_tx,
        cpu7_bar_tx,
    ];

    let (cpu_graph_tx, cpu_graph_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu0_graph_tx, cpu0_graph_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu1_graph_tx, cpu1_graph_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu2_graph_tx, cpu2_graph_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu3_graph_tx, cpu3_graph_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu4_graph_tx, cpu4_graph_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu5_graph_tx, cpu5_graph_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu6_graph_tx, cpu6_graph_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (cpu7_graph_tx, cpu7_graph_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let cpu_graph_tx_channels: [Sender<u64>; 8] = [
        cpu0_graph_tx,
        cpu1_graph_tx,
        cpu2_graph_tx,
        cpu3_graph_tx,
        cpu4_graph_tx,
        cpu5_graph_tx,
        cpu6_graph_tx,
        cpu7_graph_tx,
    ];

    build_bar(
        get_widget("drawing_area_cpu_percent_bar", &builder),
        500,
        6,
        cpu_bar_rx,
    );
    build_graph(
        get_widget("cpu_graph", &builder),
        500,
        100,
        cpu_graph_rx,
        Some(100),
    );

    build_cpu_core_bars(
        &builder,
        cpu0_bar_rx,
        cpu1_bar_rx,
        cpu2_bar_rx,
        cpu3_bar_rx,
        cpu4_bar_rx,
        cpu5_bar_rx,
        cpu6_bar_rx,
        cpu7_bar_rx,
    );
    build_cpu_core_graphs(
        &builder,
        cpu0_graph_rx,
        cpu1_graph_rx,
        cpu2_graph_rx,
        cpu3_graph_rx,
        cpu4_graph_rx,
        cpu5_graph_rx,
        cpu6_graph_rx,
        cpu7_graph_rx,
    );

    update(
        &mut sys,
        &label_cpu_percent,
        &cpu_bar_tx,
        &cpu_graph_tx,
        &cpu_percent_tx_channels,
        &cpu_graph_tx_channels,
    );
    glib::timeout_add_seconds_local(UPDATE_INTERVAL_CPU, move || {
        update(
            &mut sys,
            &label_cpu_percent,
            &cpu_bar_tx,
            &cpu_graph_tx,
            &cpu_percent_tx_channels,
            &cpu_graph_tx_channels,
        );

        return glib::Continue(true);
    });
}

fn update(
    sys: &mut System,
    label_cpu_percent: &gtk::Label,
    cpu_bar_tx: &Sender<f64>,
    cpu_graph_tx: &Sender<u64>,
    cpu_percent_tx_channels: &[Sender<f64>; 8],
    cpu_graph_tx_channels: &[Sender<u64>; 8],
) {
    sys.refresh_cpu();

    let mut total_percent = 0.0;

    for (i, cpu) in sys.cpus().into_iter().enumerate() {
        let usage = cpu.cpu_usage();
        total_percent += usage;
        cpu_percent_tx_channels[i]
            .send(usage as f64 / 100.0)
            .unwrap();
        cpu_graph_tx_channels[i].send(usage as u64).unwrap();
    }

    cpu_bar_tx.send(total_percent as f64 / 800.0).unwrap();
    cpu_graph_tx.send(total_percent as u64 / 8).unwrap();
    label_cpu_percent.set_text(&format!("{:.1}%", total_percent / 8.0));
}

fn build_cpu_core_bars(
    builder: &gtk::Builder,
    cpu0_bar_rx: Receiver<f64>,
    cpu1_bar_rx: Receiver<f64>,
    cpu2_bar_rx: Receiver<f64>,
    cpu3_bar_rx: Receiver<f64>,
    cpu4_bar_rx: Receiver<f64>,
    cpu5_bar_rx: Receiver<f64>,
    cpu6_bar_rx: Receiver<f64>,
    cpu7_bar_rx: Receiver<f64>,
) {
    const BAR_WIDTH: i32 = 120;
    const BAR_HEIGHT: i32 = 6;

    build_bar(
        get_widget("cpu0_bar", builder),
        BAR_WIDTH,
        BAR_HEIGHT,
        cpu0_bar_rx,
    );

    build_bar(
        get_widget("cpu1_bar", builder),
        BAR_WIDTH,
        BAR_HEIGHT,
        cpu1_bar_rx,
    );

    build_bar(
        get_widget("cpu2_bar", builder),
        BAR_WIDTH,
        BAR_HEIGHT,
        cpu2_bar_rx,
    );

    build_bar(
        get_widget("cpu3_bar", builder),
        BAR_WIDTH,
        BAR_HEIGHT,
        cpu3_bar_rx,
    );

    build_bar(
        get_widget("cpu4_bar", builder),
        BAR_WIDTH,
        BAR_HEIGHT,
        cpu4_bar_rx,
    );

    build_bar(
        get_widget("cpu5_bar", builder),
        BAR_WIDTH,
        BAR_HEIGHT,
        cpu5_bar_rx,
    );

    build_bar(
        get_widget("cpu6_bar", builder),
        BAR_WIDTH,
        BAR_HEIGHT,
        cpu6_bar_rx,
    );

    build_bar(
        get_widget("cpu7_bar", builder),
        BAR_WIDTH,
        BAR_HEIGHT,
        cpu7_bar_rx,
    );
}

fn build_cpu_core_graphs(
    builder: &gtk::Builder,
    cpu0_graph_rx: Receiver<u64>,
    cpu1_graph_rx: Receiver<u64>,
    cpu2_graph_rx: Receiver<u64>,
    cpu3_graph_rx: Receiver<u64>,
    cpu4_graph_rx: Receiver<u64>,
    cpu5_graph_rx: Receiver<u64>,
    cpu6_graph_rx: Receiver<u64>,
    cpu7_graph_rx: Receiver<u64>,
) {
    const GRAPH_WIDTH: i32 = 120;
    const GRAPH_HEIGHT: i32 = 50;

    build_graph(
        get_widget("cpu0_graph", builder),
        GRAPH_WIDTH,
        GRAPH_HEIGHT,
        cpu0_graph_rx,
        Some(100),
    );

    build_graph(
        get_widget("cpu1_graph", builder),
        GRAPH_WIDTH,
        GRAPH_HEIGHT,
        cpu1_graph_rx,
        Some(100),
    );

    build_graph(
        get_widget("cpu2_graph", builder),
        GRAPH_WIDTH,
        GRAPH_HEIGHT,
        cpu2_graph_rx,
        Some(100),
    );

    build_graph(
        get_widget("cpu3_graph", builder),
        GRAPH_WIDTH,
        GRAPH_HEIGHT,
        cpu3_graph_rx,
        Some(100),
    );

    build_graph(
        get_widget("cpu4_graph", builder),
        GRAPH_WIDTH,
        GRAPH_HEIGHT,
        cpu4_graph_rx,
        Some(100),
    );

    build_graph(
        get_widget("cpu5_graph", builder),
        GRAPH_WIDTH,
        GRAPH_HEIGHT,
        cpu5_graph_rx,
        Some(100),
    );

    build_graph(
        get_widget("cpu6_graph", builder),
        GRAPH_WIDTH,
        GRAPH_HEIGHT,
        cpu6_graph_rx,
        Some(100),
    );

    build_graph(
        get_widget("cpu7_graph", builder),
        GRAPH_WIDTH,
        GRAPH_HEIGHT,
        cpu7_graph_rx,
        Some(100),
    );
}
