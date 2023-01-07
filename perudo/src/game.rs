use core::num;

use rand::Rng;

use crate::{config::SimConfig, helpers::{Helpers, Guess, Strat}, challenger::{Challenger}, guesser::Guesser, results::SimResults};

#[derive(Copy, Clone, Debug)]
pub struct PlayerData {
    pub prev: f64,
    pub next: f64
}

pub struct GuessParams {
    pub use_palafico_guessing: bool,
    pub guess_history: Vec<Guess>,
    pub prev_guess: Guess,
    pub strat: Strat,
    pub my_dice: Vec<usize>,
    pub all_dice: Vec<Vec<usize>>,
    pub no_perudo_round: bool,
    pub player_data: PlayerData
}
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
        let mut cur_player:usize = Helpers::get_start_player(player_count);
        let mut prev_player:usize = cur_player;
    
        // pick random strategies and initialize player memories
        let mut strategies = Vec::new();
        let mut all_player_data: Vec<PlayerData> = Vec::new();
        for _i in 0..player_count
        {
            strategies.push(Helpers::pick_random_strategy(cfg));
            all_player_data.push( PlayerData { prev: 0.5, next: 0.5 } );
        }

        // to check if combined strategies work, we fix them on player 0 and approach the simulation that way
        if cfg.track_per_player
        {
            strategies[0] = cfg.fixed_strat;
        }

        Helpers::print(cfg, "STRATEGIES");
        Helpers::print(cfg, &strategies);

        let mut last_player_died:usize = 0;
        let mut last_player_alive:usize = 0;

        // play rounds until somebody loses
        while !game_has_ended
        {
            Helpers::print(cfg, "== NEW ROUND ==");

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

            Helpers::print(cfg, "DICE");
            Helpers::print(cfg, &all_dice);

            let mut prev_guess:Guess = (0,0);
            prev_player = cur_player;

            let mut guess_history:Vec<Guess> = Vec::new();
            let mut no_perudo_round:bool = false;

            // keep doing turns (guessing or challenging) until a challenge ends the round
            loop {
                if num_dice[cur_player] <= 0 { cur_player = (cur_player + 1) % player_count; continue; }

                Helpers::print(cfg, "= NEW TURN =");
                Helpers::print(cfg, "PLAYER?");
                Helpers::print(cfg, cur_player);

                // NOTE: I need to clone most of these things to make this work, which isn't great
                // But it's MUCH cleaner to pass a consistent struct into the functions below,
                // than specifying almost 10 function parameters each time (and the list grows)
                let params = GuessParams {
                    use_palafico_guessing: cfg.use_palafico_guessing,
                    guess_history: guess_history.clone(),
                    prev_guess,
                    strat: strategies[cur_player].clone(),
                    my_dice: all_dice[cur_player].clone(),
                    all_dice: all_dice.clone(),
                    no_perudo_round,
                    player_data: all_player_data[cur_player].clone()
                };

                // do we want to challenge? And what is the result?
                prev_guess = Guesser::execute(&params);
                let challenge_wanted = Challenger::wants_to_challenge(&params) || Helpers::is_initial_guess(prev_guess);

                if challenge_wanted
                {
                    let won = Challenger::execute(cfg, &params, &mut all_player_data, cur_player, prev_player, &mut num_dice);

                    // NOTE: _we_ won the challenge, so turn goes back to the _other_ player
                    if won { cur_player = prev_player; } 
                    break;
                }

                // otherwise, solidify the next guess and continue
                guess_history.push(prev_guess);
                prev_player = cur_player;
                cur_player = (cur_player + 1) % player_count;

                Helpers::print(cfg, "NEW_GUESS");
                Helpers::print(cfg, prev_guess);
            }

            // activate new perudo round if changed player is now on 1 die
            no_perudo_round = (cfg.use_no_perudo_round && num_dice[cur_player] == 1);

            Helpers::print(cfg, "NO PERUDO ROUND?");
            Helpers::print(cfg, no_perudo_round);

            // check for dead and alive players
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
                game_has_ended = num_dead_players >= (player_count-1);
                continue;
            } 

            game_has_ended = num_dead_players >= 1;
        }

        // record the results
        let relevant_player = if cfg.continue_until_winner { last_player_alive } else { last_player_died };
        let my_strats = &strategies[relevant_player];

        res.strats.bluff.push(my_strats.bluff);
        res.strats.init.push(my_strats.init);
        res.strats.guess.push(my_strats.guess);
        res.strats.prev_player.push(my_strats.prev_player);
        res.strats.next_player.push(my_strats.next_player);
        res.strats.chall_offset.push(my_strats.chall_offset);

        res.wins_per_player.push(relevant_player);
    }
}