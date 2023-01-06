/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package settlersofcatan;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Random;
import statistics.DiscreteUniformDistribution;
import statistics.Distribution;

/* THIS IS A LIST OF THINGS I COULD HAVE BUILD INTO THE SIMULATION
   and probably should have
   but I ran out of steam, and felt alright with a simulation that was like 98% correct

   - Optimize street building. Now, computers sometimes build a street which they don't end op using.
     But, this is hard to implement, as there's no way to predict what other players will do, so you can't know if a street is going to be useful in the end

   - Use the smart trade-route strategy: when you have an open edge, with a street on one side, and a street on the other side, CLOSE THE GAP!

   - Apparently, trade routes can be broken up by villages of other players. I didn't implement this, as our rules at home didn't say so. Damn you, stupid old rules.

   - Allow a mix of strategies, or allow a computer to switch to a different strategy if the current one isn't working.
   - Essentially, the way strategies were implemented could have been cleaner. Now they are all over the place, and there are probably a few logic errors.
     But so far all the basic principles seem to work, so I don't know how to improve now (without spending even more hours on this simulation)
*/

/**
 *
 * @author s148698
 */
public class SettlersOfCatan {

    
    public static Board BOARD;
    public static Player[] PLAYERS;
    
    public static Results results;
    
    private boolean CREATE_VISUALS = true;
    static int AMOUNT_SIMULATIONS = 1;
    
    public static boolean contains2(int[] array, int v) {
        for(int i = 0; i < array.length; i++) {
            if(array[i] == v) {
                return true;
            }
        }
        return false;
    }
    
    public int r(int max) {
        int val = (new Random()).nextInt(max)+1;
        return val;
    }
    
    public int r(int max, int[] exceptions) {
        int val = 0;
        boolean contains = false;
        do {
            val = (new Random()).nextInt(max)+1;
            contains = contains2(exceptions, val);
        } while(contains);
        
        return val;
    }
    
    public void simulateGame(int amountPlayers, int simNumber) {
        if(simNumber % 1000 == 0) {
            System.out.println("Sim number: " + simNumber);
        }
        
        Painting PAINTING = null;
        if(CREATE_VISUALS) {
            Frame frame = new Frame(1500, 900);
            PAINTING = frame.getPainting();
        }

        /* SETUP */
        // CREATE GAME BOARD
        BOARD = new Board();
        
        // CREATE PLAYERS
        PLAYERS = new Player[amountPlayers];
        for (int i = 0; i < amountPlayers; i++) {
            PLAYERS[i] = new Player(i, amountPlayers);
        }
        
        // SETS STARTING-POSITION STRATEGIES
        /*
          1 => Place village around fields with largest probability
          2 => Place village on harbour location
          3 => Place village at most needed resources
          4 => Place village as far away from other players as possible
          5 => Place your village as close as possible to your other village
        */
        int[][] startingStrategies = new int[amountPlayers][2];
        startingStrategies[0] = new int[]{3,3};
        startingStrategies[1] = new int[]{3,3};
        startingStrategies[2] = new int[]{3,3};
        startingStrategies[3] = new int[]{3,3};
        
        /* RANDOM STARTING STRATEGIES
        startingStrategies[0] = new int[]{r(4),r(4)};
        startingStrategies[1] = new int[]{r(4),r(4)};
        startingStrategies[2] = new int[]{r(4),r(4)};
        startingStrategies[3] = new int[]{r(4),r(4)};
        
        System.out.println(Arrays.deepToString(startingStrategies));
        */
        
        /*
          0 => Street building
          1 => Village building
          2 => City building
          3 => Development cards
          4 => Trading strategy
          5 => Robber strategy
          6 => Social attitude
        */
        // SETS THE SET OF OVERALL STRATEGIES
        PLAYERS[0].setStrategy(new int[]{1,1,4,1,1,2,3});
        PLAYERS[1].setStrategy(new int[]{2,1,4,1,1,2,3});
        PLAYERS[2].setStrategy(new int[]{3,1,4,1,1,2,3});
        PLAYERS[3].setStrategy(new int[]{4,1,4,1,1,2,3});
        
        /* RANDOM PLAYING STRATEGIES
        PLAYERS[0].setStrategy(new int[]{r(4),r(7, new int[]{2,6}),r(4),r(4),1,2,r(3, new int[]{2})});
        PLAYERS[1].setStrategy(new int[]{r(4),r(7, new int[]{2,6}),r(4),r(4),1,2,r(3, new int[]{2})});
        PLAYERS[2].setStrategy(new int[]{r(4),r(7, new int[]{2,6}),r(4),r(4),1,2,r(3, new int[]{2})});
        PLAYERS[3].setStrategy(new int[]{r(4),r(7, new int[]{2,6}),r(4),r(4),1,2,r(3, new int[]{2})});
        
        System.out.println(Arrays.toString(PLAYERS[0].getStrategy()));
        System.out.println(Arrays.toString(PLAYERS[1].getStrategy()));
        System.out.println(Arrays.toString(PLAYERS[2].getStrategy()));
        System.out.println(Arrays.toString(PLAYERS[3].getStrategy()));
        */
        
        // LET PLAYERS CHOOSE STARTING POSITION (twice)
        int curPlayer = (new Random()).nextInt(amountPlayers); // (randomize the starting player)
        
        for(int a = 0; a < 2; a++) {
            for(int i = 0; i < amountPlayers; i++) {
                // the second time, we move counter-clockwise around the players
                // to ensure an equal start for everyone
                int num = (curPlayer + i) % amountPlayers;
                if(a == 1) {
                    num = (curPlayer + (amountPlayers-1-i)) % amountPlayers;
                }
                
                // players late in the chain, will benefit greatly from using a different strategy
                // this makes the game much more fair
                // but essentially requires you to play with two different "best strategies", depending on whether you're one of the first players or not
                int startingStrategy = startingStrategies[num][a];
                if(i == (amountPlayers-1) || i == (amountPlayers-2)) {
                    if(startingStrategy == 3) {
                        startingStrategy = 1;
                    }
                    
                    if(i == (amountPlayers-1)) {
                        startingStrategy = 4;
                    }
                }
                
                // find a vertex to place a building
                Vertex v = BOARD.getStartingPosition(startingStrategy, num);
                v.build(0, num);
                PLAYERS[num].giveBuilding(v);
                
                // place a street randomly next to it                
                Edge e;
                do {
                    e = BOARD.getEdge(v.getEdge(-1));
                } while(e == null);
                e.build(num); 
                PLAYERS[num].giveStreet(e);
                
                // only the first time, give the player the resources that belong to this vertex
                if(a == 0) {
                    PLAYERS[num].giveResources(v.getResources(-1));
                }
            }
        }

        /* EXECUTION */
        // loop turns until the game ends
        int playerWon = -1;
        Random rng = new Random();
        Distribution dice = new DiscreteUniformDistribution(1,6,rng);
        
        int turnCount = 0; // keeps track of turns (useful for imaging)
        
        while(playerWon < 0) {
            Player p = PLAYERS[curPlayer];
            
            BOARD.setCurrentPlayer(p);
            p.resetTurnMessage();
            
            /*****
             * 
             * PART 1: Roll the dice
             * 
            *****/
            int dice1 = (int) dice.nextRandom();
            int dice2 = (int) dice.nextRandom();
            int diceTotal = dice1 + dice2;
            
            p.addTurnMessage("Player " + curPlayer + " throws a " + diceTotal);
            
            if(diceTotal == 7) {
                // everyone loses half their resources! (if they have more than 7)
                for(int i = 0; i < amountPlayers; i++) {
                    PLAYERS[i].robResources();
                }
                
                // the robber is placed on a certain spot (that spot doesn't yield income anymore)
                int[] pos = BOARD.determineBestRobberPos(curPlayer, p.getEnemy());
                BOARD.placeRobber(pos, curPlayer);
                
                // the current player may steal a card from someone
                Field robField = BOARD.getField(pos);
                int victim = robField.pickVictim(curPlayer);
                if(robField.isPlayerAround(p.getEnemy())) {
                    victim = p.getEnemy();
                }
                
                if(victim >= 0) {
                    int pickCard = PLAYERS[victim].pickCard();
                    if(pickCard != -1) {
                        p.addTurnMessage(" => Steals from Player " + victim);
                        PLAYERS[victim].removeResources(new int[]{pickCard});
                        PLAYERS[victim].updateRelations(curPlayer, -1);
                        p.giveResources(pickCard);
                    }
                }
            } else {
                // give all players resources (robber is taken into account down the chain)       
                for(int i = 0; i < amountPlayers; i++) {
                    PLAYERS[i].getResourcesFromBuildings(diceTotal);
                }
            }
            
            /*****
             * 
             * PART 2: Trade
             * 
            *****/
            // determine which resources we'd ideally receive this trading round
            p.checkNeededResources();
            
            // trade with special trade harbours
            p.checkTradeHarbours();
            
            // trade with other players
            p.checkTradePlayers();
            
            // trade with the bank
            p.checkTradeBank();
            
            /*****
             * 
             * PART 3: Build
             * 
            *****/
            // Re-check our available spots (as other players might have build on them)
            p.recheckAvailableSpots();
            
            // 1. special cases: where chosen strategy (in extremis) overrides goal
            // LONGEST TRADEROUTE + COIN FLIP
            if(p.strategy[0] == 1 || (p.strategy[0] == 2 && (new Random()).nextInt(2) == 1)) {
                if(p.checkResources(0)) {
                    p.BUILD(0,0);
                }
            }
            
            // LARGEST KNIGHTHOOD + COIN FLIP
            if(p.strategy[3] == 1 || (p.strategy[3] == 2 && (new Random()).nextInt(2) == 1)) {
                if(p.checkResources(3)) {
                    p.BUILD(3,0);
                }
            }
            
            // IMMEDIATE CITIES + COIN FLIP
            if(p.strategy[2] == 1 || (p.strategy[2] == 2 && (new Random()).nextInt(2) == 1)) {
                if(p.checkResources(2)) {
                    p.BUILD(2,0);
                }
            }
            
            // 2. if we have the resources for our goal ...
            if(p.checkResources(p.goal)) {
                // ... execute the goal
                // implement the VILLAGE-building strategies here
                int mayBuild = 0;
                if(p.goal == 1) {
                    int strat = p.strategy[1];
                    mayBuild = -1;
                    
                    switch(strat) {
                        case 1:
                            // nothing special
                            mayBuild = 0;
                            break;

                        case 2:
                            // only build on spots that give (at least) 2 resources
                            for(int i = 0; i < p.openVertices.size(); i++) {
                                if(p.openVertices.get(i).getNumFields() >= 2) {
                                    mayBuild = i;
                                    break;
                                }
                            }
                            break;

                        case 3:
                            // only build on spots that give 3 resources
                            for(int i = 0; i < p.openVertices.size(); i++) {
                                if(p.openVertices.get(i).getNumFields() >= 3) {
                                    mayBuild = i;
                                    break;
                                }
                            }
                            break;

                        case 4:
                            // only build if you don't already have a village bordering the same field
                            mainLoop:
                            for(int i = 0; i < p.openVertices.size(); i++) {
                                Vertex b = p.openVertices.get(i);
                                
                                for(int j = 0; j < 3; j++) {
                                    Field f = BOARD.getField(b.getFieldByNum(j));
                                    if(f != null) {
                                        for(int k = 0; k < 6; k++) {
                                            if(BOARD.getVertex(f.getVertex(k)).getOwner() == curPlayer) {
                                                continue mainLoop;
                                            }
                                        }
                                    }
                                }
                                
                                // if nothing's wrong, we've found our prime candidate
                                mayBuild = i;
                                break;
                            }
                            break;

                        case 5:
                            // only build if you get field(s) with resources you don't get each round, or get the least

                            // check the current influx of resources (based on villages built)
                            int[] totalResources = new int[5];
                            for(int i = 0; i < p.buildings.size(); i++) {
                                int[] arr = p.buildings.get(i).getResources(-1);
                                for(int j = 0; j < 3; j++) {
                                    if(arr[j] != -1) {
                                        totalResources[arr[j]]++;
                                    }
                                }
                            }
                            
                            // check least resource
                            int least = 0;
                            int leastNum = totalResources[0];
                            for(int i = 1; i < 5; i++) {
                                if(totalResources[i] < leastNum) {
                                    leastNum = totalResources[i];
                                    least = i;
                                }
                            }
                            
                            mainLoop:
                            for(int i = 0; i < p.openVertices.size(); i++) {
                                Vertex b = p.openVertices.get(i);
                                for(int j = 0; j < 3; j++) {
                                    Field f = BOARD.getField(b.getFieldByNum(3));
                                    if(f != null && f.getTileType() == least) {
                                        mayBuild = i;
                                        break mainLoop;
                                    }
                                }
                            }
                            break;

                        case 6:
                            // only build if one field around it has a high probability
                            // (which I, for some reason, saved as a low probValue, so probValue <= 2)
                            mainLoop:
                            for(int i = 0; i < p.openVertices.size(); i++) {
                                Vertex b = p.openVertices.get(i);
                                for(int j = 0; j < 3; j++) {
                                    Field f = BOARD.getField(b.getFieldByNum(3));
                                    if(f != null && f.probValue <= 2) {
                                        mayBuild = i;
                                        break mainLoop;
                                    }
                                }
                            }
                            break;
                            
                        case 7:
                            // build at a harbour location if possible
                            for(int i = 0; i < p.openVertices.size(); i++) {
                                if(p.openVertices.get(i).isHarbour()) {
                                    mayBuild = i;
                                    break;
                                }
                            }
                            
                            // otherwise, just build what you can
                            if(mayBuild == -1) {
                                mayBuild = 0;
                            }

                    }
                }
                
                if(mayBuild >= 0) {
                    // within the BUILD function, it checks if it's actually possible or not (to prevent errors)
                    int result = p.BUILD(p.goal, mayBuild);
                }
            }
            
            // 3. Any additional stuff to think about?
            
            // CHECK WHAT TO DO WITH THE ROBBER
            p.handleRobber();
            
            // check for longest trade route
            int tempLargestRoute = 0;
            for(int i = 0; i < p.streets.size(); i++) {
                Edge s = p.streets.get(i);
                
                // if we find the starting place of a route, recursively find the longest route
                if(s.checkLeft() == 0 || s.checkRight() == 0) {
                    int result = BOARD.checkStreet(s, curPlayer, new ArrayList<Edge>());
                    if(result > tempLargestRoute) {
                        tempLargestRoute = result;
                    }
                }
            }
            
            p.largeRoute = tempLargestRoute;
            
            // if this player's max route is at least 5
            // claim the largestRoute reward (if it's actually the largest route on the board)
            if(p.largeRoute >= 5) {
                if(tempLargestRoute > BOARD.largestRoute) {
                    if(BOARD.largestRouteOwner > -1) {
                        PLAYERS[BOARD.largestRouteOwner].updateScore(-2);
                    }

                    BOARD.largestRoute = tempLargestRoute;
                    BOARD.largestRouteOwner = curPlayer;
                    p.updateScore(2);
                    p.addTurnMessage("Wins largest route!");
                }
            }

            // check if this player won the game (you can only win in your own turn)
            if(p.getScore() >= 10) {
                playerWon = curPlayer;
            }
            

            if(CREATE_VISUALS) {
                // paint the current game state
                PAINTING.paintState(turnCount);
                
                 // wait a few ms (so I can see what's going on)
                try {
                    Thread.sleep(50);
                } catch (InterruptedException ex) {
                    System.out.println("Something went wrong with painting");
                }
            }
            
            // fetch the next player
            curPlayer = (curPlayer + 1) % amountPlayers;
            turnCount++;
            
            if(turnCount >= 1000) {
                break;
            }
        }
        
        /* FINISH */
        if(turnCount < 1000) {
            boolean hadLR = (BOARD.largestRoute == PLAYERS[playerWon].largeRoute);
            boolean hadLK = (BOARD.largestKnighthood == PLAYERS[playerWon].getKnights());
            results.addGame(playerWon, turnCount, hadLR, hadLK, startingStrategies[playerWon], PLAYERS[playerWon].getStrategy());
        }
    }
    
    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        SettlersOfCatan sim = new SettlersOfCatan();
        
        int amountPlayers = 4;
        results = new Results(AMOUNT_SIMULATIONS, amountPlayers);
        
        for(int i = 0; i < AMOUNT_SIMULATIONS; i++) {
            sim.simulateGame(amountPlayers, i);
        }
        
        results.printResults();
    }
    
    
}
