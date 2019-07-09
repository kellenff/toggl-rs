use ansi_term;
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
    } else if matches.value_of("stop").is_some() {
        let current_entry = toggl.get_running_entry().expect("Error");
        toggl.stop_entry(&current_entry).expect("Error");
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
                .short("t")
                .long("stop")
                .help("Stops the current task"),
        )
        .get_matches();

    print_projects(&project_ids);


    run_matches(matches, &toggl, projects);
}
