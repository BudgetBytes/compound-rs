use rand::Rng;
use std::{
    collections::HashMap,
    io::{stdin, Write},
};

struct Drop {
    percentage: f64,
    count: u64,
}

struct Input {
    monthly_compound: f64,
    yearly_roi: f64,
    deadline: u64,
    drops: Vec<Drop>,
}

struct Stats {
    invested_capital: f64,
    roi: f64,
    pnl: f64,
    avg: f64,
    investments: Vec<f64>,
}

fn calc_stats(input: &Input) -> Stats {
    let monthly_avg = input.yearly_roi / 12.0;
    let months = input.deadline * 12;
    let mut monthly_buy: Vec<f64> = vec![];
    let mut pnl: f64 = 0.0;
    let mut drop_months: HashMap<u64, &Drop> = HashMap::new();

    for drop in input.drops.iter() {
        let mut rng = rand::thread_rng();

        for _ in 0..drop.count {
            let rng_month = rng.gen_range(0..months);
            drop_months.insert(rng_month, drop);
        }
    }

    for i in 0..months {
        monthly_buy.push(input.monthly_compound);
        for m in &mut monthly_buy {
            if drop_months.contains_key(&i) {
                *m -= *m * drop_months.get(&i).unwrap().percentage;
            } else {
                *m += *m * monthly_avg;
            }
        }
    }

    for m in &monthly_buy {
        pnl += m;
    }

    let invested_capital = months as f64 * input.monthly_compound;
    let roi = pnl / invested_capital * 100.0;
    let avg = pnl / months as f64;
    let investments = monthly_buy;

    Stats {
        invested_capital,
        roi,
        pnl,
        avg,
        investments,
    }
}
fn parse_args() -> Input {
    let monthly_compound = read_float("How much money do you expect to invest each month?");
    let yearly_roi =
        read_float("What is the yearly return on investment of the stock? (e.g., 0.04)");
    let deadline = read_unsigned_int("How many years do you plan to compound this investment?");
    let drops = if confirm("Do you want to add drops in the calculation? y/n") {
        parse_drops()
    } else {
        vec![]
    };

    Input {
        monthly_compound,
        yearly_roi,
        deadline,
        drops,
    }
}

fn read_float(prompt: &str) -> f64 {
    loop {
        let input = read_input(prompt);
        match input.trim().parse::<f64>() {
            Ok(value) => break value,
            Err(_) => println!("ERROR: Not a valid float. Please try again."),
        }
    }
}

fn read_unsigned_int(prompt: &str) -> u64 {
    loop {
        let input = read_input(prompt);
        match input.trim().parse::<u64>() {
            Ok(value) => break value,
            Err(_) => println!("ERROR: Not a valid unsigned integer. Please try again."),
        }
    }
}

fn confirm(prompt: &str) -> bool {
    let input = read_input(prompt);
    input.trim().eq_ignore_ascii_case("y")
}

fn read_input(prompt: &str) -> String {
    let mut buffer = String::new();
    println!("{}", prompt);
    stdin()
        .read_line(&mut buffer)
        .expect("ERROR: Failed to read from stdin");
    buffer
}

fn parse_drops() -> Vec<Drop> {
    let mut drops = Vec::new();

    loop {
        let option = read_input(
            "Select an option:\n1. Small dips ~ 5%\n2. Correction ~ 15%\n3. Recession ~ 40%\n4. Exit",
        );

        let count = match option.trim() {
            "1" | "2" | "3" => read_unsigned_int("How many?"),
            "4" => break,
            _ => {
                println!("Invalid option. Please try again.");
                continue;
            }
        };

        let percentage = match option.trim() {
            "1" => 0.05,
            "2" => 0.15,
            "3" => 0.40,
            _ => unreachable!(),
        };

        drops.push(Drop { percentage, count });
    }

    drops
}

fn print_stats(stats: &Stats) {
    println!(" invested_capital: ${:.2}", stats.invested_capital);
    println!(" roi: {:.2}%", stats.roi);
    println!(" pnl: ${:.2}", stats.pnl);
    println!(" monthly avg: ${:.2}", stats.avg);
    for (i, investment) in stats.investments.iter().enumerate() {
        if i % 12 == 0 {
            println!("\tInvestment #{i}. Pnl: ${investment:.2}");
        }
    }
}

fn save_stats(input: &Input, stats: &Stats) {
    let mut file = std::fs::File::create(format!(
        "{compound}-{roi}-{years}.txt",
        compound = input.monthly_compound,
        roi = input.yearly_roi,
        years = input.deadline
    ))
    .expect("ERROR: Unable to create output file");

    file.write_fmt(format_args!(
        " invested_capital: ${:.2}\n",
        stats.invested_capital
    ))
    .expect("ERROR: Failed to write pnl to file");

    file.write_fmt(format_args!(" roi: {:.2}%\n", stats.roi))
        .expect("ERROR: Failed to write roi to file");

    file.write_fmt(format_args!(" pnl: ${:.2}\n", stats.pnl))
        .expect("ERROR: Failed to write pnl to file");

    file.write_fmt(format_args!(" montly avg: ${:.2}\n", stats.avg))
        .expect("ERROR: Failed to write avg to file");

    for (i, investment) in stats.investments.iter().enumerate() {
        if i % 12 == 0 {
            file.write_fmt(format_args!("\tInvestment #{i}. Pnl: ${investment:.2}\n"))
                .expect("ERROR: Failed to write investment");
        }
    }
}

fn main() {
    let input = parse_args();
    let stats = calc_stats(&input);

    let mut buffer = String::new();

    println!("Do you want to save to file? y/n");
    stdin()
        .read_line(&mut buffer)
        .expect("ERROR: Failed to read from stdin");

    if buffer.trim().eq_ignore_ascii_case("y") {
        save_stats(&input, &stats);
    } else {
        print_stats(&stats);
    }
}
