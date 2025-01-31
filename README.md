# proteome-hmm
 - rust crate for hmm domain search, selection and annotation mapper. 
 
 ```
  cargo build 
 ```
 ```
 ➜  proteome-model git:(main) ✗ ./target/debug/hmm-domain -h
 set of evolutionary analysis for proteome

 Usage: hmm-domain <COMMAND>

 Commands:
  target-hmm    select the targets from the hmm
  score-hmm     select the scores from the hmm
  evalue-hmm    select the evalues from the hmm
  targetcounts  prepare the unique counts of the domains
  help          Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version
 ```

 Gaurav Sablok
