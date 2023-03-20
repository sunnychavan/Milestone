use super::super::cli::play_genetic_game;
use super::heuristics::{HeuristicWeights, NUM_HEURISTICS};
use rand::Rng;
use std::fmt::{self};

#[derive(Default)]
pub struct GeneticAlgorithm {
    pub num_batches: u32,
    pub num_agents_per_batch: u32,
    pub num_agents_retained: u32,
    pub num_children_per_retained_agent: u32,
}

impl GeneticAlgorithm {
    pub fn new() -> GeneticAlgorithm {
        GeneticAlgorithm {
            num_batches: 2,
            num_agents_per_batch: 2,
            num_agents_retained: 1,
            num_children_per_retained_agent: 1,
        }
    }

    pub fn run(self) -> GeneticAlgorithm {
        let mut best_agents_from_previous_batch = None;
        for _ in 0..self.num_batches {
            let batch = Batch::new(
                self.num_agents_per_batch,
                self.num_agents_retained,
                self.num_children_per_retained_agent,
            )
            .initialize_agents(best_agents_from_previous_batch)
            .schedule_agent_matches()
            .run_batch();
            best_agents_from_previous_batch =
                Some(batch.best_agents_from_batch());
        }

        self
    }
}

#[derive(Default)]
pub struct Batch {
    pub num_agents_per_batch: u32,
    pub agents: Vec<Agent>,
    pub num_agents_retained: u32,
    pub num_children_per_retained_agent: u32,
    pub match_schedule: Vec<(usize, usize)>,
}

impl Batch {
    pub fn new(
        num_agents_per_batch: u32,
        num_agents_retained: u32,
        num_children_per_retained_agent: u32,
    ) -> Batch {
        Batch {
            num_agents_per_batch,
            agents: Vec::with_capacity(
                usize::try_from(num_agents_per_batch).unwrap(),
            ),
            num_agents_retained,
            num_children_per_retained_agent,
            match_schedule: Vec::with_capacity(
                usize::try_from(num_agents_per_batch * num_agents_per_batch)
                    .unwrap(),
            ),
        }
    }

    pub fn initialize_agents(
        mut self,
        best_agents: Option<Vec<Agent>>,
    ) -> Batch {
        self.include_best_agents(&best_agents);
        self.add_children_from_best_agents(&best_agents);
        self.generate_and_add_new_agents();
        println!("BATCH AGENT WEIGHTS");
        for agent in self.agents.clone() {
            println!("AGENT WEIGHTS:");
            println!("{:?}", agent.weights);
        }
        self
    }

    pub fn include_best_agents(&mut self, best_agents: &Option<Vec<Agent>>) {
        match best_agents.to_owned() {
            Some(agent_vec) => {
                for agent in agent_vec {
                    self.agents.push(agent)
                }
            }
            None => (),
        }
    }

    pub fn add_children_from_best_agents(
        &mut self,
        best_agents: &Option<Vec<Agent>>,
    ) {
        if self.num_children_per_retained_agent > 0 {
            match best_agents.to_owned() {
                Some(agent_vec) => {
                    for agent in agent_vec {
                        for num_children in
                            0..self.num_children_per_retained_agent
                        {
                            self.agents.push(
                                agent.perturb_weights().normalize_weights(),
                            );
                        }
                    }
                }
                None => (),
            }
        }
    }

    pub fn generate_and_add_new_agents(&mut self) {
        if self.agents.len() < self.num_agents_per_batch as usize {
            let num_agents_to_add =
                self.num_agents_per_batch as usize - self.agents.len();
            for _ in 0..num_agents_to_add {
                self.agents
                    .push(Agent::new().randomize_weights().normalize_weights());
            }
        }
    }

    /* Not a perfect scheduler but currently schedules matches */
    pub fn schedule_agent_matches(mut self) -> Batch {
        let mut rng = rand::thread_rng();
        let mut agent_index = 0;
        while agent_index < self.agents.len() {
            let opponent_agent_index = rng.gen_range(0..self.agents.len());

            if agent_index != opponent_agent_index {
                self.match_schedule
                    .push((agent_index, opponent_agent_index));
                agent_index += 1;
            }
        }
        self
    }

    /* Runs all Matches in a Given Batch */
    pub fn run_batch(mut self) -> Batch {
        for (agent0_index, agent1_index) in self.match_schedule.clone() {
            self.run_match(agent0_index, agent1_index);
        }
        self
    }

    pub fn run_match(&mut self, agent0_index: usize, agent1_index: usize) {
        self.run_game(agent0_index, agent1_index);
        self.run_game(agent1_index, agent0_index);
    }

    /* Runs single game */
    pub fn run_game(&mut self, black_index: usize, white_index: usize) {
        let winner = play_genetic_game(
            self.agents[black_index].weights,
            self.agents[white_index].weights,
        )
        .unwrap();

        println!(" The Winner of the game is ({winner})");
        match winner {
            0 => {
                self.agents[black_index].wins += 1;
                self.agents[white_index].losses += 1
            }
            1 => {
                self.agents[white_index].wins += 1;
                self.agents[black_index].losses += 1
            }
            _ => println!("No Agent Has Won. Something Has Went Wrong"),
        }
    }

    pub fn best_agents_from_batch(self) -> Vec<Agent> {
        let mut agents_clone = self.agents.clone();
        agents_clone.sort_by(|agent0, agent1| agent1.wins.cmp(&agent0.wins));
        agents_clone[0..self.num_agents_retained as usize]
            .iter()
            .map(|&elt| elt.clear_record())
            .collect()
    }
}

#[derive(Clone, Copy, Default)]
pub struct Agent {
    weights: [f64; NUM_HEURISTICS],
    wins: u32,
    losses: u32,
}

impl Agent {
    pub fn new() -> Agent {
        Agent {
            weights: [0.0; NUM_HEURISTICS],
            wins: 0,
            losses: 0,
        }
    }

    /* Used for introducing new random Agents */
    pub fn randomize_weights(mut self) -> Agent {
        let mut rng = rand::thread_rng();
        for w in 0..self.weights.len() {
            self.weights[w] = rng.gen_range(0.0..1.0);
        }
        self
    }

    /* Used for Creating Children Agents */
    pub fn perturb_weights(mut self) -> Agent {
        let mut rng = rand::thread_rng();
        for w in 0..self.weights.len() {
            let perturb_value = rng.gen_range(-1.0..1.0);
            let diff: f64;

            if perturb_value < 0.0 {
                diff = self.weights[w] + perturb_value;
                if diff > 0.0 {
                    self.weights[w] = diff
                } else {
                    self.weights[w] = 0.0
                }
            } else {
                diff = self.weights[w] + perturb_value;
                if diff <= 1.0 {
                    self.weights[w] = diff
                } else {
                    self.weights[w] = 1.0
                }
            }
        }
        self
    }

    pub fn normalize_weights(mut self) -> Agent {
        let sum: f64 = self.weights.iter().sum();
        for w in 0..self.weights.len() {
            self.weights[w] /= sum;
        }
        self
    }

    pub fn clear_record(mut self) -> Agent {
        self.wins = 0;
        self.losses = 0;
        self
    }
}
