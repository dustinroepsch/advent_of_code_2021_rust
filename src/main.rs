use advent_of_code_2021::problems::{problem_one, problem_one_part_two, two::problem_two};

use advent_of_code_2021::problems::two::problem_two_two;
use structopt::StructOpt;

fn main() {
    #[derive(StructOpt)]
    #[structopt(about = "My solutions to AoC 2021.")]
    struct Opt {
        #[structopt(long, short)]
        problems_to_run: Option<Vec<usize>>,
    }

    let opt = Opt::from_args();
    let problem_sets = [
        || {
            println!("1-1: ({})", problem_one());
            println!("1-2: ({})", problem_one_part_two());
        },
        || {
            println!("2-1: ({})", problem_two());
            println!("2-2: ({})", problem_two_two());
        },
    ];

    if let Some(problems) = opt.problems_to_run {
        for index in problems {
            problem_sets[index]();
        }
    } else {
        println!("No problem specified, running all :");
        println!(
            "--------------------------------------------------------------------------------"
        );
        for set in problem_sets {
            set();
            println!(
                "--------------------------------------------------------------------------------"
            );
        }
    }
}
