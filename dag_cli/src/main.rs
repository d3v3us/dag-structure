use clap::{command, Arg, ArgAction};
use dag::stats::DAGStats;

fn main() {
    let matches = command!()
        .arg(Arg::new("input_file").required(true).index(1))
        .arg(
            Arg::new("render")
                .short('r')
                .long("render")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    println!("render: {:?}", matches.get_flag("render"));

    let dag = dag::dag::Dag::from_file(matches.get_one::<String>("input_file").unwrap()).unwrap();
    let stats= dag.calculate();
    print!("{}", stats);

    if matches.get_flag("render"){
        print!("{}", dag);
    }
}
