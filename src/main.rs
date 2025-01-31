mod args;
mod count;
mod evalue;
mod proteome;
mod score;
mod target;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::count::hmmcount;
use crate::evalue::hmmevalue;
use crate::score::hmmscore;
use crate::target::hmmtarget;
use clap::Parser;

/*
* Author Gaurav Sablok
* Date: 2025-1-31
* Universitat Potsdam.
*
* hmm protein domain analyzer and selector. This is the last piece of
* the code i wrote at Universitat Potsdam and i thank you.
* */

fn main() {
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::TargetHMM {
            hmmpath,
            targetdesc,
        } => {
            let command = hmmtarget(hmmpath, &targetdesc.clone()).unwrap();
            println!(
                "The atom coordinates for the following pdbf file are: {:?}",
                command
            );
        }
        Commands::ScoreHMM { hmmpath, score } => {
            let command = hmmscore(hmmpath, score).unwrap();
            println!(
                "The values after applying the score filter are:{:?}",
                command
            );
        }
        Commands::EvalueHMM { hmmpath, evalue } => {
            let command = hmmevalue(hmmpath, evalue).unwrap();
            println!(
                "The values after applying the evalue filter are: {:?}",
                command
            );
        }
        Commands::Targetcounts { hmmpath } => {
            let command = hmmcount(hmmpath).unwrap();
            println!(
                "The domain aligned targets for the enrichment are: {:?}",
                command
            );
        }
    }
}
