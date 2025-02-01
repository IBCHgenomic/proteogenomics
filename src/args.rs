use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "evolutionary-proteome",
    version = "1.0",
    about = "hmm proteome analyze"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// select the targets from the hmm.
    TargetHMM {
        /// provide the path to the hmm file
        hmmpath: String,
        /// filter according to the target description
        targetdesc: String,
    },
    /// select the scores from the hmm
    ScoreHMM {
        /// provide the path to the hmm file
        hmmpath: String,
        /// filter according to the score
        score: String,
    },
    /// select the evalues from the hmm
    EvalueHMM {
        /// provide the path to the hmm file
        hmmpath: String,
        /// filter according to the evalue
        evalue: String,
    },
    /// select the targets based on evalue and target
    EvalTarget {
        /// provide the path to the hmm file
        hmmpath: String,
        /// provide the evalue selector
        evalue: String,
        /// provide the target selector
        target: String,
    },
    /// prepare the unique counts of the domains
    Targetcounts {
        /// provide the path to the hmm file
        hmmpath: String,
    },
}
