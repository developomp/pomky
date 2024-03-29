use gdk::glib::{self, Receiver, Sender};
use gtk::prelude::*;
use sysinfo::{CpuExt, RefreshKind, System, SystemExt};

use crate::custom_components::bar::build_bar;
use crate::custom_components::graph::build_graph;
use crate::util::get_widget;

const NUM_CPU: usize = 8;
const UPDATE_INTERVAL: u32 = 1;

const BAR_WIDTH: i32 = 120;
const BAR_HEIGHT: i32 = 6;

const GRAPH_WIDTH: i32 = 120;
const GRAPH_HEIGHT: i32 = 50;

pub fn setup(builder: &gtk::Builder) {
    let mut sys = System::new_with_specifics(RefreshKind::new());

    let label_percent = get_widget("label_cpu_percent", &builder);

    let (total_bar_tx, total_bar_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (bar_tx_channels, bar_rx) = generate_channels();

    let (total_graph_tx, total_graph_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (graph_tx_channels, graph_rx) = generate_channels();

    build_bar(
        get_widget("drawing_area_cpu_percent_bar", &builder),
        500,
        6,
        total_bar_rx,
    );
    build_graph(
        get_widget("cpu_graph", &builder),
        500,
        100,
        total_graph_rx,
        Some(100),
    );

    build_core_bars(&builder, bar_rx);
    build_core_graphs(&builder, graph_rx);

    update(
        &mut sys,
        &label_percent,
        &total_bar_tx,
        &total_graph_tx,
        &bar_tx_channels,
        &graph_tx_channels,
    );
    glib::timeout_add_seconds_local(UPDATE_INTERVAL, move || {
        update(
            &mut sys,
            &label_percent,
            &total_bar_tx,
            &total_graph_tx,
            &bar_tx_channels,
            &graph_tx_channels,
        );

        return glib::Continue(true);
    });
}

fn update(
    sys: &mut System,
    label_percent: &gtk::Label,
    total_bar_tx: &Sender<f64>,
    total_graph_tx: &Sender<u64>,
    bar_tx_channels: &[Sender<f64>; 8],
    graph_tx_channels: &[Sender<u64>; 8],
) {
    sys.refresh_cpu();

    let mut total_percent = 0.0;

    for (i, cpu) in sys.cpus().into_iter().enumerate() {
        let usage = cpu.cpu_usage();
        total_percent += usage;
        bar_tx_channels[i].send(usage as f64 / 100.0).unwrap();
        graph_tx_channels[i].send(usage as u64).unwrap();
    }

    total_bar_tx.send(total_percent as f64 / 800.0).unwrap();
    total_graph_tx.send(total_percent as u64 / 8).unwrap();
    label_percent.set_text(&format!("{:.1}%", total_percent / 8.0));
}

fn generate_channels<T>() -> ([Sender<T>; NUM_CPU], [Receiver<T>; NUM_CPU]) {
    let (tx0, rx0) = glib::MainContext::channel::<T>(glib::PRIORITY_DEFAULT);
    let (tx1, rx1) = glib::MainContext::channel::<T>(glib::PRIORITY_DEFAULT);
    let (tx2, rx2) = glib::MainContext::channel::<T>(glib::PRIORITY_DEFAULT);
    let (tx3, rx3) = glib::MainContext::channel::<T>(glib::PRIORITY_DEFAULT);
    let (tx4, rx4) = glib::MainContext::channel::<T>(glib::PRIORITY_DEFAULT);
    let (tx5, rx5) = glib::MainContext::channel::<T>(glib::PRIORITY_DEFAULT);
    let (tx6, rx6) = glib::MainContext::channel::<T>(glib::PRIORITY_DEFAULT);
    let (tx7, rx7) = glib::MainContext::channel::<T>(glib::PRIORITY_DEFAULT);

    return (
        [tx0, tx1, tx2, tx3, tx4, tx5, tx6, tx7],
        [rx0, rx1, rx2, rx3, rx4, rx5, rx6, rx7],
    );
}

fn build_core_bars(builder: &gtk::Builder, bar_rx_channels: [Receiver<f64>; NUM_CPU]) {
    let mut i = 0;
    for rx in bar_rx_channels {
        build_bar(
            get_widget(format!("cpu{}_bar", i).as_str(), builder),
            BAR_WIDTH,
            BAR_HEIGHT,
            rx,
        );
        i += 1;
    }
}

fn build_core_graphs(builder: &gtk::Builder, graph_rx_channels: [Receiver<u64>; NUM_CPU]) {
    let mut i = 0;
    for rx in graph_rx_channels {
        build_graph(
            get_widget(format!("cpu{}_graph", i).as_str(), builder),
            GRAPH_WIDTH,
            GRAPH_HEIGHT,
            rx,
            Some(100),
        );
        i += 1;
    }
}
