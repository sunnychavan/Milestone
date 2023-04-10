use std::{env};

use lazy_static::lazy_static;
use log::{debug, info, warn};
use rand::{Rng};
use rusqlite::{params, Connection, Result};
use crate::{ai::heuristics::NUM_HEURISTICS, DATABASE_URL};
use crate::ai::tree::SearchLimit;
use crate::game::player::AI;
use bincode::{serialize};
use chrono::Utc;
use super::referee::Referee;

lazy_static! {
    static ref NUM_BATCHES: usize =
        env::var("NUM_BATCHES").map_or(10, |elt| match elt.parse() {
            Ok(i) => {
                info!("Using NUM_BATCHES environment variable ({})", i);
                i
            }
            _ => 10,
        });
    static ref NUM_AGENTS_RETAINED: usize = env::var("NUM_AGENTS_RETAINED")
        .map_or(10, |elt| match elt.parse() {
          Ok(i) => {
                info!("Using NUM_AGENTS_RETAINED environment variable ({})", i);
                i
          },
          _ => 10
        });
    static ref NUM_CHILDREN_PER_RETAINED_AGENT: usize =
        env::var("NUM_CHILDREN_PER_RETAINED_AGENT")
            .map_or(2, |elt| match elt.parse() {
              Ok(i) => {
                info!("Using NUM_CHILDREN_PER_RETAINED_AGENT environment variable ({})", i);
                i
              }
              _ => 2
            });
    static ref PERTURB_AMT: f64 =
        env::var("PERTURB_AMT").map_or(0.1, |elt| match elt.parse() {
          Ok(i) => {
            info!("Using PERTURB_AMT environment variable ({})", i);
            i
          }
          _ => 0.1
        });
    pub static ref NUM_AGENTS: usize =
        env::var("NUM_AGENTS").map_or(36, |elt| match elt.parse() {
          Ok(i) => {
            info!("Using NUM_AGENTS environment variable ({})", i);
            i
          }
          _ => 36
        });
    pub static ref NUM_MATCHES: usize =
        env::var("NUM_MATCHES").map_or(108, |elt| match elt.parse() {
          Ok(i) => {
            info!("Using NUM_MATCHES environment variable ({})", i);
            i
          }
          _ => 108
        });
    static ref AGENT_DEPTH: SearchLimit = env::var("AGENT_DEPTH")
        .map_or(SearchLimit::Depth(4), |elt| SearchLimit::Depth(
            match elt.parse() {
              Ok(i) => {
                info!("Using AGENT_DEPTH environment variable ({})", i);
                i
              }
              _ => 4
            }
        ));
}

pub fn run() -> AI {
    let mut prev_batch = initial_batch();
    // prev_batch.push_batch().unwrap();
    let mut batch_num = 1;

    while batch_num <= *NUM_BATCHES {
        prev_batch = run_one_batch(prev_batch);
        // prev_batch.push_batch().unwrap();
        batch_num += 1
    }
    get_best_agents(prev_batch).first().unwrap().to_owned()
}

fn initial_batch() -> Referee {
    let mut agents = vec![];
    for _ in 0..*NUM_AGENTS {
        agents.push(random_agent());
    }

    Referee::new(agents, 1)
}

fn run_one_batch(mut prev: Referee) -> Referee {
    let old_batch_num = prev.batch_num;
    debug!(
        "Running batch #{old_batch_num} with agents: {:#.3?}",
        prev.agents
    );
    prev.play();
    push_batch(&prev).unwrap_or_else(|e| warn!("Could not push to recovery table: {e}"));
    let old_best_agents = get_best_agents(prev);
    info!("Batch #{old_batch_num} completed with best agents: {old_best_agents:#.3?}");
    let new_agents = mutate(old_best_agents);

    Referee::new(new_agents, old_batch_num + 1)
}

fn checked_div(a: u32, b: u32) -> f32 {
    if b == 0 {
        0.0
    } else {
        a as f32 / b as f32
    }
}

fn get_best_agents(r: Referee) -> Vec<AI> {
    let mut sorted_agents = r.get_agents_with_results();
    sorted_agents.sort_by(|((w1, t1), _), ((w2, t2), _)| {
        let p1 = checked_div(*w1, *t1);
        let p2 = checked_div(*w2, *t2);
        p2.partial_cmp(&p1).unwrap()
    });
    sorted_agents
        .into_iter()
        .take(*NUM_AGENTS_RETAINED)
        .map(|elt| elt.1)
        .collect::<Vec<AI>>()
}

fn children_from_agent(parent: AI) -> Vec<AI> {
    let mut children = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..*NUM_CHILDREN_PER_RETAINED_AGENT {
        let mut child_weights = parent.weights.to_owned();
        for (idx, w) in child_weights.into_iter().enumerate() {
            child_weights[idx] =
                w * rng.gen_range(1.0 - *PERTURB_AMT..1.0 + *PERTURB_AMT)
        }
        children.push(AI::new(
            String::default(),
            child_weights,
            AGENT_DEPTH.to_owned(),
        ))
    }

    children
}

fn random_agent() -> AI {
    let mut weights = [1.0; NUM_HEURISTICS];
    let mut rng = rand::thread_rng();
    for (idx, _) in weights.into_iter().enumerate() {
        weights[idx] = rng.gen_range(0.0..1.0);
    }
    AI::new(String::default(), weights, AGENT_DEPTH.to_owned())
}

fn mutate(previous_best: Vec<AI>) -> Vec<AI> {
    let mut new_gen = vec![];

    for previous_agent in previous_best.into_iter() {
        new_gen.append(&mut children_from_agent(previous_agent));
    }

    while new_gen.len() < *NUM_AGENTS {
        new_gen.push(random_agent());
    }

    new_gen
}

fn push_batch(prev: &Referee) -> Result<()> {
    let conn =
        Connection::open(&*DATABASE_URL).unwrap();
    
    let serialized_agents = serialize(&(prev.agents)).unwrap();
    let timestamp = Utc::now().to_string();
    let batch_id = prev.batch_num;

    conn.execute(
        r#"
        INSERT INTO recovery_table (batch_id, agents, timestamp)
        VALUES (?, ?, ?)
        "#,
        params![batch_id, serialized_agents, timestamp],
    )?;

    // let mut stmt = conn.prepare("SELECT agents FROM recovery_table")?;
    // let agents_iter = stmt.query_map([], |row| {
    //     let bin: Vec<u8> = row.get(0)?;
    //     let agents: Vec<AI> = bincode::deserialize(&bin).unwrap();
    //     Ok(agents)
    // })?;

    // for agents in agents_iter {
    //     println!("Found list of agents: {:?}", agents.unwrap());
    // }

    Ok(())
}
