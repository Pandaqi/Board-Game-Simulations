use rand::Rng;

use crate::{config::SimConfig, helpers::{Helpers, Guess}, challenger::{Challenger, ChallengeResult}, guesser::Guesser, results::SimResults};

pub struct Game {}

impl Game
{

    pub fn play(cfg:&SimConfig, res:&mut SimResults)
    {
        let mut rng = rand::thread_rng();

        let mut player_count = cfg.player_count;
        if cfg.randomize_player_count {
            player_count = rng.gen_range(3..7);
        }
        
        let mut num_dice = vec![5; player_count];
        let mut game_has_ended:bool = false;
        let mut last_player_died:usize = 0;
        let mut last_player_alive:usize = 0;
        let mut cur_player:usize = Helpers::get_start_player(player_count);
        let mut no_perudo_round:bool = false;

        // pick random strategies
        let mut strategies = Vec::new();
        
        for _i in 0..player_count
        {
            strategies.push(Helpers::pick_random_strategy(cfg));
        }

        // play rounds until somebody loses
        while !game_has_ended
        {

            // throw the dice
            let mut all_dice: Vec<Vec<usize>> = Vec::new();
            for i in 0..player_count
            {
                let mut dice = Vec::new();
                for _j in 0..num_dice[i]
                {
                    dice.push(Helpers::get_random_die_value());
                }

                all_dice.push(dice);
            }

            // keep changing the guess until a challenge ends the round
            let mut cur_guess:Guess = (0,0);

            loop {
                cur_player = (cur_player + 1) % player_count;

                if num_dice[cur_player] <= 0 { continue; }

                // do we want to challenge? And what is the result?
                let my_dice = all_dice[cur_player].clone(); // TO DO: is this clone necessary?
                let my_strats = &strategies[cur_player];
                let challenge_result = Challenger::check(cur_guess, &my_strats, &my_dice, &all_dice, no_perudo_round);

                let mut round_is_done:bool = false;
                match challenge_result {
                    ChallengeResult::Lost => { 
                        num_dice[cur_player] -= 1;
                        round_is_done = true;
                    }
                    ChallengeResult::Won => { 
                        cur_player = (cur_player + player_count - 1) % player_count; 
                        num_dice[cur_player] -= 1;
                        round_is_done = true;
                    }
                    _ => { }
                }

                if round_is_done { break; }

                // otherwise, perform the next guess
                cur_guess = Guesser::execute(cfg, cur_guess, &my_strats, &my_dice, &all_dice, no_perudo_round);
            }

            // check if the changed player is now on 1 die
            no_perudo_round = false;
            if cfg.use_no_perudo_round && num_dice[cur_player] == 1
            {
                no_perudo_round = true;
            }

            // check for player death
            let mut num_dead_players:usize = 0;
            for (idx, elem) in num_dice.iter().enumerate()
            {
                if elem.to_owned() <= 0 { 
                    num_dead_players += 1;
                    last_player_died = idx;
                } else {
                    last_player_alive = idx;
                }
            }

            if cfg.continue_until_winner 
            {
            if num_dead_players >= player_count-1
            {
                    game_has_ended = true;
            }
            continue;
            } 

            if num_dead_players >= 1
            {
                game_has_ended = true; 
            }
        }

        // record the results
        let mut my_strats = &strategies[last_player_died];
        if cfg.continue_until_winner { my_strats = &strategies[last_player_alive]; }

        res.strats.bluff.push(my_strats.bluff);
        res.strats.init.push(my_strats.init);
        res.strats.guess.push(my_strats.guess);
        res.strats.prev_player.push(my_strats.prev_player);
        res.strats.next_player.push(my_strats.next_player);
        res.strats.chall_offset.push(my_strats.chall_offset);
    }
}