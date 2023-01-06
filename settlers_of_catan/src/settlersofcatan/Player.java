/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package settlersofcatan;

import static java.lang.Math.floor;
import static java.lang.Math.min;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.HashSet;
import java.util.Random;
import java.util.Set;
import static settlersofcatan.SettlersOfCatan.BOARD;
import static settlersofcatan.SettlersOfCatan.PLAYERS;
import statistics.DiscreteUniformDistribution;
import statistics.Distribution;

/**
 *
 * @author s148698
 */
public class Player {
    
    int num;
    int knights;
    
    int[] cards;
    
    ArrayList<Edge> streets;
    ArrayList<Vertex> buildings;
    
    Random rng;
    Distribution dist;
    
    int score;
    int largeRoute; // for testing largest route award
    
    int[] inStorage;
    
    int goal; // the current goal of this player (0 => street, 1 => village, 2 => city, 3 => development)
    int[] resourceNeeds;
    
    int[] strategy;
    
    ArrayList<Edge> openEdges;
    ArrayList<Vertex> openVertices;
    
    ArrayList<String> turnMessage;
    
    int[] socialStatus;
    
    public Player(int num, int amountPlayers) {
        this.num = num;
        this.cards = new int[]{0,0,0,0,0};
        this.buildings = new ArrayList<Vertex>();
        this.streets = new ArrayList<Edge>();
        this.score = 2;
        this.inStorage = new int[]{15, 5, 4}; // streets, villages, cities
        
        this.rng = new Random();
        this.dist = new DiscreteUniformDistribution(1,34, rng);
        
        this.knights = 0;
        this.largeRoute = 0;
        this.goal = 0;
        
        this.openEdges = new ArrayList<Edge>();
        this.openVertices = new ArrayList<Vertex>();
        
        turnMessage = new ArrayList<String>();
        socialStatus = new int[amountPlayers];
        
        resourceNeeds = new int[5];
    }
    
    public void giveBuilding(Vertex b) {
        inStorage[1]--;
        buildings.add(b);
        
        addTurnMessage("Builds a village");
        //updateAvailableSpots();
        findNewGoal(0);
    }
    
    public void giveBuildingUpgrade() {
        inStorage[2]--;
        
        addTurnMessage("Upgrades to city");
        findNewGoal(1);
    }
    
    public void giveStreet(Edge e) {
        inStorage[0]--;
        streets.add(e);
        
        addTurnMessage("Builds a street");
        updateAvailableSpots(e);
        findNewGoal(2);
    }
    
    public void findNewGoal(int prevAction) {
        goal = -1;
        if(openVertices.size() > 0) {
            goal = 1; //village
        } else if(openEdges.size() > 0 && (strategy[0] != 2 || (new Random()).nextInt(2) == 1) && strategy[0] != 4) {
            goal = 0; //street
        } else {
            // strategy 5 disallows city building
            if(strategy[2] != 5) {
                // this strategy only allows city building if all villages have been used up
                if(strategy[2] == 4) {
                    if(buildings.size() == 5) {
                        goal = 2;
                    }
                } else {
                    goal = 2; // city
                }

            } else {
                // strategy 4 disallows development card buying
                if(strategy[3] != 4) {
                    goal = 3; // development
                }
            }
        }
        
        // if nothing has been chosen, we always resort to street building
        if(goal == -1) {
            goal = 0;
        }
        
        // if the goal is not in storage, pick something that IS
        if(inStorage[goal] <= 0) {
            boolean foundSomething = false;
            for(int i = 0; i < 3; i++) {
                if(inStorage[i] > 0) {
                    goal = i;
                    foundSomething = true;
                    break;
                }
            }
            if(!foundSomething) {
                goal = 3;
            }
        }
    }
    
    public void updateRelations(int who, int dr) {
        socialStatus[who] += dr;
    }
    
    public int getEnemy() {
        int enemy = -1;
        int lowest = 100;
        
        // a defensive or neutral strategy forbids having enemies
        if(strategy[6] == 1 || strategy[6] == 2) {
            return -1;
        }
        
        for(int i = 0; i < socialStatus.length; i++) {
            // don't hate yourself
            if(i == num) {
                continue;
            }
            
            // hate the lowest one
            if(socialStatus[i] < lowest) {
                enemy = i;
                lowest = socialStatus[i];
            }
        }
        
        return enemy;
    }
    
    public void recheckAvailableSpots() {
        // Remove spots that have been taken (and thus are not available anymore)
        for(int i = 0; i < openEdges.size(); i++) {
            if(openEdges.get(i).isOccupied()) {
                // Punish the one that stole it from us
                updateRelations(openEdges.get(i).getOwner(), -1);
                
                openEdges.remove(i);
                if(i != 0) { i--; }
            } else if(strategy[6] == 1) {
                // the defensive strategy forbids building stuff in the spots of other players
                Player[] players = SettlersOfCatan.PLAYERS;
                for(int j = 0; j < players.length; j++) {
                    if(j == num) { continue; }
                    
                    if(players[j].openEdges.contains(openEdges.get(i))) {
                        openEdges.remove(i);
                        if(i != 0) { i--; } else { break; };
                    }
                }
            }
        }
        
        for(int i = 0; i < openVertices.size(); i++) {
            if(!openVertices.get(i).isAllowed(num)) {
                // Punish the one that stole it from us
                int owner = openVertices.get(i).getOwner();
                if(owner >= 0) {
                    updateRelations(owner, -1);
                }
                
                openVertices.remove(i);
                if(i != 0) { i--; }
            } else if(strategy[6] == 1) {
                // the defensive strategy forbids building stuff in the spots of other players
                Player[] players = SettlersOfCatan.PLAYERS;
                for(int j = 0; j < players.length; j++) {
                    if(j == num) { continue; }
                    
                    if(players[j].openVertices.contains(openVertices.get(i))) {
                        openVertices.remove(i);
                        if(i != 0) { i--; } else { break; };
                    }
                }
            }
        }
        
        // Duplicates are already checked for BEFORE inserting something new
    }
    
    public void updateAvailableSpots(Edge e) {
        Board b = SettlersOfCatan.BOARD;
        
        // check all surrounding EDGES
        // the ones that are not already used, have become available
        for(int i = 0; i < 2; i++) {
            Edge tempE = b.getEdge(e.getEdgeLeft(i));
            if(tempE == null) {
                continue;
            }
            
            if(!tempE.isOccupied() && !openEdges.contains(tempE)) {
                openEdges.add(tempE);
            }
        }
        
        for(int i = 0; i < 2; i++) {
            Edge tempE = b.getEdge(e.getEdgeRight(i));
            if(tempE == null) {
                continue;
            }
            
            if(!tempE.isOccupied() && !openEdges.contains(tempE)) {
                openEdges.add(tempE);
            }
        }
        
        // check surrounding VERTICES
        // the ones that are allowed, have become available
        Vertex v1 = b.getVertex(e.getVertexLeft());
        if(v1 != null && v1.isAllowed(num) && !openVertices.contains(v1)) {
            openVertices.add(v1);
        }
        
        Vertex v2 = b.getVertex(e.getVertexRight());
        if(v2 != null && v2.isAllowed(num) && !openVertices.contains(v2)) {
            openVertices.add(v2);
        }
    }
    
    public void giveResources(int[] res) {
        for(int i = 0; i < res.length; i++) {
            giveResources(res[i]);
        }
    }
    
    public int pickCard() {
        if(countTotal(cards) <= 0) {
            return -1;
        }
        
        int card = -1;
        do {
            card = (new Random()).nextInt(5);
        } while(cards[card] == 0);
        
        return card;
    }
    
    public int countTotal(int[] arr) {
        int sum = 0;
        for(int i = 0; i < arr.length; i++) {
            sum += arr[i];
        }
        return sum;
    }
    
    public void removeResources(int[] res) {
        for(int i = 0; i < res.length; i++) {
            removeResources(res[i]);
        }
    }
    
    public void removeResources(int res) {
        cards[res]--;
    }
    
    public void giveResources(int res) {
        if(res < 0) {
            return;
        }
        cards[res]++;
    }

    void getResourcesFromBuildings(int checkNum) {
        for(int i = 0; i < buildings.size(); i++) {
            giveResources(buildings.get(i).getResources(checkNum));
        }
    }
    
    public boolean updateScore(int ds) {
        score += ds;
        
        if(score >= 10) {
            return true;
        }
        return false;
    }
    
    public int getScore() {
        return score;
    }
    
    public int findLeastResource() {
        int least = cards[0];
        int leastType = 0;
        for(int i = 1; i < 5; i++) {
            int myFreq = cards[i];
            if(myFreq < least) {
                least = myFreq;
                leastType = i;
            }
        }
        return leastType;
    }
    
    public int getRandomNeededResource() {
        if(needNoResources()) {
            return (new Random()).nextInt(5);
        }
        
        int card = -1;
        do {
            card = (new Random()).nextInt(5);
        } while(resourceNeeds[card] < 0);
        return card;
    }
    
    public boolean shouldTrade() {
        // solves an error I don't understand the origin of
        // (a nullpointerexception in this function)
        if(resourceNeeds == null) {
            return false;
        }
        
        int allDone = 0;
        for(int i = 0; i < resourceNeeds.length; i++) {
            if(resourceNeeds[i] <= 0) {
                allDone++;
            }
        }
        
        // if we don't need anything, why trade?
        return !(allDone >= 5);
    }
    
    public boolean needNoResources() {
        int allDone = 0;
        for(int i = 0; i < resourceNeeds.length; i++) {
            if(resourceNeeds[i] <= 0) {
                allDone++;
            }
        }
        
        return (allDone == 5);
    }
    
    public boolean needAllResources() {
        int allDone = 0;
        for(int i = 0; i < resourceNeeds.length; i++) {
            if(resourceNeeds[i] > 0 || cards[i] == 0) {
                allDone++;
            }
        }
        
        return (allDone == 5);
    }
    
    public void checkNeededResources() {
        // The cost of everything
        int[][] resources = new int[4][5];
        resources[0] = new int[]{1,0,0,1,0};
        resources[1] = new int[]{1,1,1,1,0};
        resources[2] = new int[]{0,2,0,0,3};
        resources[3] = new int[]{0,1,1,0,1};
        
        // Subtract the cost of our goal from what we already have
        // If a resource is _positive_, we still need it
        // If a resource is _zero_ or _negative_, we're all good
        for(int i = 0; i < 5; i++) {
            resourceNeeds[i] = resources[goal][i] - cards[i];
        }
    }
    
    public void checkTradeBank() {
        if(!shouldTrade()) { return; }
        int wantedType = getRandomNeededResource();
        
        for(int i = 0; i < 5; i++) {
            // if we have four or more of a certain type
            // trade against the type we have the least
            if(cards[i] >= 4 && resourceNeeds[i] <= -4) {
                removeResources(new int[]{i, i, i, i});
                giveResources(wantedType);
                
                resourceNeeds[wantedType]--;
                resourceNeeds[i] += 4;
                
                addTurnMessage("Bank Trade: 4 " + toResourceString(i) + " for 1 " + toResourceString(wantedType));
            }
        }
    }
    
    public void checkTradePlayers() {
        if(!shouldTrade()) { return; }
        
        int myStrat = strategy[4];
        
        // we do not trade with other people!
        if(myStrat == 4) {
            return;
        }
        
        Player[] players = SettlersOfCatan.PLAYERS;
        
        int attitude = (new Random()).nextInt(1); // strategy 3 = random by default
        if(myStrat == 1) {
            attitude = 0;
        } else if(myStrat == 2) {
            attitude = 1;
        }
        
        ArrayList<TradeDeal> deals = new ArrayList<TradeDeal>();
        // go through all players ...
        for(int i = 0; i < players.length; i++) {
            // ... except yourself, of course
            if(i == num) {
                continue;
            }
            
            // ... and the people you hate, or who hate you (with a certain probability)
            int rand = (new Random()).nextInt(2);
            if(players[i].getEnemy() == num || getEnemy() == i) {
                if(rand >= 1) {
                    continue;
                }
            }
            
            Player p = players[i];
            p.checkNeededResources();
            
            int[] copyResourceNeeds = new int[5];
            System.arraycopy(resourceNeeds, 0, copyResourceNeeds, 0, 5);
            
            TradeDeal newDeal = new TradeDeal(num, i);
            for(int a = 0; a < 5; a++) {
                // if we need this resource
                if(copyResourceNeeds[a] > 0) {
                    // and the other player has it
                    if(p.resourceNeeds[a] <= -1) {
                        // check if we have enough of value to give in return
                        int maxExchange = min(-p.resourceNeeds[a], copyResourceNeeds[a]);
                        
                        // as long as we want to trade, check if we can, and if so, save the transaction
                        for(int b = 0; b < maxExchange; b++) {
                            for(int c = 0; c < 5; c++) {
                                if(c == a) {
                                    continue;
                                }
                                if(copyResourceNeeds[c] <= -1 && p.resourceNeeds[c] > 0) {
                                    newDeal.addFrom(c);
                                    newDeal.addTo(a);
                                    
                                    copyResourceNeeds[c]++;
                                    p.resourceNeeds[c]--;
                                }
                            }
                        }
                    }
                }
            }
            
            newDeal.saveLeftoverResources(copyResourceNeeds);
            
            if(newDeal.getTotal() > 0) {            
                // add the deal to the complete list of deals
                deals.add(newDeal);
            }
        }
        
        //System.out.println("Amount deals: " + deals.size());
        
        // pick the BEST deal (order arraylist high to low, pick first)
        Collections.sort(deals, new Comparator<TradeDeal>(){
            public int compare(TradeDeal o1, TradeDeal o2){
                if(o1.getTotal() == o2.getTotal()) {
                 return 0;
                }
                return o1.getTotal() > o2.getTotal() ? -1 : 1;
            }
        });
        
        if(deals.size() <= 0) {
            return;
        }
        
        TradeDeal bestDeal = null;
        for(int i = 0; i < deals.size(); i++) {
            boolean extraRequested = false;
            // if the player hates you, he will ask a lot more
            if(players[deals.get(i).giver].getEnemy() == num) {
                deals.get(i).requestExtra();
                deals.get(i).requestExtra();
                extraRequested = true;
            }
            
            // if the player is smug, he will ask more than an equal exchange
            if((i == 0 && deals.size() > 1)) {
                deals.get(i).requestExtra();
                extraRequested = true;
            } 
            
            // if we are unforgiving, we do not take such a blown up deal
            // otherwise, just take it, cuz it's the best
            if(extraRequested && attitude == 0) {
                continue;
            } else {
                bestDeal = deals.get(i);
                break;
            }
        }
        
        if(bestDeal == null) {
            return;
        }
        
        addTurnMessage("Player Trade: Player " + bestDeal.giver + " gives " + countTotal(bestDeal.TO) + " for " + countTotal(bestDeal.FROM));
        
        // execute the chosen deal (exchange cards)
        for(int i = 0; i < 5; i++) {
            cards[i] -= bestDeal.FROM[i];
            cards[i] += bestDeal.TO[i];
            
            players[bestDeal.giver].cards[i] += bestDeal.FROM[i];
            players[bestDeal.giver].cards[i] -= bestDeal.TO[i];
        }
        
        // update relations between players (because a succesful deal, is a succesful deal!)
        updateRelations(bestDeal.giver, 1);
        players[bestDeal.giver].updateRelations(num, 1);

        
    }
    
    public void checkTradeHarbours() {
        if(!shouldTrade()) { return; }
        
        for(int i = 0; i < buildings.size(); i++) {
            if(buildings.get(i).isHarbour()) {
                int type = buildings.get(i).getHarbour();
                int amount = 2;
                if(type == -1) { 
                    amount = 3; 
                    type = getRandomNeededResource();
                }
                
                if(resourceNeeds[type] > 0) {
                    for(int j = 0; j < 5; j++) {
                        // trade our cards to retrieve the one we need (for cheap, relatively)
                        // but, it would be stupid to trade away cards we NEED later
                        if(cards[j] >= amount && resourceNeeds[j] <= -amount) {
                            
                            for(int a = 0; a < amount; a++) {
                                removeResources(j);
                            }

                            giveResources(type);
                            
                            // keep dynamic track of our resource needs
                            resourceNeeds[type]--;
                            resourceNeeds[j] += amount;
                            
                            addTurnMessage("Harbour trade: " + amount + " " + toResourceString(j) + " for 1 " + toResourceString(type));
                        }
                    }
                }
            }
        }
    }
    
    public String toResourceString(int i) {
        if(i < 0) {
            return "NOTFOUND";
        }
        String[] res = new String[]{"wood", "wool", "corn", "clay", "ore"};
        return res[i];
    }
    
    // 0 => street, 1 => village, 2 => city, 3 => development
    public boolean checkResources(int what) {
        
        int[][] resources = new int[4][5];
        resources[0] = new int[]{1,0,0,1,0};
        resources[1] = new int[]{1,1,1,1,0};
        resources[2] = new int[]{0,2,0,0,3};
        resources[3] = new int[]{0,1,1,0,1};
        
        // of course, we can't build it if we don't have it in storage
        if(what != 3 && inStorage[what] <= 0) {
            return false;
        }
        
        for(int i = 0; i < resources[what].length; i++) {
            if(cards[i] < resources[what][i]) {
                return false;
            }
        }
        
        return true;
    }
    
    public void robResources() {
        if(countTotal(cards) > 7) {
            int discardAmount = (int) floor(countTotal(cards)*0.5);
            for(int i = 0; i < discardAmount; i++) { 
                // find a type of card we DO have
                int card = -1;
                do {
                    card = (new Random()).nextInt(5);
                } while(cards[card] == 0 || (shouldTrade() && resourceNeeds[card] > 0 && !needAllResources()));
                // if shouldTrade() is true, we have resource needs, 
                // and we don't want to give away cards that we need!
                // otherwise, giving away any card is fine
                
                // remove it
                cards[card]--;
                resourceNeeds[card]++;
            }
        }
    }
    
    public void handleRobber() {
        int myStrat = strategy[5];
        
        switch(myStrat) {
            case 1:
                if(countTotal(cards) > 7) {
                    // build something
                    buildRandomly();
                }
                break;
            
            case 2:
                if(countTotal(cards) > 8) {
                    // build something
                    buildRandomly();
                }
                break;
            
            case 3:
                if(countTotal(cards) > 9) {
                    // build something
                    buildRandomly();
                }
                break;
            
            case 4:
                if(countTotal(cards) > 7) {
                    int resNeededLeft = 0;
                    for(int i = 0; i < 5; i++) {
                        if(resourceNeeds[i] > 0) {
                            resNeededLeft += resourceNeeds[i];
                        }
                    }
                    // if we are at least 2 resources away from our goal
                    if(resNeededLeft >= 2) {
                        // build something
                        buildRandomly();
                    }
                }
                break;
                
        }
    }
    
    public int BUILD(int type, int edgeNum) {
        switch(type) {
            case 0:
                if(openEdges.size() <= edgeNum) {
                    return -1;
                }
                
                Edge e = openEdges.get(edgeNum);
                e.build(num);
                giveStreet(e);
                removeResources(new int[]{0,3});
                openEdges.remove(edgeNum);
                return 0;
                
            case 1:
                if(openVertices.size() <= edgeNum) {
                    return -1;
                }
                
                Vertex v = openVertices.get(edgeNum);
                v.build(0, num);
                giveBuilding(v);
                updateScore(1);
                removeResources(new int[]{0,1,2,3});
                openVertices.remove(edgeNum);
                return 0;
            
            case 2:
                for(int i = 0; i < buildings.size(); i++) {
                    Vertex b = buildings.get(i);
                    if(b.getType() == 0) {
                        b.upgrade();
                        giveBuildingUpgrade();
                        updateScore(1);
                        removeResources(new int[]{2,2,4,4,4});
                        break;
                   }
               } 
                return 0;
            
            case 3:
                removeResources(new int[]{1,2,4});
                return drawDevelopment();
        }
        return -1;
    }
    
    public void buildRandomly() {
        for(int i = 0; i < 4; i++) {
            if(checkResources(i)) {
                BUILD(i, 0);
            }
        }
    }
    
    public int drawDevelopment() {
        ArrayList<Integer> devCards = SettlersOfCatan.BOARD.developmentCards;
        int randNum = (new Random()).nextInt(devCards.size());
        int randCard = devCards.get(randNum);
        
        switch (randCard) {
            case 0:
                // KNIGHT
                knights++;
                devCards.remove(randNum);
                addTurnMessage("Draws development card (KNIGHT)");
                break;
                
            case 1:
                // PROGRESS (is returned to the deck)
                if(!shouldTrade()) {
                    giveResources(new int[]{findLeastResource(), findLeastResource()});
                } else {
                    giveResources(new int[]{getRandomNeededResource(), getRandomNeededResource()});
                }
                addTurnMessage("Draws development card (PROGRESS)");
                break;
                
            case 2:
                // VICTORY POINTS
                updateScore(1);
                devCards.remove(randNum);
                addTurnMessage("Draws development card (POINT)");
                break;
                
            default:
                break;
        }
        
        // a knight allows for the repositioning of the robber
        // and, perhaps, relocation of Largest Knighthood award
        if(randCard == 0) {
            Board BOARD = SettlersOfCatan.BOARD;
            Player[] PLAYERS = SettlersOfCatan.PLAYERS;
            // the robber is placed on a certain spot (that spot doesn't yield income anymore)
            int[] pos = BOARD.determineBestRobberPos(num, getEnemy());
            BOARD.placeRobber(pos, num);

            // the current player may steal a card from someone
            Field robField = BOARD.getField(pos);
            int victim = robField.pickVictim(num);
            if(robField.isPlayerAround(getEnemy())) {
                victim = getEnemy();
            }
            
            if(victim >= 0) {
                int pickCard = PLAYERS[victim].pickCard();
                if(pickCard != -1) {
                    PLAYERS[victim].removeResources(new int[]{pickCard});
                    giveResources(pickCard);
                    PLAYERS[victim].updateRelations(num, -1);
                    addTurnMessage(" => Steals from Player " + victim);
                }
            }

            // check if we have more knights than anyone else (and at least three)
            if(getKnights() >= 3) {
                if(getKnights() > BOARD.largestKnighthood) {
                    if(BOARD.largestKnighthoodOwner > -1) {
                        PLAYERS[BOARD.largestKnighthoodOwner].updateScore(-2);
                    }
                    updateScore(2);

                    BOARD.largestKnighthood = getKnights();
                    BOARD.largestKnighthoodOwner = num;
                    addTurnMessage("Wins largest knighthood!");
                }
            }
        }
        
        return randCard;
    }
    
    public int getKnights() {
        return knights;
    }
    
    public void setStrategy(int[] strat) {
        this.strategy = strat;
    }
    
    public int[] getStrategy() {
        return strategy;
    }
    
    public void resetTurnMessage() {
        turnMessage = new ArrayList<String>();
    }
    
    public void addTurnMessage(String s) {
        turnMessage.add(s);
    }
    
    public ArrayList<String> getTurnMessage() {
        return turnMessage;
    }
    
}
