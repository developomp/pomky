use gdk::glib::{self, Sender};
use gtk::prelude::*;
use gtk::Builder;
use sysinfo::{RefreshKind, System, SystemExt};

use crate::config;
use crate::custom_components::bar::build_bar;
use crate::custom_components::graph::build_graph;
use crate::util::{get_widget, kib_2_gb};

pub fn setup(builder: &Builder) {
    let label_memory_used = get_widget("label_memory_used", &builder);
    let label_memory_total = get_widget("label_memory_total", &builder);
    let label_memory_free = get_widget("label_memory_free", &builder);
    let label_memory_percent = get_widget("label_memory_percent", &builder);

    let mut sys = System::new_with_specifics(RefreshKind::new().with_memory());

    let (bar_tx, bar_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let (graph_tx, graph_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    build_bar(get_widget("memory_usage_bar", &builder), 500, 6, bar_rx);
    build_graph(
        get_widget("memory_graph", &builder),
        500,
        50,
        graph_rx,
        Some(100),
    );

    update(
        &mut sys,
        &label_memory_used,
        &label_memory_total,
        &label_memory_free,
        &label_memory_percent,
        &bar_tx,
        &graph_tx,
    );
    glib::timeout_add_seconds_local(config::UPDATE_INTERVAL_MEMORY, move || {
        update(
            &mut sys,
            &label_memory_used,
            &label_memory_total,
            &label_memory_free,
            &label_memory_percent,
            &bar_tx,
            &graph_tx,
        );

        return glib::Continue(true);
    });
}

fn update(
    sys: &mut System,
    label_memory_used: &gtk::Label,
    label_memory_total: &gtk::Label,
    label_memory_free: &gtk::Label,
    label_memory_percent: &gtk::Label,
    bar_tx: &Sender<f64>,
    graph_tx: &Sender<u64>,
) {
    sys.refresh_memory();

    let mem_used = sys.used_memory();
    let mem_total = sys.total_memory();
    let ratio = mem_used as f64 / mem_total as f64;

    label_memory_used.set_text(&format!("{:.1} GB", kib_2_gb(mem_used)));
    label_memory_total.set_text(&format!("{:.1} GB", kib_2_gb(mem_total)));
    label_memory_free.set_text(&format!("{:.1} GB", kib_2_gb(sys.free_memory())));
    label_memory_percent.set_text(&format!("{:.1}%", 100.0 * ratio));

    bar_tx.send(ratio).unwrap();
    graph_tx.send((100.0 * ratio) as u64).unwrap();
}
