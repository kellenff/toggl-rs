use ansi_term::Color::{Green, Red};
use chrono;
use clap::{App, Arg, ArgMatches};
use toggl_rs::project::ProjectTrait;
use toggl_rs::time_entry::TimeEntryExt;
use toggl_rs::{init, Toggl};

fn print_projects(ids: &[String]) {
    print!("Projects: ");
    for (i, name) in ids.iter().enumerate() {
        print!("{}: {}, ", i, name);
    }
    println!();
}

fn format_duration(c: &chrono::Duration) -> String {
    let secs = c.num_seconds() % 60;
    let mins = c.num_minutes() % 60;
    let hours = c.num_hours();

    let mut st = String::new();
    if hours > 0 {
        st.push_str(&format!("{:2}:", hours));
    }
    if (hours > 0) | (mins > 0) {
        st.push_str(&format!("{:02}:", mins));
    }
    st.push_str(&format!("{:02}", secs));
    st
}

fn print_current(t: &Toggl) {
    print!("Current: ");
    let res = t.get_running_entry().expect("API Problem");
    if let Some(current) = res {
        let running_for = chrono::Utc::now() - current.start;
        println!(
            "{}: {}@{}, {} Running for: {}",
            Green.paint("Running"),
            current.description,
            current.project.name,
            current.start.with_timezone(&chrono::Local).format("%H:%M"),
            format_duration(&running_for)
        );
    } else {
        println!("{}", Red.paint("Not Running"));
    }
}

fn print_todays_tasks(t: &Toggl) {
    let start_date = chrono::Utc::today().and_hms(0, 0, 0);
    println!("+----------------------------------------------------------------------------------+");
    let mut entries = t
        .get_time_entries_range(Some(start_date), None)
        .expect("API Error");
    if t.get_running_entry().unwrap_or(None).is_some() {
        entries.truncate(entries.len() - 1); //the last one is the currently running one which we handle separately
    }
    for (idx, i) in entries.iter().enumerate() {
        let start_format = i.start.with_timezone(&chrono::Local).format("%H:%M");
        let stop_format = i
            .stop
            .unwrap()
            .with_timezone(&chrono::Local)
            .format("%H:%M");
        let duration = i.stop.unwrap() - i.start;
        let dur_format = format_duration(&duration);
        println!(
            "|{} | {} | {} | {:<30} | {:^15} | {:>10} |",
            idx, start_format, stop_format, i.description, i.project.name, dur_format
        );
    }
    println!("+----------------------------------------------------------------------------------+");

    //print stats
    let sum = chrono::Duration::seconds(
        entries
            .iter()
            .map(|t| t.stop.unwrap() - t.start)
            .map(|t| t.num_seconds())
            .sum::<i64>(),
    );
    let project_nums = t
        .projects
        .as_ref()
        .unwrap_or(&Vec::new())
        .iter()
        .map(|project| {
            (
                project.name.clone(),
                entries
                    .iter()
                    .filter(|task| task.project == *project)
                    .map(|task| task.stop.unwrap() - task.start)
                    .map(|task| task.num_seconds())
                    .sum::<i64>(),
            )
        })
        .collect::<Vec<(String, i64)>>();

    for (name, seconds) in project_nums {
        print!(
            "| {}:{} ({:.2}%) ",
            name,
            format_duration(&chrono::Duration::seconds(seconds)),
            seconds as f64 / sum.num_seconds() as f64
        );
    }

    println!(
        "| Total: {} | Ctx: {}",
        format_duration(&sum),
        std::cmp::max(entries.len() as i64 - 1, 0)
    );
}

fn run_matches(matches: ArgMatches, toggl: &Toggl, projects: &toggl_rs::project::Projects) {
    if let Some(mut v) = matches.values_of("start") {
        let title = v.next().unwrap_or("Default");
        let project_idx = v.next().and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
        let project = projects.get(project_idx);
        if let Some(p) = project {
            toggl.start_entry("test", &[], &p).expect("Error");
            println!("Started Task: {} for Project {}", title, (*p).name);
        } else {
            println!("Project not found");
        }
    } else if matches.is_present("stop") {
        let res = toggl.get_running_entry().expect("API Error");
        if let Some(current_entry) = res {
            toggl.stop_entry(&current_entry).expect("Error");
        } else {
            println!("No task currently running");
        }
    }
}

fn main() {
    let mut toggl = init(include_str!("../api_token")).expect("Could not connect to toggl");
    toggl.fill_projects();
    let projects = toggl.projects.as_ref().unwrap();
    let project_ids = projects
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<String>>();

    let matches = App::new("toggl")
        .about("Controls toggl")
        .arg(
            Arg::with_name("start")
                .short("s")
                .long("start")
                .help("Starts a task with the appropriate id")
                .min_values(1)
                .max_values(2)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("stop")
                .short("p")
                .long("stop")
                .help("Stops the current task"),
        )
        .get_matches();

    print_projects(&project_ids);
    print_current(&toggl);
    print_todays_tasks(&toggl);

    run_matches(matches, &toggl, projects);
}
