use log::{debug, info, trace};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::game::{
    gamestate::{GameBuilder, State},
    player::{PossiblePlayer, AI},
};

use super::emperor::{NUM_AGENTS, NUM_MATCHES};
use std::{collections::HashSet, iter::zip};

pub struct Referee {
    pub agents: [AI; NUM_AGENTS],
    results: [Result; NUM_AGENTS],
    pub batch_num: u32,
}

pub type Result = (u32, u32);

impl Referee {
    pub fn new(agents: [AI; NUM_AGENTS], batch_num: u32) -> Referee {
        Referee {
            agents,
            results: [(0, 0); NUM_AGENTS],
            batch_num,
        }
    }

    fn generate_matches(&self) -> HashSet<(usize, usize)> {
        let mut matches = HashSet::new();
        let mut rng = rand::thread_rng();
        let mut num_matches_created = 0;

        let mut agents = [0; NUM_AGENTS];
        for (idx, _) in agents.into_iter().enumerate() {
            agents[idx] = idx;
        }
        agents.shuffle(&mut thread_rng());

        while num_matches_created < NUM_MATCHES {
            let agent_index = agents[num_matches_created % NUM_AGENTS];
            let opponent_agent_index = rng.gen_range(0..NUM_AGENTS);

            let m: (usize, usize) = (
                std::cmp::min(agent_index, opponent_agent_index),
                std::cmp::max(agent_index, opponent_agent_index),
            );

            if agent_index != opponent_agent_index && !matches.contains(&m) {
                matches.insert(m);
                num_matches_created += 1;
            }
        }

        matches
    }

    pub fn play(&mut self) {
        let matches = self.generate_matches();
        let num_matches = matches.len();

        for (match_num, (agent_one_idx, agent_two_idx)) in
            matches.iter().enumerate()
        {
            debug!(
                "Playing a match between {} and {} in batch {}. Match {} of {}.",
                agent_one_idx,
                agent_two_idx,
                self.batch_num,
                match_num + 1,
                num_matches,
            );
            let (agent_one_wins, agent_two_wins) = Self::play_one_match(
                &self.agents[*agent_one_idx],
                &self.agents[*agent_two_idx],
            );

            self.results[*agent_one_idx].0 += agent_one_wins as u32;
            self.results[*agent_one_idx].1 += 2;
            self.results[*agent_two_idx].0 += agent_two_wins as u32;
            self.results[*agent_two_idx].1 += 2;
        }

        info!("Results of batch {}: {:?}", self.batch_num, self.results);
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
        }
        game.winner
    }

    pub fn get_agents_with_results(self) -> Vec<(Result, AI)> {
        zip(self.results, self.agents).collect::<Vec<(Result, AI)>>()
    }
}
