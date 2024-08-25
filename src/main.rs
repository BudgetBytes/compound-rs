use std::io::Write;

struct Input {
    monthly_compound: f64,
    yearly_avg: f64,
    deadline: u64,
}

struct Stats {
    invested_capital: f64,
    roi: f64,
    pnl: f64,
    avg: f64,
    investments: Vec<f64>,
}

fn calc_stats(input: &Input) -> Stats {
    let monthly_avg = input.yearly_avg / 12.0;
    let months = input.deadline * 12;
    let mut monthly_buy: Vec<f64> = vec![];
    let mut pnl: f64 = 0.0;

    for _ in 0..months {
        monthly_buy.push(input.monthly_compound);
        for m in &mut monthly_buy {
            *m += *m * monthly_avg;
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

fn parse_args(args: &[String]) -> Option<Input> {
    let monthly_compound: f64 = match args.get(1).unwrap().parse() {
        Ok(res) => res,
        Err(err) => {
            eprintln!("ERROR: {err}");
            return None;
        }
    };

    let yearly_avg: f64 = match args.get(2).unwrap().parse() {
        Ok(res) => res,
        Err(err) => {
            eprintln!("ERROR: {err}");
            return None;
        }
    };

    let deadline: u64 = match args.get(3).unwrap().parse() {
        Ok(res) => res,
        Err(err) => {
            eprintln!("ERROR: {err}");
            return None;
        }
    };

    Some(Input {
        monthly_compound,
        yearly_avg,
        deadline,
    })
}

fn print_usage(program: &String) {
    println!("USAGE");
    println!("{program} <monthly_compound> <yearly_avg> <deadline> [save]");
    println!("ARGS");
    println!("monthly_compound -> amount of money you want to compound each month");
    println!("yearly_avg -> Yearly average return of the underlying stock");
    println!("deadline -> Amount of years you want to keep the position");
    println!("save -> Save output to file");
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
        roi = input.yearly_avg,
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
    let args: Vec<String> = std::env::args().collect();
    let program = args
        .first()
        .expect("ERROR: Program name should always exist");
    if args.len() < 4 {
        print_usage(program);
        return;
    }

    if let Some(input) = parse_args(&args) {
        let stats = calc_stats(&input);
        if args.get(4).is_some() {
            save_stats(&input, &stats);
        } else {
            print_stats(&stats);
        }
    } else {
        print_usage(program);
    }
}
