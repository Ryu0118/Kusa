use ::reqwest::blocking::Client;
use ansi_term::{Colour, Style};
use anyhow::{Context, Result};
use clap::Parser;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use std::process;

//////////////////////////////////////////////////////////
static GITHUB_ACCESS_TOKEN : &str = "GITHUB_ACCESS_TOKEN";
//////////////////////////////////////////////////////////

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug"
)]

struct Kusa;

type Date = String;

#[derive(Parser)]
#[clap(
    name = "kusa",
    version = "0.0.2",
    about = "Command to display Github Contributions graph on your shell"
)]

struct Command {
    #[clap(name = "github user name", action = clap::ArgAction::Set)]
    user_name: String,
}

struct DailyStatus {
    date: String,
    color: String,
}

impl DailyStatus {
    fn get_month(&self) -> usize {
        self.date
            .split('-')
            .nth(1)
            .and_then(|m| m.parse().ok())
            .unwrap()
    }
}

trait HexToRGB {
    fn get_rgb(&self) -> Colour;
}

impl HexToRGB for str {
    fn get_rgb(&self) -> Colour {
        // #ebedf0 -> ebedf0
        let v = i64::from_str_radix(&self[1..], 16).unwrap() as f64;
        let r: u8 = (v / 256_f64.powf(2.0) % 256.0) as u8;
        let g: u8 = (v / 256_f64.powf(1.0) % 256.0) as u8;
        let b: u8 = (v / 256_f64.powf(0.0) % 256.0) as u8;
        Colour::RGB(r, g, b)
    }
}

fn get_github_contributions(response_data: kusa::ResponseData) -> (i64, Vec<Vec<DailyStatus>>) {
    match response_data.user {
        Some(user) => {
            let contribution_calendar = user.contributions_collection.contribution_calendar;

            let total_contributions = contribution_calendar.total_contributions;

            let weekly_status = contribution_calendar
                .weeks
                .iter()
                .map(|weekly_status| {
                    weekly_status
                        .contribution_days
                        .iter()
                        .map(|daily_status| DailyStatus {
                            date: daily_status.date.to_string(),
                            color: daily_status.color.to_string(),
                        })
                        .collect()
                })
                .collect();
            (total_contributions, weekly_status)
        }
        None => {
            println!("No users found");
            process::exit(1)
        }
    }
}

fn post_graphql_query(user_name: String) -> Result<kusa::ResponseData> {

    let variables = kusa::Variables { user_name };

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", GITHUB_ACCESS_TOKEN))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()?;

    let response_body =
        post_graphql::<Kusa, _>(&client, "https://api.github.com/graphql", variables)?;

    response_body.data.context("failed to fetch data")
}

fn transpose(weekly_statuses: &[Vec<DailyStatus>]) -> Vec<Vec<&DailyStatus>> {
    let week_count = weekly_statuses.len();
    let mut kusa: Vec<Vec<&DailyStatus>> = Vec::new();
    for column_index in 0..7 {
        let mut row = Vec::new();
        for weekly_status in weekly_statuses.iter().take(week_count) {
            if let Some(contribution) = weekly_status.get(column_index) {
                row.push(contribution);
            }
        }
        kusa.push(row);
    }
    kusa
}

#[cfg(not(target_os = "windows"))]
fn print_month(kusa: &[Vec<&DailyStatus>]) {
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let mut month_line = "".to_string();
    for (index, daily_status) in kusa[0].iter().enumerate() {
        if index == 0 {
            let month = daily_status.get_month();
            month_line += months[month - 1];
            continue;
        }

        let month = daily_status.get_month();
        let previous_month = kusa[0][index - 1].get_month();

        if month != previous_month {
            let require_width = index * 2;
            let current_width = month_line.len();
            let require_space = (require_width as i64) - (current_width as i64);
            if require_space > 0 {
                let adjustment = " ".repeat(require_space as usize);
                month_line += &adjustment;
                month_line += months[month - 1];
            } else {
                let adjustment_space = 3 + require_space;
                month_line.clear();
                month_line += &" ".repeat(adjustment_space as usize);
                month_line += months[month - 1];
            }
        }
    }
    println!("{}", month_line);
}

#[cfg(not(target_os = "windows"))]
fn print_gradation(kusa: &[Vec<&DailyStatus>]) {
    let start_point = (kusa[6].len()) * 2 - 18;
    let colors = [
        "#ebedf0", //Less
        "#9be9a8", "#40c463", "#30a14e", "#216e39", //More
    ];
    let whitespaces = " ".repeat(start_point);
    print!("{}Less ", whitespaces);
    for color in colors {
        color
            .get_rgb()
            .paint("■ ".as_bytes())
            .write_to(&mut std::io::stdout())
            .unwrap();
    }
    println!("More");
}

fn print_kusa(kusa: &Vec<Vec<&DailyStatus>>) {
    for weekly_kusa in kusa {
        for daily_kusa in weekly_kusa {
            daily_kusa
                .color
                .get_rgb()
                .paint("■ ".as_bytes())
                .write_to(&mut std::io::stdout())
                .unwrap();
        }
        println!();
    }
}

fn main() -> Result<()> {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support();

    let args = Command::parse();

    let data = post_graphql_query(args.user_name)?;
    let (total_contributions, weekly_statuses) = get_github_contributions(data);
    let kusa = transpose(&weekly_statuses);

    println!(
        "{} contributions in the last year",
        Style::new().bold().paint(total_contributions.to_string())
    );

    #[cfg(not(target_os = "windows"))]
    print_month(&kusa);

    print_kusa(&kusa);

    #[cfg(not(target_os = "windows"))]
    print_gradation(&kusa);

    Ok(())
}
