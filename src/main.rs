use clap::{Arg, Command};
use std::io::BufRead;

// Results are placed into mutable arguments. color_buf is cleared before use.
fn parse_line(line: &str, read_id: &mut u64, color_buf: &mut Vec<u64>){
    color_buf.clear();

    let tokens = line.split(' ').collect::<Vec<&str>>();
    *read_id = tokens[0].parse::<u64>().unwrap();
    for &token in tokens.iter().skip(1) {
        let color = token.parse::<u64>().unwrap();
        color_buf.push(color);
    }
}

fn print_stats(input: &String, color_names: Option<&String>) {
    let mut reader = std::io::BufReader::new(std::fs::File::open(input).unwrap());
    let mut line = String::new();
    let mut color_buf = Vec::<u64>::new();

    let mut n_reads = 0_u64;
    let mut n_positive_reads = 0_u64;
    let mut n_unique_positive_reads = 0_u64;
    let mut max_color_id = 0_u64;
    while reader.read_line(&mut line).unwrap() > 0{
        let mut read_id = 0_u64;
        parse_line(&line, &mut read_id, &mut color_buf);
        for color in color_buf.iter() {
            if *color > max_color_id {
                max_color_id = *color;
            }
        }
        n_reads += 1;
        n_positive_reads += (!color_buf.is_empty()) as u64;
        n_unique_positive_reads += (color_buf.len() == 1) as u64;
    }

    eprintln!("Number of reads: {}", n_reads);
    eprintln!("Fraction of positive reads: {}", n_positive_reads as f64 / n_reads as f64);
    eprintln!("Fraction of unique positive reads: {}", n_unique_positive_reads as f64 / n_reads as f64);
}

#[allow(clippy::single_match)]
fn main() {
    let cli = Command::new("themisto_tools")
        .version("0.1.0")
        .author("Jarno N. Alanko")
        .arg_required_else_help(true)
    .subcommand(Command::new("stats")
        .about("Prints statistics from a pseudoalignment output file")
        .arg(Arg::new("input")
            .help("Pseudoalignment file")
            .short('i')
            .long("input")
            .required(true))
        .arg(Arg::new("color_names")
            .short('c')
            .long("color-names")
            .help("A file with one color name per line in the same order as in the index")
            .required(false)
    )); // Todo: relevant k-mers thing

    let matches = cli.get_matches();
    match matches.subcommand() {
        Some(("stats", stats_matches)) => {
            let input = stats_matches.get_one::<String>("input").unwrap();
            let color_names = stats_matches.get_one::<String>("input");
            print_stats(input, color_names);
        }
        _ => {}
    }
}
