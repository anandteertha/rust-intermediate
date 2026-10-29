use concurrency_pitfalls::{conflicting_order_probe, run_transfers};
use std::process::ExitCode;

fn main() -> ExitCode {
    println!(
        "Conflicting order: {} second-lock acquisitions would block.",
        conflicting_order_probe()
    );
    println!("Blocking on those second locks would create circular wait.");
    match run_transfers(10_000) {
        Ok(summary) => {
            println!(
                "Global order: {} transfers completed; balances={:?}",
                summary.transfers, summary.balances
            );
            println!("All workers joined; balances preserved.");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Transfer run failed: {error}");
            ExitCode::FAILURE
        }
    }
}
