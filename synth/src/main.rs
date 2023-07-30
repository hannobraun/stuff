mod run;
mod signal;
mod synth;
mod ui;

fn main() -> anyhow::Result<()> {
    run::run()
}
