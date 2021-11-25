use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;
use anyhow::Result;

mod rpn_calculator;
use crate::rpn_calculator::RpnCalculator;

#[derive(Parser)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Radish",
    about = "Super awesome sample RPN calculator",
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<PathBuf>,
}

// BufReaderから一行ずつファイルを読み込む関数
// BufReadトレイトを実装している型はすべて読み取れるように、トレイト境界をセット
fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:?}", e),
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        // ファイルが指定された場合の処理

        // ファイルを開く
        let f = File::open(path)?;
        // File::openで得られるreaderよりも高水準なreaderを作成
        let reader = BufReader::new(f);

        // ファイルを一行ずつ読みこむ
        let _ = run(reader, opts.verbose);
    } else {
        // ファイルが指定されなかった場合の処理

        let stdin = stdin();
        // Stdin型だと、1バイト読み込みごとに排他制御がかかるので遅くなる
        // そこで、ロックすることでバッファリングして読み出せるStdinLock型に変換し、処理を高速化
        let reader = stdin.lock();
        let _ = run(reader, opts.verbose);
    }

    Ok(())
}
