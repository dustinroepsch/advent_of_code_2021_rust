use advent_of_code_2021::problems;
use advent_of_code_2021::problems::ProblemSet;
use std::path::PathBuf;
use structopt::StructOpt;

const PROBLEMS: [ProblemSet; 7] = [
    problems::one::PROBLEM_SET,
    problems::two::PROBLEM_SET,
    problems::three::PROBLEM_SET,
    problems::four::PROBLEM_SET,
    problems::five::PROBLEM_SET,
    problems::six::PROBLEM_SET,
    problems::seven::PROBLEM_SET,
];

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    pub day: usize,

    #[structopt(parse(from_os_str))]
    pub file_path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let text =
        std::fs::read_to_string(opt.file_path).expect("Unable to read the provided input file");

    let problem_set = &PROBLEMS.get(opt.day - 1).expect("Invalid choice for day");

    println!("Day: {}", opt.day);
    println!("Part A: ({})", (problem_set.part_a)(&text));
    println!("Part B: ({})", (problem_set.part_b)(&text));
}
