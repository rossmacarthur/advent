#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub enum Summary {
    #[cfg_attr(feature = "json", serde(rename = "benches"))]
    Bench(Vec<Bench>),
    #[cfg_attr(feature = "json", serde(rename = "runs"))]
    Run(Vec<Run>),
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct Bench {
    pub name: String,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub stats: Stats,
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct Run {
    pub name: String,
    pub result: String,
    pub elapsed: f64,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "json", derive(serde::Serialize))]
pub struct Stats {
    pub samples: usize,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
}
