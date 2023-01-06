
# ---------------------
# DETERMINES WHAT TO DO UPON RECEIVING THE CARD
#  - s = strategy
#  - l = is it the last player?
#  - t = your own table
#  - r = recipient's table
#  - m = the message given with the card
#  - p = player probabilities
# ---------------------
determine_decision <- function(s, l, t, r, m, p) {
  action <- sample(1:3, 1)
  
  if(1 %in% s) {
    action <- 3
  }
  
  if(2 %in% s) {
    action <- 1
  }
  
  if(3 %in% s) {
    action <- 2
  }
  
  if(4 %in% s) {
    action <- sample(1:2, 1)
  }
  
  if(5 %in% s) {
    action <- sample(1:3, 1, prob = c(0.5, 0.25, 0.25))
  }
  
  if(6 %in% s) {
    if(sum(t == m) >= 2) {
      action <- 1
    }
  }
  
  if(7 %in% s) {
    if(sum(t == m) <= 1) {
      action <- 2
    }
  }
  
  if(8 %in% s) {
    if(sum(r == m) >= 2) {
      action <- 1
    }
  }
  
  if(9 %in% s) {
    action <- sample(1:3, 1, prob = p)
  }
  
  # you can't pass the card on if there's nobody left
  # in that case, it's a coin toss
  if(action == 3 && l) {
    action <- sample(1:2, 1)
  }
  
  return(action)
}

# ---------------------
# DETERMINES WHOM TO GIVE THE CARD TO
#  - s = strategy
#  - p = players left
#  - d = data collected on players
# ---------------------
determine_player <- function(s, p, d) {
  player <- sample(p, 1) 
  
  if(8 %in% s) {
    max_prob <- 0
    max_player <- 0
    
    # for all players left ...
    for(i in 1:length(p)) {
      # calculate pass-through probability
      cur_p <- p[[i]]
      
      # can't divide by zero!
      if(d[[cur_p]][[2]] == 0) {
        prob <- 0
      } else {
        prob <- d[[cur_p]][[1]] / d[[cur_p]][[2]]
      }

      if(prob > max_prob) {
        max_prob <- prob
        max_player <- cur_p
      }
    }
    
    if(max_player != 0) {
      player <- max_player
    }

  }
  
  return(player)
}

# ---------------------
# DETERMINES WHICH CARD TO GIVE
#  - s = strategy
#  - h = your own hand
#  - t = your own table
#  - r = recipient's table
# ---------------------
determine_card <- function(s, h, t, r) {
  card <- sample(unlist(h), 1)
  
  if(4 %in% s) {
    # for each card in our hand ...
    for(i in 1:length(h)) {
      # if the card appears at most once before us on the table ...
      if(sum(t == h[[i]]) <= 1) {
        card <- h[[i]]
        break
      }
    }
  }
  
  if(5 %in% s) {
    # for each card in our hand ...
    for(i in 1:length(h)) {
      # if the card appears at least twice before us on the table ...
      if(sum(t == h[[i]]) >= 2) {
        card <- h[[i]]
        break
      }
    }
  }
  
  if(7 %in% s) {
    count_occurences <- sort(table(unlist(r)), decreasing=TRUE)
    if(length(count_occurences) >= 2) {
      if(sum(t == count_occurences[[2]]) > 0) {
        card <- count_occurences[[2]]
      }
    }
  }

  return(card)
}

# ---------------------
# DETERMINES MESSAGE WHEN GIVING CARD
#  - s = strategy
#  - t = true value of card
#  - r = recipient's table
#  - p = player probabilities
# ---------------------
determine_message <- function(s, t, r, p) {
  msg <- sample(1:8, 1)
  
  if(1 %in% s) {
    temp_sample <- 1:8
    temp_sample <- temp_sample[-t]
    msg <- sample(temp_sample, 1)
  }
  
  if(2 %in% s) {
    msg <- t
  }
  
  if(3 %in% s) {
    if(runif(1, 0, 1) >= 0.5) {
      msg <- t
    } else {
      temp_sample <- 1:8
      temp_sample <- temp_sample[-t]
      msg <- sample(temp_sample, 1)
    }
  }
  
  if(6 %in% s || 7 %in% s) {
    count_occurences <- sort(table(unlist(r)), decreasing=TRUE)
    if(length(count_occurences) >= 1) {
      msg <- count_occurences[[1]]
    }
  }
  
  if(9 %in% s) {
    if(runif(1, 0, 1) <= p[[1]]) {
      msg <- t
    } else {
      temp_sample <- 1:8
      temp_sample <- temp_sample[-t]
      msg <- sample(temp_sample, 1)
    }
  }
  
  return(msg)
}


# ---------------------
# SIMULATES ONE GAME
# ---------------------
simulate_game <- function(a_p, return_card_count) {
  # ---------------------
  # CREATE DECK OF CARDS
  # ---------------------
  
  # there are 8 types of cards, and every card appears 8 times in the deck
  cards <- c()
  
  for(i in 1:8) {
    for(j in 1:8) {
      cards <- c(cards, i)
    }
  }
  
  
  # ---------------------
  # HAND CARDS TO PLAYERS (UNTIL DECK IS EMPTY)
  # ---------------------
  amount_players <- a_p
  player_hands <- list()
  
  # these two variables contain the strategies used by players
  giving_strategies <- list()
  receiving_strategies <- list()
  
  # STRATEGIES ARE HERE GODDAMNIT
  # populate the players' hands and array of strategies
  for(i in 1:amount_players) {
    player_hands[[i]] <- list()
    giving_strategies[[i]] <- list()
    receiving_strategies[[i]] <- list()
  }
  
  giving_strategies[[1]] <- list(3)
  receiving_strategies[[1]] <- list(7)
  
  giving_strategies[[2]] <- list(3)
  receiving_strategies[[2]] <- list(7,8)
  
  giving_strategies[[3]] <- list(3,8)
  receiving_strategies[[3]] <- list(7)
  
  giving_strategies[[4]] <- list(3)
  receiving_strategies[[4]] <- list(6,7)
  
  giving_strategies[[5]] <- list(8)
  receiving_strategies[[5]] <- list(3,4,7)
  
  giving_strategies[[6]] <- list(4)
  receiving_strategies[[6]] <- list(4,7)
  
  temp_player <- 1
  
  # while there are cards to distribute
  while(length(cards) > 0) {
    
    # pick a random one from the deck
    random_card <- sample(cards, 1)
    current_hand_size <- length(player_hands[[temp_player]])
    player_hands[[temp_player]][[current_hand_size+1]] <- random_card
    
    # remove it from deck
    index <- which(cards == random_card)
    if(length(index) > 0) { index <- index[[1]] }
    cards <- cards[-index]
    
    # go to next player
    temp_player <- temp_player + 1
    if(temp_player > amount_players) {
      temp_player <- 1
    }
  }
  
  # ---------------------
  # PLAY THE GAME!
  # ---------------------
  
  # this variables holds the cards they have lying in front of them, on the table
  player_table <- list()
  player_stats <- list()
  player_probs <- list()
  player_probs2 <- list()
  
  for(i in 1:amount_players) {
    player_table[[i]] <- list()
    player_stats[[i]] <- list(0, 0)
    player_probs[[i]] <- c()
    player_probs2[[i]] <- c()
  }
  
  # sets the starting player
  cur_player <- sample(1:amount_players, 1)
  
  # keep track of when the game's finished, and who lost
  somebody_lost <- FALSE
  who_lost <- 0
  
  # keeps track of temporary state of the game
  players_left <- 1:amount_players
  new_round <- TRUE
  cur_card <- NULL
  
  amount_cards_check <- list()
  
  # while nobody has lost yet, the game continues
  while(!somebody_lost) {
    # basically, the game is a series of one player giving a card to another
    # and the other player making a decision

    # count amount of cards on table for each player
    if(return_card_count) {
      temp_list <- list()
      for(i in 1:amount_players) {
        temp_list[[i]] <- length(player_table[[i]])
      }
      amount_cards_check[[(length(amount_cards_check)+1)]] <- temp_list
    }
    
    my_hand <- player_hands[[cur_player]]
    my_giving_strategy <- giving_strategies[[cur_player]]
    my_table <- player_table[[cur_player]]
    
    if(new_round) {
      # if the player is out of cards, he/she loses as well
      if(length(my_hand) < 1) {
        somebody_lost <- TRUE
        who_lost <- cur_player
        break
      }
      
      # also remove this player from available players
      players_left <- players_left[-which(players_left == cur_player)]
    }
    
    # pick a NEW player
    # remove him from the available players
    new_player <- determine_player(my_giving_strategy, players_left, player_stats)
    players_left <- players_left[-which(players_left == new_player)]
    new_table <- player_table[[new_player]]
    new_receiving_strategy <- receiving_strategies[[new_player]]
    
    # if a new round starts, pick card from player's hands, and remove it
    if(new_round) {
      new_round <- FALSE
      
      cur_card <- determine_card(my_giving_strategy, my_hand, my_table, new_table)
      
      index <- which(my_hand == cur_card)
      if(length(index) > 0) { index <- index[[1]] }
      
      player_hands[[cur_player]] <- my_hand[-index]
    }
    
    # tell him something about the card (which one is it, according to you?)
    cur_message <- determine_message(my_giving_strategy, cur_card, new_table, player_probs2[[cur_player]])
    
    # the card is now given to the NEW player
    # the NEW player makes a decision (I agree, I disagree, I pass the card on)
    new_decision <- determine_decision(new_receiving_strategy, (length(players_left) < 1), my_table, new_table, cur_message, player_probs[[new_player]]) 
    player_stats[[new_player]][[2]] <- player_stats[[new_player]][[2]] + 1
     
    new_player_wins <- FALSE
    if(new_decision == 1) {
      # I AGREE
      
      if(cur_message == cur_card) {
        # My guess was correct, so I won
        # Add card to current player's table
        new_player_wins <- TRUE
      } else {
        # My guess was incorrect, I lose
        # Add card to new player's table
        new_player_wins <- FALSE
      }
      
    } else if(new_decision == 2) {
      # I DISAGREE
      
      if(cur_message != cur_card) {
        # My guess was correct, so I won
        # Add card to current player's table
        new_player_wins <- TRUE
      } else {
        # My guess was incorrect, I lose
        # Add card to new player's table
        new_player_wins <- FALSE
      }
      
    } else {
      # I PASS THE CARD ON
      player_stats[[new_player]][[1]] <- player_stats[[new_player]][[1]] + 1
      
      cur_player <- new_player
    }
    
    # if the card has found a destination, a new round is started
    # we also check whether anybody has won yet
    if(new_decision == 1 || new_decision == 2) {
      # reset the round and the players left
      new_round <- TRUE
      players_left <- 1:amount_players
      
      if(new_player_wins) {
        table_size <- length(player_table[[cur_player]]) + 1
        player_table[[cur_player]][[table_size]] <- cur_card
        
        count_this_card <- sum(player_table[[cur_player]] == cur_card)
        
        if(count_this_card == 4) {
          somebody_lost <- TRUE
          who_lost <- cur_player
        }
        
      } else {
        cur_player <- new_player
        
        table_size <- length(player_table[[new_player]]) + 1
        player_table[[new_player]][[table_size]] <- cur_card
        
        count_this_card <- sum(player_table[[new_player]] == cur_card)
        
        if(count_this_card == 4) {
          somebody_lost <- TRUE
          who_lost <- new_player
        }
      }
      
    }
    
  }
  
  if(return_card_count) {
    return(amount_cards_check)
  } else {
    return(who_lost)
  }
}

# simulate this shizzle many times, accumulate results
amount_sim <- 500
results <- rep(0, amount_sim)
amount_players <- 6
amount_cards_global <- NULL

# simulate one time to get a timeline of how many cards each player has
amount_cards_global <- simulate_game(amount_players, TRUE)

for(i in 1:amount_sim) {
  results[[i]] <- simulate_game(amount_players, FALSE)
}

# display discrete histogram (which is apparently a barplot)
par(bg=NA)
barplot(table(results), xlab = "speler", ylab = "aantal potjes verloren")

# display overview of how many cards a player has
categories <- c("Player 1", "Player 2", "Player 3", "Player 4", "Player 5", "Player 6", "Player 7")
colors <- c("green", "blue", "black", "magenta", "orange", "red", "purple")
markers <- 1:amount_players

a_matrix <- matrix(unlist(amount_cards_global), ncol = amount_players, byrow = TRUE)

matplot(1:nrow(a_matrix), a_matrix, type="l", col=colors, lty=1, pch=markers,
        bty="n", las=1, xlab="beurten", ylab="aantal kaarten")
legend("topleft", col=colors, categories, bg="transparent", lwd=1, pch=markers)

