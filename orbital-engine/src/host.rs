use anyhow::bail;
use tokio::{fs::File, io::AsyncReadExt, process::Command};

pub struct Host {
    pub store: wasmer::Store,
    pub instance: wasmer::Instance,
}

impl Host {
    pub async fn new() -> anyhow::Result<Self> {
        let status = Command::new("cargo")
            .arg("rustc")
            .args(["--manifest-path", "../orbital-game/Cargo.toml"])
            .args(["--target", "wasm32-unknown-unknown"])
            .args(["--crate-type", "cdylib"])
            .status()
            .await?;
        if !status.success() {
            bail!("Error building WASM module");
        }

        let mut wasm = Vec::new();
        File::open("../target/wasm32-unknown-unknown/debug/orbital_game.wasm")
            .await?
            .read_to_end(&mut wasm)
            .await?;

        let mut store = wasmer::Store::default();
        let module = wasmer::Module::new(&store, &wasm)?;
        let imports = wasmer::imports! {};
        let instance = wasmer::Instance::new(&mut store, &module, &imports)?;

        Ok(Self { store, instance })
    }
}
