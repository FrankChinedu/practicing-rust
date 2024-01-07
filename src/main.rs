pub mod builder_pattern;
pub mod myfs;

use builder_pattern::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _worker = WorkerBuilder::new()
        .keep_alive(true)
        .memsize(256 * 1024)
        .keep_alive(false)
        .workload("workload")
        .keep_alive(false);
    Ok(())
}
