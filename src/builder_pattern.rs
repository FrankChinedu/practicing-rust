pub struct Worker {
    pub workload: String,
    pub memsize: u128,
    pub keep_alive: bool,
}
pub struct WorkerBuilder<W> {
    pub workload: W,
    pub memsize: u128,
    pub keep_alive: bool,
}

pub struct NoWorkload;

impl Default for WorkerBuilder<NoWorkload> {
    fn default() -> Self {
        Self {
            workload: NoWorkload,
            memsize: Default::default(),
            keep_alive: Default::default(),
        }
    }
}

impl WorkerBuilder<NoWorkload> {
    pub fn new() -> Self {
        Self {
            workload: NoWorkload,
            memsize: 128 * 1024,
            keep_alive: false,
        }
    }

    pub fn workload(self, workload: impl Into<String>) -> WorkerBuilder<String> {
        WorkerBuilder {
            workload: workload.into(),
            memsize: self.memsize,
            keep_alive: self.keep_alive,
        }
    }
}

impl WorkerBuilder<String> {
    pub fn build(self) -> Worker {
        Worker {
            workload: self.workload,
            memsize: self.memsize,
            keep_alive: self.keep_alive,
        }
    }

    pub fn workload(mut self, workload: impl Into<String>) -> Self {
        self.workload = workload.into();
        self
    }
}

impl<W> WorkerBuilder<W> {
    pub fn keep_alive(mut self, keep_alive: bool) -> Self {
        self.keep_alive = keep_alive;
        self
    }

    pub fn memsize(mut self, memsize: u128) -> Self {
        self.memsize = memsize;
        self
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _worker = WorkerBuilder::new()
        .keep_alive(true)
        .memsize(256 * 1024)
        .keep_alive(false)
        .workload("workload")
        .keep_alive(false);
    Ok(())
}
