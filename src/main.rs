use advent_of_code_2021::problems::{problem_one, problem_one_part_two};
use structopt::StructOpt;

fn main() {
    #[derive(StructOpt)]
    #[structopt(about = "My solutions to AoC 2021.")]
    struct Opt {
        #[structopt(long)]
        one: bool,
    }

    let opt = Opt::from_args();

    if opt.one {
        problem_one();
        problem_one_part_two();
    }
}
