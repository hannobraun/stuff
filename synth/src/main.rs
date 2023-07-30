mod clock;
mod osc;
mod run;
mod signal;
mod wave;

fn main() -> anyhow::Result<()> {
    run::run()
}
