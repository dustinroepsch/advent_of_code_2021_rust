use advent_of_code_2021::problems;
use advent_of_code_2021::problems::ProblemSet;
use std::path::PathBuf;
use structopt::StructOpt;

const PROBLEMS: [ProblemSet; 2] = [problems::one::PROBLEM_SET, problems::two::PROBLEM_SET];

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    pub day: usize,

    #[structopt(parse(from_os_str))]
    pub file: Option<PathBuf>,
}

fn main() {
    let opt: Opt = Opt::from_args();
    let path = opt.file.unwrap();
    let text: String = std::fs::read_to_string(path).unwrap();
    let problem_set = &PROBLEMS[opt.day - 1];
    println!("Day: {}", opt.day);
    println!("Part A: ({})", (problem_set.part_a)(&text));
    println!("Part B: ({})", (problem_set.part_b)(&text));
}
