use gdk::glib;
use gtk::prelude::LabelExt;
use sysinfo::{ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

use crate::util::get_widget;

const PROCESSES_UPDATE_INTERVAL: u32 = 1;

pub fn setup(builder: &gtk::Builder) {
    let process_percent1 = get_widget::<gtk::Label>("process_percent1", &builder);
    let process_percent2 = get_widget::<gtk::Label>("process_percent2", &builder);
    let process_percent3 = get_widget::<gtk::Label>("process_percent3", &builder);

    let label_pid1 = get_widget::<gtk::Label>("pid1", &builder);
    let label_pid2 = get_widget::<gtk::Label>("pid2", &builder);
    let label_pid3 = get_widget::<gtk::Label>("pid3", &builder);

    let process_name1 = get_widget::<gtk::Label>("process_name1", &builder);
    let process_name2 = get_widget::<gtk::Label>("process_name2", &builder);
    let process_name3 = get_widget::<gtk::Label>("process_name3", &builder);

    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::new().with_cpu()),
    );

    update(
        &mut sys,
        &process_percent1,
        &process_percent2,
        &process_percent3,
        &label_pid1,
        &label_pid2,
        &label_pid3,
        &process_name1,
        &process_name2,
        &process_name3,
    );
    glib::timeout_add_seconds_local(PROCESSES_UPDATE_INTERVAL, move || {
        update(
            &mut sys,
            &process_percent1,
            &process_percent2,
            &process_percent3,
            &label_pid1,
            &label_pid2,
            &label_pid3,
            &process_name1,
            &process_name2,
            &process_name3,
        );

        return glib::Continue(true);
    });
}

fn update(
    sys: &mut System,

    process_percent1: &gtk::Label,
    process_percent2: &gtk::Label,
    process_percent3: &gtk::Label,

    label_pid1: &gtk::Label,
    label_pid2: &gtk::Label,
    label_pid3: &gtk::Label,

    process_name1: &gtk::Label,
    process_name2: &gtk::Label,
    process_name3: &gtk::Label,
) {
    sys.refresh_processes_specifics(ProcessRefreshKind::new().with_cpu());

    let mut processes: Vec<Process> = vec![];

    for (pid, process) in sys.processes() {
        processes.push(Process {
            pid: i32::from(*pid),
            cpu_usage: process.cpu_usage(),
            name: String::from(process.name()),
        });
    }

    processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());

    process_percent1.set_text(format!("{:.1}%", processes[0].cpu_usage).as_str());
    process_percent2.set_text(format!("{:.1}%", processes[1].cpu_usage).as_str());
    process_percent3.set_text(format!("{:.1}%", processes[2].cpu_usage).as_str());

    label_pid1.set_text(format!("{}", processes[0].pid).as_str());
    label_pid2.set_text(format!("{}", processes[1].pid).as_str());
    label_pid3.set_text(format!("{}", processes[3].pid).as_str());

    process_name1.set_text(processes[0].name.as_str());
    process_name2.set_text(processes[1].name.as_str());
    process_name3.set_text(processes[2].name.as_str());
}

#[derive(Debug)]
struct Process {
    pub cpu_usage: f32,
    pub pid: i32,
    pub name: String,
}
