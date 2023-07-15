use gdk::glib;
use gtk::prelude::*;
use sysinfo::{Pid, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

use crate::util::get_widget;

const UPDATE_INTERVAL: u32 = 1;
const NUM_LABELS: usize = 3;

pub fn setup(builder: &gtk::Builder) {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::new().with_cpu()),
    );

    let process_percents: [gtk::Label; NUM_LABELS] = [
        get_widget("process_percent1", &builder),
        get_widget("process_percent2", &builder),
        get_widget("process_percent3", &builder),
    ];
    let pids: [gtk::Label; NUM_LABELS] = [
        get_widget("pid1", &builder),
        get_widget("pid2", &builder),
        get_widget("pid3", &builder),
    ];
    let process_names: [gtk::Label; NUM_LABELS] = [
        get_widget("process_name1", &builder),
        get_widget("process_name2", &builder),
        get_widget("process_name3", &builder),
    ];

    update(&mut sys, &process_percents, &pids, &process_names);
    glib::timeout_add_seconds_local(UPDATE_INTERVAL, move || {
        update(&mut sys, &process_percents, &pids, &process_names);
        return glib::Continue(true);
    });
}

fn update(
    sys: &mut System,
    process_percents: &[gtk::Label; 3],
    pids: &[gtk::Label; 3],
    process_names: &[gtk::Label; 3],
) {
    sys.refresh_processes_specifics(ProcessRefreshKind::new().with_cpu());

    let mut processes: Vec<Process> = vec![];

    for (pid, process) in sys.processes() {
        processes.push(Process {
            pid: *pid,
            cpu_usage: process.cpu_usage(),
            name: String::from(process.name()),
        });
    }

    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());

    for i in 0..NUM_LABELS {
        process_percents[i].set_text(format!("{:.1}%", processes[i].cpu_usage).as_str());
        pids[i].set_text(format!("{}", processes[i].pid).as_str());
        process_names[i].set_text(processes[i].name.as_str());
    }
}

#[derive(Debug)]
struct Process {
    pub cpu_usage: f32,
    pub pid: Pid,
    pub name: String,
}
