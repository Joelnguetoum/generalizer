use clap::ArgMatches;
use crate::interactions::syntax::interaction::Interaction;

pub fn cli_test(_matches: &ArgMatches) {

    let p = Interaction::partitions_at_least_half(&vec![1,2,3]);

    println!("{:?}",p);
}