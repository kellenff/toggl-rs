use ansi_term::Color::{Green, Red};
use chrono;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs;
use std::rc::Rc;
use toggl_rs::{TimeEntry, Toggl, TogglExt};

fn print_projects(ids: &[String]) {
    print!("Projects: ");
    for (i, name) in ids.iter().enumerate() {
        print!("{}: {}, ", i, name);
    }
    println!();
}

fn format_duration(c: &chrono::Duration) -> String {
    let mins = c.num_minutes() % 60;
    let hours = c.num_hours();

    let mut st = String::new();
    if hours > 0 {
        st.push_str(&format!("{:2}:", hours));
    }
    if hours == 0 {
        st.push_str(&format!("{} m", mins));
    } else if (hours > 0) | (mins > 0) {
        st.push_str(&format!("{:02} h", mins));
    }
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
            current.description.unwrap_or_else(|| "".to_string()),
            current
                .project
                .as_ref()
                .map(|v| v.name.as_ref())
                .unwrap_or(""),
            current.start.with_timezone(&chrono::Local).format("%H:%M"),
            format_duration(&running_for)
        );
    } else {
        println!("{}", Red.paint("Not Running"));
    }
}

fn get_todays_stored_entries(t: &Toggl) -> Vec<TimeEntry> {
    let start_date = chrono::Utc::today().and_hms(0, 0, 0);
    let mut entries = t
        .get_time_entries_range(Some(start_date), None)
        .expect("API Error");
    if t.get_running_entry().unwrap_or(None).is_some() {
        entries.truncate(entries.len() - 1); //the last one is the currently running one which we handle separately
    }
    entries
}

fn print_todays_timeentries(t: &Toggl) {
    println!(
        "+-----------------------------------------------------------------------------------+"
    );
    let entries = get_todays_stored_entries(t);
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
            "|{:2} | {} | {} | {:<30} | {:^15} | {:>10} |",
            idx + 1,
            start_format,
            stop_format,
            i.description.as_ref().unwrap_or(&"".to_string()),
            i.project.as_ref().map(|v| v.name.as_ref()).unwrap_or(""),
            dur_format,
        );
    }
    println!(
        "+-----------------------------------------------------------------------------------+"
    );

    //print stats
    let sum = chrono::Duration::seconds(
        entries
            .iter()
            .map(|t| t.stop.unwrap() - t.start)
            .map(|t| t.num_seconds())
            .sum::<i64>(),
    );
    let dummy_project = Rc::new(toggl_rs::project::Project {
        id: -1,
        name: "No Project".to_string(),
        billable: false,
        active: false,
    });
    let project_nums = t
        .projects
        .iter()
        .map(|project| {
            (
                project.name.clone(),
                entries
                    .iter()
                    .filter(|time_entry| {
                        time_entry.project.as_ref().unwrap_or(&dummy_project) == project
                    })
                    .map(|time_entry| time_entry.stop.unwrap() - time_entry.start)
                    .map(|time_entry| time_entry.num_seconds())
                    .sum::<i64>(),
            )
        })
        .collect::<Vec<(String, i64)>>();

    for (name, seconds) in project_nums {
        print!(
            "| {}: {} ({:.2}%) ",
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

fn run_matches_time_entry(
    matches: &ArgMatches,
    t: &Toggl,
    projects: &toggl_rs::project::Projects,
) -> Result<(), String> {
    if let Some(mut v) = matches.values_of("start") {
        let title = v.next().map(|v| v.to_owned());
        let project_idx = v.next().and_then(|s| s.parse::<usize>().ok());
        let project = project_idx.and_then(|v| projects.get(v));

        t.start_entry(title.to_owned(), &[], project)
            .expect("Error");
        println!(
            "Started Time Entry: {} for Project {}",
            title.unwrap_or_else(|| "".to_string()),
            project.as_ref().map(|v| v.name.as_ref()).unwrap_or("")
        );
        Ok(())
    } else if matches.is_present("stop") {
        let res = t.get_running_entry().expect("API Error");
        if let Some(current_entry) = res {
            t.stop_entry(&current_entry).expect("Error");
            Ok(())
        } else {
            Err("No time entry currently running".into())
        }
    } else if matches.is_present("swap") {
        let mut entries = get_todays_stored_entries(t);
        if entries.is_empty() {
            return Err("Not enough entries stored to swap".into());
        }

        entries.sort_by(|a, b| b.cmp(a)); //reverse it
        t.start_entry(
            entries[0].description.to_owned(),
            &[],
            entries[0].project.clone(),
        )
        .expect("API Error");
        Ok(())
    } else if let Some(id_string) = matches.value_of("delete") {
        let entries = get_todays_stored_entries(t);
        let id = id_string.parse::<usize>();
        if let Ok(id) = id {
            println!("len, id {} {}", entries.len(), id);
            if id - 1 < entries.len() {
                t.delete_entry(&entries[id - 1]).expect("API Error");
                Ok(())
            } else {
                Err("You tried to delete and entry that does not exist".into())
            }
        } else {
            Err("Could not parse id".into())
        }
    } else if let Some(mut new) = matches.values_of("edit") {
        let id: Option<usize> = new.next().and_then(|s| s.parse::<usize>().ok());
        let new_description = new.next();
        let new_project = new.next().and_then(|s| s.parse::<usize>().ok());

        let entries = get_todays_stored_entries(t);
        if entries.is_empty() {
            Err("Not enough entries stored to edit".into())
        } else if let Some(id) = id {
            if (id - 1 < entries.len())
                & (new_description.is_some())
                & (new_project.is_some())
                & (new_project.unwrap_or(0) < projects.len())
            {
                let project = projects[new_project.unwrap()].clone();

                let mut entry = entries[id - 1].clone();
                entry.description = new_description.map(|v| v.to_string());
                entry.project = Some(project);
                t.update_entry(entry).expect("API Error");
                Ok(())
            } else {
                Err("Argument requirement not fulfilled".into())
            }
        } else {
            Err("Could not parse values".into())
        }
    } else {
        // nothing was parsed which is fine
        Ok(())
    }
}

fn run_matches(
    matches: ArgMatches,
    t: &Toggl,
    projects: &toggl_rs::project::Projects,
) -> Result<(), String> {
    if let Some(matches) = matches.subcommand_matches("time_entry") {
        run_matches_time_entry(matches, t, projects)
    } else {
        Ok(())
    }
}

fn main() {
    let credentials = fs::read_to_string("api_token")
        .expect("Please supply a file called api_token with your api_token");
    let toggl = Toggl::init(&credentials).expect("Could not connect to toggl");
    let projects = &toggl.projects;
    let project_ids = projects
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<String>>();

    let te_match = ["time_entry", "te"].iter().map(|v|
        SubCommand::with_name(v)
        .about("Modifies Time Entries")
        .arg(
            Arg::with_name("start")
                .short("s")
                .long("start")
                .help("Starts a time entry with the appropriate id (optional values description, projectid)")
                .min_values(0)
                .max_values(2)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("stop")
                .short("p")
                .long("stop")
                .help("Stops the current time entry"),
        )
        .arg(Arg::with_name("swap").short("w").long("swap").help(
            "Stops the current entry and starts the entry that was running before the current one",
        ))
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .number_of_values(1)
                .value_names(&["project_id"])
                .takes_value(true)
                .help("Deletes the entry with the idea from the current day"),
        )
        .arg(
            Arg::with_name("edit")
                .short("e")
                .long("edit")
                .number_of_values(3)
                .value_names(&["timeentry_number", "new_description", "new_project_id"])
                .help("Edits the entry given by the first "),
        ));

    let matches = App::new("toggl")
        .about("Controls toggl")
        .subcommands(te_match)
        .get_matches();

    print_projects(&project_ids);
    if let Err(s) = run_matches(matches, &toggl, projects) {
        println!("Error in executing: {}", s);
    } else {
        print_current(&toggl);
        print_todays_timeentries(&toggl);
    }
}
