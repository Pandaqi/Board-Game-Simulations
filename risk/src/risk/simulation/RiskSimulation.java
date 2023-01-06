
package risk.simulation;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.Random;
import statistics.DiscreteUniformDistribution;
import statistics.Distribution;

/**
 *
 * @author s148698
 */
public class RiskSimulation {

    static ArrayList<Area> areas;
    int amount_players;
    Player[] players;
    int[] starting_amounts = {35, 30, 25, 20, 15, 10, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2}; // this is not in the constructor, because this syntax isn't allowed there.
    Distribution dice_roll;
    Distribution card_draws;
    static final int amount_simulations = 10;
    static final boolean world_domination = false;
    static SimulationResults simResults;
    
    /**
     * Constructor; initialises parameters
     * @param amount_players amount of players in this game
     */
    public RiskSimulation(int amount_players) {
        this.amount_players = amount_players;

        Random rng = new Random();
        this.dice_roll = new DiscreteUniformDistribution(1, 6, rng);
        this.card_draws = new DiscreteUniformDistribution(0, 2, rng);
        
        this.simResults = new SimulationResults(amount_simulations, amount_players);
    }
    
    /**
     * Simulates one game
     * @param index number of simulation
     */
    public void simulate_game(int index, int[] cur_strategy) {
        int cur_start_amount = starting_amounts[amount_players-3];

        // load all secret missions
        ArrayList<Mission> allMissions = new ArrayList<>();
        allMissions.add(new Mission(2, 24));
        
        allMissions.add(new Mission(1, new int[] {0, 1, 0, 0, 1, 0}));
        allMissions.add(new Mission(1, new int[] {0, 0, 1, 0, 1, 0}));
        allMissions.add(new Mission(1, new int[] {1, 0, 1, 0, 0, 0}));
        allMissions.add(new Mission(1, new int[] {0, 1, 0, 1, 0, 1}));
        
        // initialize players and strategies
        players = new Player[amount_players];
        int[] strategies = new int[amount_players];
        for(int i = 0; i < players.length; i++) {
            // if strategies were not given, make them random
            if(cur_strategy.length != amount_players) {
               strategies[i] = r(0,6);
            } else {
                strategies[i] = cur_strategy[i];
            }
            allMissions.add(new Mission(0, i));
        }
        
        simResults.printFileLine("strategies_used <- \"" + Arrays.toString(strategies) + "\"");
        simResults.printFileLine("amount_players <- " + amount_players);
        
        for(int i = 0; i < players.length; i++) {
            // get a random mission for each player
            int randIndex = (int) Math.round(Math.random() * (allMissions.size() - 1));
            Mission randomMission;
            if(randIndex != 0 && allMissions.get(randIndex).getSpecSingle() != i) {
                randomMission = allMissions.remove(randIndex);
            } else {
                randomMission = allMissions.get(0);
            }
            
            // create the new player
            players[i] = new Player(i, randomMission, strategies[i]);
        }
        
        // create board (using a connected list of areas)
        areas = new ArrayList<>();
        ArrayList<Integer> temp_areas = new ArrayList<>();
        for (int i = 0; i < Board.GAME_BOARD.length; i++) {
            Area create_area = new Area(Board.AREA_NAMES[i], -1, Board.CONTINENTS[i], Board.GAME_BOARD[i], i);
            areas.add( create_area );
            temp_areas.add(i);
        }
        
        // and randomly assign territories to every player
        int cur_player = 0;
        while(temp_areas.size() > 0) {    
            int randIndex = (int) Math.round( Math.random() * (temp_areas.size() - 1.0) );
            Area random_area = areas.get(temp_areas.remove(randIndex));
            players[cur_player].assignArea(random_area);
            random_area.setOwner(cur_player);
                    
            cur_player = (cur_player + 1) % amount_players;
        }
        
        // randomly place infantry across the territories every player owns
        for (int i = 0; i < cur_start_amount; i++) {
            for (int j = 0; j < amount_players; j++) {
                players[j].placeRandomInfantry(1);
            }
        }
        
        // save the starting position
        for( int i = 0; i < amount_players; i++) {
            players[i].saveStartingPos();
        }
        
        // variables to track if anybody won, and what turn we're on
        int counter = 0;
        boolean somebody_won = false;
        int who_won = 0;
        int amount_turns = 0;
        
        // keep performing player's turns until somebody wins
        while(!somebody_won) {            
            // only perform turn if player is still alive
            if(players[cur_player].getAmountAreas() > 0) {
                simulate_turn(cur_player);
                
                // count turn towards the total
                amount_turns++;
                          
                // World domination: everyone is dead, except for one player
                // No world domination: somebody has achieved his/her secret mission (in his own turn!)
                if((world_domination && counter >= (amount_players-1)) || (!world_domination && players[cur_player].hasFulfilledMission(players))) {
                    somebody_won = true;
                    who_won = cur_player;
                }
                
                counter = 0;
                           
                if(index == 0) {
                    simResults.logCurrentSituation(areas, amount_turns);
                }
                
            } else {
                players[cur_player].kill();
                counter++;
            }
            
            // this game is never going to end - abort!
            if(amount_turns > 10000) {
                break;
            }
            
            // move to next player
            cur_player = (cur_player+1) % amount_players;
        }
        
        // decide the ranking
        for(int i = 0; i < amount_players; i++) {
            if(i == who_won) {
                players[i].setRanking(1000000000);
            } else {
               players[i].calculateRanking();
            }
        }
       
        // log results to the simulation results object, who can do fancy stuff with it later
        simResults.setBestStartingPos(players[who_won].getStartingPos());
        simResults.setTotalArmies(getTotalArmies(), index);
        simResults.setWinner(who_won, index, players);
        simResults.setAmountTurns(amount_turns, index);

        Arrays.sort(players);
        simResults.setRanking(players);
        
        /*int first_player_strategy = players[0].getStrategy();
        Arrays.sort(players);
        simResults.setRanking(players, first_player_strategy);*/
        //System.out.println("Finished simulation " + index);
    }
    
    /**
     * Simulates the turn of one player
     * @param cur_player the player whose turn to simulate
     */
    public void simulate_turn(int cur_player) {
        //System.out.println("Start:" + getTotalArmies());

        Player player_obj = players[cur_player];      
        
        stepOne(player_obj);
        stepTwo(player_obj, cur_player, false);
        stepThree(player_obj, cur_player);
    }
    
    /**
     * Performs a battle between two players, spanning over two adjacent areas
     * @param our_area the area of the attacker
     * @param enemy_area the area of the enemy
     */
    public void battle(Area our_area, Area enemy_area) {
        // variables that contain the players fighting, and the size of their armies
        boolean fighting = true;
        int our_army = our_area.getInfantry();
        int cur_player = our_area.getOwner(); 
        
        int enemy_army = enemy_area.getInfantry();
        int enemy_player = enemy_area.getOwner();
        
        // if we aren't actually allowed to fight, quit immediately
        if(enemy_army == 0 || our_army < 2) {
            fighting = false;
        }
        
        // keep fighting until somebody dies
        while(fighting) {
            // roll the dice, sort the array (high to low)
            Integer[] our_dice_roll = {(int) dice_roll.nextRandom(), (int) dice_roll.nextRandom(), (int) dice_roll.nextRandom()};
            Arrays.sort(our_dice_roll, Collections.reverseOrder());
              
            Integer[] enemy_dice_roll = {(int) dice_roll.nextRandom(), (int) dice_roll.nextRandom()};
            Arrays.sort(enemy_dice_roll, Collections.reverseOrder());
            
            // decide on the amount of rounds (depends on how many armies are on each side)
            // the attacking player needs at least 2 armies to be able to attack.
            int amount_rounds = Math.min(Math.min(our_army, enemy_army), 2);
            if(our_army - amount_rounds < 1) {
                amount_rounds = 1;
            }

            // check dice rolls (high to low)
            for (int i = 0; i < amount_rounds; i++) {
                if(our_dice_roll[i] > enemy_dice_roll[i]) {
                    // we win!
                    enemy_army -= 1;
                } else {
                    // they win!
                    our_army -= 1;
                }
            }
            
            // System.out.println(Arrays.toString(our_dice_roll) + " || " + Arrays.toString(enemy_dice_roll) + " || " + our_army + " || " + enemy_army);
            // if the enemy has been defeated, or we're not allowed to fight anymore, quit the figh!
            if(enemy_army <= 0 || our_army < 2) {
                fighting = false;
            }
        }
        
        //System.out.println(our_army + " || " + enemy_army);
        
        // if we won, march into enemy territory
        if(enemy_army <= 0) {
            // set a new owner
            enemy_area.setOwner(cur_player);

            // move infantry into the new area
            int amount_moving = (int) Math.floor(our_army * 0.5);
            enemy_area.setInfantry(amount_moving);
            our_area.setInfantry(our_army - amount_moving);
            
            // move area into our possession
            players[enemy_player].rejectArea(enemy_area);
            players[cur_player].assignArea(enemy_area);
            
            // check if we completely killed the bastard (needed for mission stuff)
            players[cur_player].checkPlayerMission(players, enemy_player);
            
            // also, if we won, fetch ourselves a new card!
            players[cur_player].drawCard( (int) card_draws.nextRandom());
        } else {
        // if we lost, simply update both armies and cry in the corner
            enemy_area.setInfantry(enemy_army);
            our_area.setInfantry(our_army);
        }
    }
    
        
    public void stepOne(Player player_obj) {        
        //
        // STEP 1: Getting and placing new armies
        //
        int my_strategy = player_obj.getStrategy();
        
        // You get 1 infantry for each 3 territories you own
        int armies_to_receive = player_obj.getAreaPoints();
        
        // You get infantry for each complete continent you own
        armies_to_receive += player_obj.getContinentPoints();
        
        // Placing the armies according to strategy
        switch(my_strategy) {
            case 2:
            case 0:
                player_obj.placeRandomInfantry(armies_to_receive);
                break;
               
            case 1:
                int favourite_continent = player_obj.getAndSetFavContinent();
                player_obj.placeContinentInfantry(armies_to_receive, favourite_continent);
                break;
                
            case 5:
            case 3:
                player_obj.placeBalancedInfantry(armies_to_receive);
                break;
            
            case 4:
                player_obj.placeSafestInfantry(armies_to_receive);
                break;
            
            case 6:
                player_obj.placeBorderInfantry(armies_to_receive, areas);
                break;
        }
        
        //System.out.println("Step 1:" + getTotalArmies());
    }
    
    public void stepTwo(Player player_obj, int cur_player, boolean second_chance) {
                
        //
        // STEP 2: Attacking (optional)
        //
        int my_strategy = player_obj.getStrategy();
        if(second_chance) {
            my_strategy = 0;
        }
        
        // Find an area we can attack
        boolean found_enemy = false;
        for (int j = 0; j < player_obj.getAmountAreas(); j++) {
            Area our_area = player_obj.getArea(j);
            int[] neighbours = our_area.getNeighbours();
            
            // Check the neighbours...
            for (int i = 0; i < neighbours.length; i++ ) {
                Area enemy_area = areas.get(neighbours[i]);
                if(enemy_area.getOwner() == cur_player) {
                    continue;
                } 
                
                boolean fighting_conditions = true;
                
                // decide if this area is suitable based on strategy
                switch(my_strategy) {
                    case 5:
                    case 0:
                        fighting_conditions = (our_area.getInfantry() >= enemy_area.getInfantry());
                        break;
                        
                    case 1:
                        fighting_conditions = (our_area.getContinent() == player_obj.getFavContinent() || enemy_area.getContinent() == player_obj.getFavContinent());
                        break;
                        
                    case 2:
                        fighting_conditions = true;
                        break;
                        
                    case 3:
                        fighting_conditions = (our_area.getInfantry() >= enemy_area.getInfantry()*2);
                        break;
                    
                    case 4:
                        fighting_conditions = (enemy_area.getNeighbours().length <= our_area.getNeighbours().length);
                    
                    case 6:
                        // check if this area is surrounded by our areas
                        int counter = 0;
                        for(int k = 0; k < enemy_area.getNeighbours().length; k++) {
                            if(areas.get(enemy_area.getNeighbours()[k]).getOwner() == cur_player) {
                                counter++;
                            }
                        }
                        fighting_conditions = (counter == enemy_area.getNeighbours().length);
                        break;
                }
                
                // If we find one that doesn't belong to us, and we are standing strong here, attack!
                if(our_area.getInfantry() >= 2 && fighting_conditions) {
                    battle(our_area, enemy_area);
                    found_enemy = true;
                    break;
                }
            }
            
            if(found_enemy) {
                break;
            }
        }
        
        // to prevent an impasse - nobody does anything because of their strategy
        if(!found_enemy && !second_chance) {
            stepTwo(player_obj, cur_player, true);
        }
        
        //System.out.println("Step 2:" + getTotalArmies());
    }
    
    public void stepThree(Player player_obj, int cur_player) {
        //
        // STEP 3: Moving armies around (one area max)
        //
        int my_strategy = player_obj.getStrategy();
        
        boolean found_something = false;
        for (int j = 0; j < player_obj.getAmountAreas(); j++) {
            Area start_area = player_obj.getArea(j);
            int cur_infantry = start_area.getInfantry();
            
            // no need to check areas with very little armies
            if(cur_infantry <= 3) {
                continue;
            }
            
            // check the neighbours
            int[] neighbours = start_area.getNeighbours();
            for (int i = 0; i < neighbours.length; i++ ) {
                Area end_area = areas.get(neighbours[i]);
                boolean moving_conditions = true;
                
                switch(my_strategy) {
                    case 2:
                    case 0:
                        moving_conditions = true;
                        break;
                    
                    case 1:
                        moving_conditions = (end_area.getContinent() == player_obj.getFavContinent());
                        break;
                    
                    case 5:
                    case 3:
                        moving_conditions = (end_area.getInfantry() < start_area.getInfantry()*0.5);
                        break;
                    
                    case 4:
                        moving_conditions = (end_area.getNeighbours().length >= start_area.getNeighbours().length);
                        break;
                    
                    case 6:
                        // check if all the neighbours are our areas - if not, it's a border area
                        int counter = 0;
                        for(int k = 0; k < end_area.getNeighbours().length; k++) {
                            if(areas.get(end_area.getNeighbours()[k]).getOwner() == cur_player) {
                                counter++;
                            }
                        }
                        moving_conditions = (counter < end_area.getNeighbours().length);
                        break;
                }
                
                // if we find two adjacent areas we own, move armies!
                if(end_area.getOwner() == cur_player && moving_conditions) {
                    int amount_moving = (int) Math.floor(cur_infantry*0.5);
                    start_area.updateInfantry(-amount_moving);
                    end_area.updateInfantry(amount_moving);
                    found_something = true;
                    break;
                }
            }
            
            if(found_something) {
                break;
            }
        }
        
        //System.out.println("Step 3:" + getTotalArmies());
    }
    
    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        RiskSimulation sim = new RiskSimulation(6);
        
        //sim.recursiveStrategyTesting(new int[] {}, sim);
        
        for(int i = 0; i < amount_simulations; i++) {
            sim.simulate_game(i, new int[] {3, 3, 3, 3, 3, 3});
        }
        
        simResults.displayResults(areas);
    }
    
    public void recursiveStrategyTesting(int[] cur_strategy, RiskSimulation sim) {
        // if we've determined a strategy for everyone, perform simulations!
        if(cur_strategy.length == amount_players) {
            System.out.println(Arrays.toString(cur_strategy));
            for(int i = 0; i < amount_simulations; i++) {
                sim.simulate_game(i, cur_strategy);
            }
        // otherwise, continue adding another strategy for the next player
        } else {
            for(int i = 1; i < 7; i++) {
                int[] temp_strat = new int[cur_strategy.length+1];
                for(int j = 0; j < cur_strategy.length; j++) {
                    temp_strat[j] = cur_strategy[j];
                }
                temp_strat[cur_strategy.length] = i;
                recursiveStrategyTesting(temp_strat, sim);
            }
        }
    }
    
    /**
     * A function used for debugging; it gets the current total of armies on the board
     * @return current total sum of armies in play
     */ 
    public int getTotalArmies() {
        int check_sum = 0;
        for(int i = 0; i< areas.size(); i++) {
            check_sum += areas.get(i).getInfantry();
        }
        return check_sum;
    }
    
    public int r(int a, int b) {
        return (int) Math.floor( Math.random() * (b + 1 - a)) + a;
    }
    
}
