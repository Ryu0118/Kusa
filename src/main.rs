use ::reqwest::blocking::Client;
use anyhow::{Context, Result};
use clap::Parser;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use std::process;
use ansi_term::{Style, Colour};

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
    version = "0.0.1",
    about = "Command to display Github Contributions graph on your shell"
)]
struct Command {
    #[clap(name = "github user name", action = clap::ArgAction::Set)]
    user_name: String,
}

struct DailyStatus {
    date: String,
    contribution_count: i64,
    color: String,
}

impl DailyStatus {
    fn get_month(&self) -> usize {
        self.date.split("-").collect::<Vec<_>>()[1].parse().unwrap()
    }
}

trait HexToRGB {
    fn get_rgb(&mut self) -> Colour;
}

impl HexToRGB for String {
    fn get_rgb(&mut self) -> Colour {
        self.remove(0); //#ebedf0 -> ebedf0
        let v = i64::from_str_radix(&*self, 16).unwrap() as f64;
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
                            contribution_count: daily_status.contribution_count,
                            color: daily_status.color.to_string(),
                        })
                        .collect()
                })
                .collect();
            return (total_contributions, weekly_status);
        }
        None => {
            println!("No users found");
            process::exit(1)
        }
    }
}

fn post_graphql_query(user_name: &str) -> Result<kusa::ResponseData> {
    let github_access_token = "GITHUB_ACCESS_TOKEN";

    let variables = kusa::Variables {
        user_name: user_name.to_string(),
    };

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", github_access_token))
                    .unwrap(),
            ))
            .collect(),
        )
        .build()?;

    let response_body =
        post_graphql::<Kusa, _>(&client, "https://api.github.com/graphql", variables)?;

    response_body.data.context("failed to fetch data")
}

fn transpose(weekly_statuses: &Vec<Vec<DailyStatus>>) -> Vec<Vec<&DailyStatus>> {
    let week_count = weekly_statuses.len();
    let mut kusa: Vec<Vec<&DailyStatus>> = Vec::new();
    for column_index in 0..7 {
        let mut row = Vec::new();
        for row_index in 0..week_count {
            if let Some(contribution) = weekly_statuses[row_index].get(column_index) {
                row.push(contribution);
            }
        }
        kusa.push(row);
    }
    return kusa;
}

#[cfg(not(target_os = "windows"))]
fn print_month(kusa: &Vec<Vec<&DailyStatus>>) {
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
                month_line = "".to_string();
                month_line += &" ".repeat(adjustment_space as usize);
                month_line += months[month - 1];
            }
        }
    }
    println!("{}", month_line);
}

#[cfg(not(target_os = "windows"))]
fn print_gradation(kusa: &Vec<Vec<&DailyStatus>>) {
    let start_point = (kusa[6].len()) * 2 - 18;
    let colors = [
        "#ebedf0", //Less
        "#9be9a8",
        "#40c463",
        "#30a14e",
        "#216e39", //More
    ];
    let whitespaces = " ".repeat(start_point);
    print!("{}", whitespaces);
    print!("Less ");
    for color in colors {
        color
            .to_string()
            .get_rgb()
            .paint("■ ".as_bytes())
            .write_to(&mut std::io::stdout()).unwrap();
    }
    print!("More\n");
}

fn print_kusa(kusa: &Vec<Vec<&DailyStatus>>) {
    for weekly_kusa in kusa {
        for daily_kusa in weekly_kusa {
            daily_kusa.color
                .to_string()
                .get_rgb()
                .paint("■ ".as_bytes())
                .write_to(&mut std::io::stdout()).unwrap();
        }
        println!("");
    }
}

fn main() -> Result<()> {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support();

    let args = Command::parse();
    let user_name = args.user_name;

    let data = post_graphql_query(&user_name)?;
    let (total_contributions, weekly_statuses) = get_github_contributions(data);
    let kusa = transpose(&weekly_statuses);

    println!("{} contributions in the last year", Style::new().bold().paint(total_contributions.to_string()));

    #[cfg(not(target_os = "windows"))]
    print_month(&kusa);

    print_kusa(&kusa);

    #[cfg(not(target_os = "windows"))]
    print_gradation(&kusa);

    Ok(())
}