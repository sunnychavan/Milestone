use log::{debug, info, trace};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use rayon::prelude::*;



use crate::game::{
    gamestate::{GameBuilder, State},
    player::{PossiblePlayer, AI},
};

use rusqlite::{params, Connection, Error, Result};

use super::emperor::{NUM_AGENTS, NUM_MATCHES};
use std::time::Instant;
use std::{collections::HashSet, iter::zip};

pub struct Referee {
    pub agents: Vec<AI>,
    results: Vec<Score>,
    pub batch_num: u32,
}

pub type Score = (u32, u32);

impl Referee {
    pub fn new(agents: Vec<AI>, batch_num: u32) -> Referee {
        Referee {
            agents,
            results: vec![(0, 0); *NUM_AGENTS],
            batch_num,
        }
    }

    fn generate_matches(&self) -> HashSet<(usize, usize)> {
        let possible_matches = n_choose_r(*NUM_AGENTS, 2);
        if *NUM_MATCHES > possible_matches {
            panic!(
                "cannot schedule more matches ({}) than possible pairings of agents ({})",
                *NUM_MATCHES, possible_matches
            );
        }

        let mut matches = HashSet::new();
        let mut rng = rand::thread_rng();
        let mut num_matches_created = 0;

        let mut agent_turn = 0;
        let mut agents = (0..*NUM_AGENTS).collect::<Vec<usize>>();
        agents.shuffle(&mut thread_rng());

        while num_matches_created < *NUM_MATCHES {
            let agent_index = agents[agent_turn % *NUM_AGENTS];
            let opponent_agent_index = rng.gen_range(0..*NUM_AGENTS);

            let m: (usize, usize) = (
                std::cmp::min(agent_index, opponent_agent_index),
                std::cmp::max(agent_index, opponent_agent_index),
            );

            agent_turn += 1;
            if agent_index != opponent_agent_index && !matches.contains(&m) {
                matches.insert(m);
                num_matches_created += 1;
            }
        }

        info!("Schedule generated for batch #{}", self.batch_num);

        matches
    }

    pub fn play(&mut self) {
        let matches = Vec::from_iter(self.generate_matches());

        info!("Playing games in batch #{}", self.batch_num);
        let results: Vec<(usize, u8, usize, u8)> = matches
            .into_par_iter()
            .map(|(agent_one_idx, agent_two_idx)| {
                let before = Instant::now();
                let (agent_one_wins, agent_two_wins) = Self::play_one_match(
                    &self.agents[agent_one_idx],
                    &self.agents[agent_two_idx],
                );
                debug!(
                    "Played a match between {} and {} in {:.2?}. (Batch {})",
                    agent_one_idx,
                    agent_two_idx,
                    before.elapsed(),
                    self.batch_num,
                );

                (agent_one_idx, agent_one_wins, agent_two_idx, agent_two_wins)
            })
            .collect();

        for (a1_idx, a1_w, a2_idx, a2_w) in results {
            self.results[a1_idx].0 += a1_w as u32;
            self.results[a1_idx].1 += 2;
            self.results[a2_idx].0 += a2_w as u32;
            self.results[a2_idx].1 += 2;
        }

        debug!(
            "Results of batch {}: {:?}",
            self.batch_num,
            self.results
                .iter()
                .enumerate()
                .collect::<Vec<(usize, &Score)>>()
        );
    }

    fn play_one_match(agent_one: &AI, agent_two: &AI) -> (u8, u8) {
        let game_one = GameBuilder::new()
            .set_player_1(PossiblePlayer::AI(agent_one.to_owned()))
            .set_player_2(PossiblePlayer::AI(agent_two.to_owned()))
            .build();
        let game_two = GameBuilder::new()
            .set_player_1(PossiblePlayer::AI(agent_two.to_owned()))
            .set_player_2(PossiblePlayer::AI(agent_one.to_owned()))
            .build();

        let mut agent_one_wins = 0;
        let mut agent_two_wins = 0;
        match Self::play_one_game(game_one) {
            Some(0) => agent_one_wins += 1,
            Some(1) => agent_two_wins += 1,
            None => (),
            _ => panic!("impossible winner (there are only two players)"),
        }
        match Self::play_one_game(game_two) {
            Some(0) => agent_two_wins += 1,
            Some(1) => agent_one_wins += 1,
            None => (),
            _ => panic!("impossible winner (there are only two players)"),
        }

        (agent_one_wins, agent_two_wins)
    }

    fn play_one_game(mut game: State) -> Option<u8> {
        while game.active {
            game.play_one_turn();
            game.add_to_state_history();
        }
        game.push_game_and_state().unwrap();
        game.winner
    }

    pub fn get_agents_with_results(self) -> Vec<(Score, AI)> {
        zip(self.results, self.agents).collect::<Vec<(Score, AI)>>()
    }

    
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn n_choose_r(n: usize, r: usize) -> usize {
    (n - r + 1..=n).product::<usize>() / factorial(r)
}
