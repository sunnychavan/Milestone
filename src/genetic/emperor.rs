use log::info;
use rand::{random, Rng};

use crate::ai::heuristics::NUM_HEURISTICS;
use crate::ai::tree::SearchLimit;
use crate::game::player::AI;

use super::referee::Referee;

const NUM_BATCHES: usize = 10;
const NUM_AGENTS_RETAINED: usize = 2;
const NUM_CHILDREN_PER_RETAINED_AGENT: usize = 2;
const PERTURB_AMT: f64 = 0.1;

pub const NUM_AGENTS: usize = 12;
pub const NUM_MATCHES: usize = 36;
const AGENT_DEPTH: SearchLimit = SearchLimit::Depth(2);

pub fn run() -> Referee {
    let mut prev_batch = initial_batch();
    let mut batch_num = 1;

    while batch_num < NUM_BATCHES {
        prev_batch = run_one_batch(prev_batch);
        batch_num += 1
    }
    prev_batch
}

fn initial_batch() -> Referee {
    let mut agents = vec![];
    for _ in 0..NUM_AGENTS {
        agents.push(random_agent());
    }

    Referee::new(agents.try_into().unwrap(), 1)
}

fn run_one_batch(mut prev: Referee) -> Referee {
    let old_batch_num = prev.batch_num;
    info!(
        "Running batch #{old_batch_num} with agents: {:.3?}",
        prev.agents
    );
    prev.play();
    let old_best_agents = get_best_agents(prev);
    info!("Batch #{old_batch_num} completed with best agents: {old_best_agents:.3?}");
    let new_agents = mutate(old_best_agents);

    Referee::new(new_agents.try_into().unwrap(), old_batch_num + 1)
}

fn get_best_agents(r: Referee) -> Vec<AI> {
    let mut sorted_agents = r.get_agents_with_results();
    sorted_agents.sort_by(|((w1, t1), _), ((w2, t2), _)| {
        let p1 = *w1 as f32 / *t1 as f32;
        let p2 = *w2 as f32 / *t2 as f32;
        p1.partial_cmp(&p2).unwrap()
    });
    sorted_agents
        .into_iter()
        .take(NUM_AGENTS_RETAINED)
        .map(|elt| elt.1)
        .collect::<Vec<AI>>()
}

fn children_from_agent(parent: AI) -> Vec<AI> {
    let mut children = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_CHILDREN_PER_RETAINED_AGENT {
        let mut child_weights = parent.weights.to_owned();
        for (idx, w) in child_weights.into_iter().enumerate() {
            child_weights[idx] = w * rng.gen_range(-PERTURB_AMT..PERTURB_AMT)
        }
        children.push(AI::new(String::default(), child_weights, AGENT_DEPTH))
    }

    children
}

fn random_agent() -> AI {
    let mut weights = [1.0; NUM_HEURISTICS];
    let mut rng = rand::thread_rng();
    for (idx, _) in weights.into_iter().enumerate() {
        weights[idx] = rng.gen_range(0.0..1.5);
    }
    AI::new(String::default(), weights, AGENT_DEPTH)
}

fn mutate(previous_best: Vec<AI>) -> Vec<AI> {
    let mut new_gen = vec![];

    for previous_agent in previous_best.into_iter() {
        new_gen.append(&mut children_from_agent(previous_agent));
    }

    while new_gen.len() < NUM_AGENTS {
        new_gen.push(random_agent());
    }

    new_gen
}
