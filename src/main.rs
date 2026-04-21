pub(crate) mod config;

use crate::config::LcConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: LcConfig = confy::load("lc", Some("lc"))?;
    dbg!(&cfg);

    Ok(())
}
