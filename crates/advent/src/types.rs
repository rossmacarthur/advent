#[derive(Debug)]
pub enum Summary {
    Bench(Vec<Bench>),
    Run(Vec<Run>),
}

#[derive(Debug)]
pub struct Bench {
    pub name: String,
    pub stats: Stats,
}

#[derive(Debug)]
pub struct Run {
    pub name: String,
    pub result: String,
    pub elapsed: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub len: usize,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
}
