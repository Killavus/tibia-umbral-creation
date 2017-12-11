extern crate rand;

use std::sync::{Arc, Mutex};
use std::thread;
use rand::Rng;

enum UmbralCreationState {
    None,
    Crude,
    Regular,
    Master,
}

struct CostTable {
    pub cluster_cost: usize,
    pub dream_matter_cost: usize,
}

struct UmbralCreation {
    state: UmbralCreationState,
    pub clusters_spent: usize,
    pub dream_matters_spent: usize,
}

impl UmbralCreation {
    pub fn new() -> Self {
        UmbralCreation {
            state: UmbralCreationState::None,
            clusters_spent: 0,
            dream_matters_spent: 0,
        }
    }

    fn is_done(&self) -> bool {
        match self.state {
            UmbralCreationState::Master => true,
            _ => false,
        }
    }

    fn perform(&mut self) {
        let mut rng = rand::thread_rng();
        while !self.is_done() {
            match self.state {
                UmbralCreationState::None => {
                    let tries = 191;
                    let shot = rng.gen_range(1, tries + 1);

                    if shot <= 121 {
                        self.clusters_spent += 20;
                        self.dream_matters_spent += 1;
                        self.state = UmbralCreationState::Crude;
                    } else {
                        self.clusters_spent += 20;
                    }
                }
                UmbralCreationState::Crude => {
                    let tries = 248;
                    let shot = rng.gen_range(1, tries + 1);
                    if shot <= 96 {
                        self.clusters_spent += 75;
                        self.state = UmbralCreationState::Regular;
                    } else if shot > 96 && shot <= 174 {
                        self.clusters_spent += 75;
                    } else {
                        self.state = UmbralCreationState::None;
                    }
                }
                UmbralCreationState::Regular => {
                    let tries = 131;
                    let shot = rng.gen_range(1, tries + 1);

                    if shot <= 14 {
                        self.clusters_spent += 150;
                        self.state = UmbralCreationState::Master;
                    } else if shot > 14 && shot <= 68 {
                        self.clusters_spent += 75;
                        self.state = UmbralCreationState::Crude;
                    } else {
                        self.state = UmbralCreationState::None;
                    }
                }
                UmbralCreationState::Master => {}
            }
        }
    }
}

use std::process::exit;
use std::env::args;

fn cost_table() -> Result<CostTable, &'static str> {
    let args: Vec<String> = args().collect();
    let cluster_cost = args.get(1)
        .ok_or("Please provide the cluster of solace cost.")?;
    let dream_matter_cost = args.get(2).ok_or("Please provide the dream matter cost.")?;
    let cluster_cost = cluster_cost
        .parse()
        .map_err(|_| "Failed to parse the cluster cost.")?;
    let dream_matter_cost = dream_matter_cost
        .parse()
        .map_err(|_| "Failed to parse the cluster cost.")?;

    Ok(CostTable {
        cluster_cost,
        dream_matter_cost,
    })
}

const SIMULATION_COUNT: usize = 1000000;

fn main() {
    match cost_table() {
        Ok(cost_table) => {
            let simulations = simulate();
            let total_cluster_count: usize = simulations.iter().map(|uc| uc.clusters_spent).sum();
            let total_dream_matter_count: usize =
                simulations.iter().map(|uc| uc.dream_matters_spent).sum();
            let mut sorted_cluster_counts: Vec<usize> =
                simulations.iter().map(|uc| uc.clusters_spent).collect();
            let mut sorted_dream_matter_counts: Vec<usize> = simulations
                .iter()
                .map(|uc| uc.dream_matters_spent)
                .collect();
            sorted_cluster_counts.sort();
            sorted_dream_matter_counts.sort();

            let total_cluster_cost = total_cluster_count * cost_table.cluster_cost;
            let total_dream_matter_cost = total_dream_matter_count * cost_table.dream_matter_cost;
            let total_cost = total_cluster_cost + total_dream_matter_cost;

            let avg_cost = total_cost / SIMULATION_COUNT;
            let avg_cluster_count = total_cluster_count / SIMULATION_COUNT;
            let avg_matter_count = total_dream_matter_count / SIMULATION_COUNT;

            println!("Average umbral creation cost: {} gp", avg_cost);
            println!("Average cluster count per UC: {}", avg_cluster_count);
            println!("Average dream matter count per UC: {}", avg_matter_count);
            println!(
                "Median dream matter count per UC: {}",
                sorted_dream_matter_counts
                    .get(SIMULATION_COUNT / 2)
                    .unwrap()
            );
            println!(
                "Median cluster count per UC: {}",
                sorted_cluster_counts.get(SIMULATION_COUNT / 2).unwrap()
            );
        }
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    }
}

fn simulate() -> Vec<UmbralCreation> {
    let num_cores = 8;
    let ucs: Arc<Mutex<Vec<UmbralCreation>>> =
        Arc::new(Mutex::new(Vec::with_capacity(SIMULATION_COUNT)));

    let mut workers = Vec::with_capacity(num_cores - 1);

    for _ in 0..(num_cores - 1) {
        let tl_ucs = ucs.clone();

        workers.push(thread::spawn(move || {
            loop {
                let mut uc = UmbralCreation::new();
                uc.perform();

                let mut results = tl_ucs.lock().unwrap();
                if results.len() < SIMULATION_COUNT {
                    results.push(uc);
                } else {
                    break;
                }
            }
        }));
    }

    for worker in workers {
        worker.join().expect("Worker dedek");
    }

    if let Ok(ucs) = Arc::try_unwrap(ucs) {
        ucs.into_inner().unwrap()
    } else {
        panic!("Failed to gather simulation results.")
    }
}
