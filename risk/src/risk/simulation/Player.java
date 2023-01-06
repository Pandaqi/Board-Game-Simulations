
package risk.simulation;

import java.util.ArrayList;
import java.util.Arrays;

/**
 *
 * @author s148698
 */
public class Player implements Comparable<Player> {
    
    private ArrayList<Area> areas_owned;
    int[] original_areas_owned;
    private int my_num;
    private int my_infantry;
    private boolean alive;
    private int[] my_cards;
    private Mission mission;
    private double my_ranking;
    private int my_strategy;
    private int fav_continent;
    
    /**
     * Constructor; initialising variables
     * @param my_num the player's number, for identification
     */
    public Player(int my_num, Mission my_mission, int my_strategy) {
        this.areas_owned = new ArrayList<Area>();
        this.original_areas_owned = new int[42];
        this.my_num = my_num;
        this.alive = true;
        this.my_cards = new int[3];
        this.mission = my_mission;
        this.my_ranking = 0;
        this.my_strategy = my_strategy;
        this.fav_continent = -1;
    }
    
    public int getArmySize() {
        int sum = 0;
        for(int i = 0; i < areas_owned.size(); i++) {
            sum += areas_owned.get(i).getInfantry();
        }
        return sum;
    }
    
    public int getNum() {
        return my_num;
    }
    
    public void calculateRanking() {
        my_ranking = getContinentPoints() + getAreaPoints() + getArmySize();
    }
    
    public void setRanking(double n) {
        my_ranking = n;
    }
    
    public int getStrategy() {
        return my_strategy;
    }
    
    public double getRanking() {
        return my_ranking;
    }
    
    public int compareTo(Player comparePlayer) {
        double compareQuantity = ((Player) comparePlayer).getRanking();

        //ascending order
        if(compareQuantity > this.my_ranking) {
            return 1;
        } else if(compareQuantity == this.my_ranking) {
            return 0;
        } else {
            return -1;
        }

        //descending order
        //return compareQuantity - this.quantity;
    }
    
        
    public void saveStartingPos() {
        for(int i = 0; i < areas_owned.size(); i++) {
            original_areas_owned[areas_owned.get(i).getIndex()] = 1;
        }
    }
    
    public int[] getStartingPos() {
        return original_areas_owned;
    }
    
    /**
     * Checks whether the player is still alive
     * @return true if player is alive, false if he's out of the game
     */
    public boolean isAlive() {
        return this.alive;
    }
    
    /**
     * Kills the player
     */
    public void kill() {
        this.alive = false;
    }
    
    /**
     * Checks whether the player has fulfilled his secret mission
     * @param players array with all the current players in the game
     * @return true if player fulfilled secret mission, false if not
     */
    public boolean hasFulfilledMission(Player[] players) {
        if(mission.getType() == 1 && checkContinents()) {
            return true;
        } else if(mission.getType() == 0 && !players[mission.getSpecSingle()].isAlive()) {
            return true;
        } else if(mission.getType() == 2 && getAmountAreas() == mission.getSpecSingle()) {
            return true;
        }
        return false;
    }
    
    public void checkPlayerMission(Player[] players, int enemy) {
        // if we defeated somebody, but it wasn't our mission
        if(players[enemy].getAmountAreas() == 0 && mission.getSpecSingle() != enemy) {
            players[enemy].kill();
            // check if somebody else had that mission - if so, they now have to defeat us
            for(int i = 0; i < players.length; i ++) {
                if(players[i].getMission().getType() == 0 && players[i].getMission().getSpecSingle() == enemy) {
                    players[i].getMission().setSpecSingle(my_num);
                }
            }
        }
    }
    
    /**
     * Gets the player's mission
     * @return the player's mission
     */
    public Mission getMission() {
        return mission;
    }
    
    /**
     * Give this player a new area
     * @param area the area to give to this player
     */
    public void assignArea(Area area) {
        areas_owned.add( area );
    }
    
    /**
     * Make this player lose an area (he owns)
     * @param area the area to reject
     */
    public void rejectArea(Area area) {
        areas_owned.remove( area );
    }
    
    /**
     * Gets the amount of areas this players owns
     * @return the amount of areas this player owns
     */
    public int getAmountAreas() {
        return areas_owned.size();
    }
    
    /**
     * Places infantry each on a random area
     * (placeInfantry actually places them, this just generates random areas)
     * @param amount the amount of infantry to place
     */
    public void placeRandomInfantry(int amount) {
        for(int i = 0; i < amount; i++) {
            placeInfantry(getRandomArea(), 1);
        }
    }
    
    public void placeContinentInfantry(int amount, int continent) {
        for(int i = 0; i < amount; i++) {
            placeInfantry(getRandomArea(continent), 1);
        }
    }
    
    public void placeBalancedInfantry(int amount) {
        // create temporary array, so we don't need to recheck things every time
        int[] temp_counter = new int[getAmountAreas()];
        for(int i = 0; i < getAmountAreas(); i++) {
            temp_counter[i] = areas_owned.get(i).getInfantry();
        }
        
        // keep giving armies to the weakest areas
        while(amount > 0) {
            // find the weakest area first
            int cur_min = 1000000000;
            int cur_index = -1;
            for(int i = 0; i < getAmountAreas(); i++) {
                if(temp_counter[i] < cur_min) {
                    cur_min = temp_counter[i];
                    cur_index = i;
                }
            }
            placeInfantry(areas_owned.get(cur_index), 1);
            temp_counter[cur_index]++;
            amount--;
        }
    }
    
    public void placeSafestInfantry(int amount) {
        int connections = 20;
        int index = -1;
        for(int i = 0; i < getAmountAreas(); i++) {
            int amountNeighbours = areas_owned.get(i).getNeighbours().length;
            if(amountNeighbours < connections) {
                connections = amountNeighbours;
                index = i;
            }
        }
        placeInfantry(areas_owned.get(index), amount);
    }
    
    public void placeBorderInfantry(int amount, ArrayList<Area> all_areas) {
        for(int i = 0; i < amount; i++) {
            placeInfantry(getRandomArea("border", all_areas), 1);
        }
    }
    
    /**
     * Places a certain amount of infantry on a single area
     * @param which_area the area to place the new infantry on
     * @param amount the amount of infantry to place
     */
    public void placeInfantry(Area which_area, int amount) {
        which_area.updateInfantry(amount);
    }

    /**
     * Calculate the armies this player gets for the areas he controls
     * @return the amount of armies this player gets for area control
     */
    public int getAreaPoints() {
        return Math.max((int) Math.floor(areas_owned.size() * 0.33), 1);
    }
    
    /*public int getAreaPoints() {
        return Math.max((int) Math.floor(areas_owned.size() * 0.33), 3);
    }*/
    
    /**
     * Calculates whether the player owns any complete continents, and if so, awards armies
     * @return the amount of extra armies this player receives for owning continents
     */
    public int getContinentPoints() {
        int[] running_tally = {0, 0, 0, 0, 0, 0};
        int armies_to_receive = 0;
        for (int i = 0; i < areas_owned.size(); i++ ) {
            int cur_cont = areas_owned.get(i).getContinent();
            running_tally[ cur_cont ]++;
            if(running_tally[cur_cont] == Board.AREAS_PER_CONTINENT[cur_cont]) {
                armies_to_receive += Board.ARMIES_PER_CONTINENT[cur_cont];
            }
        }
        return armies_to_receive;
    }
    
    /**
     * Checks to see if we have all the continents stated in our secret mission
     * @return true if we have succeeded our mission, false otherwise
     */
    public boolean checkContinents() {
        int[] running_tally = {0, 0, 0, 0, 0, 0};
        int[] continents_to_check = mission.getSpecArr().clone();
        int prevSum = arraySum(continents_to_check);
        int curSum = 0;
        for (int i = 0; i < areas_owned.size(); i++ ) {
            int cur_cont = areas_owned.get(i).getContinent();
            running_tally[ cur_cont ]++;
            if(running_tally[cur_cont] == Board.AREAS_PER_CONTINENT[cur_cont]) {
                if(continents_to_check[cur_cont] == 1) {
                    curSum++;
                }
            }
        }
        //System.out.println(prevSum + " | " + Arrays.toString(continents_to_check) + " | " + curSum);
        if(prevSum == curSum) {
            return true;
        }
        return false;
    }
    
    /**
     * Sums all the values of an array
     * @param arr the array containing the values to sum
     * @return the sum of the array its values
     */
    public int arraySum(int[] arr) {
        int sum = 0;
        for(int i = 0; i < arr.length; i++) {
            sum += arr[i];
        }
        return sum;
    }
    
    /**
     * Gets a random area (in this player's possession)
     * @return a random area in this player's possession
     */
    public Area getRandomArea() {
        int rand = (int) Math.round(Math.random() * (areas_owned.size() - 1.0));
        return areas_owned.get(rand);
    }
    
    public Area getRandomArea(int continent) {
        ArrayList<Area> temp_areas = new ArrayList<>();
        for(int i = 0; i < areas_owned.size(); i++) {
            if(areas_owned.get(i).getContinent() == continent) {
                temp_areas.add(areas_owned.get(i));
            }
        }
        if(temp_areas.size() == 0) {
            return getRandomArea();
        }
        return temp_areas.get( (int) Math.round( Math.random() * (temp_areas.size() - 1.0)));
    }
    
    public Area getRandomArea(String what, ArrayList<Area> all_areas) {
        ArrayList<Area> temp_areas = new ArrayList<>();
        for(int i = 0; i < areas_owned.size(); i++) {
            int[] nb = areas_owned.get(i).getNeighbours();
            int counter = 0;
            for(int j = 0; j < nb.length; j++) {
                if(all_areas.get(j).getOwner() == my_num) {
                    counter++;
                }
            }
            if(counter < nb.length - 1) {
                temp_areas.add(areas_owned.get(i));
            }
        }
        if(temp_areas.size() == 0) {
            return getRandomArea();
        }
        return temp_areas.get( (int) Math.round( Math.random() * (temp_areas.size() - 1.0)));
    }
    
    /**
     * Gets a specific area in this player's possession, based on index
     * @param index the index to fetch the area from
     * @return the area required
     */
    public Area getArea(int index) {
        return areas_owned.get(index);
    }
    
    public int getAndSetFavContinent() {
        int[] running_tally = {0, 0, 0, 0, 0, 0};
        for(int i = 0; i < areas_owned.size(); i++) {
            int cur_cont = areas_owned.get(i).getContinent();
            running_tally[cur_cont]++;
            // ignore continents we already own completely
            if(running_tally[cur_cont] == Board.AREAS_PER_CONTINENT[cur_cont]) {
                running_tally[cur_cont] = -1;
            }
        }
        
        int cur_max = -1;
        int fav_cont = -1;
        for(int i = 0; i < running_tally.length; i++) {
            if(running_tally[i] > cur_max) {
                fav_cont = i;
                cur_max = running_tally[i];
            }
        }
        this.fav_continent = fav_cont;
        return fav_cont;
    }
    
    public int getFavContinent() {
        return fav_continent;
    }
    
    /**
     * Draws a card, and immediately checks if it can exchange cards for armies
     * @param rand a random number used to determine which of the three types of cards to draw
     */
    public void drawCard(int rand) {
        // draw a card
        int[] card_worth = {4, 6, 8, 10};
        my_cards[rand]++;
        
        // check if we own the area; if so, get 2 armies (on that area)
        if(Math.random() < (areas_owned.size() / 42.0)) {
            placeInfantry(getRandomArea(), 2);
        }
        
        // check if we should get armies
        // either when we have 3 of the same type, or 1 of each type
        int counter = 0;
        for(int i = 2; i >= 0; i--) {
            if(my_cards[i] >= 3) {
                my_cards[i] = 0;
                placeRandomInfantry(card_worth[i]);
            } else if(my_cards[i] > 0) {
                counter++;
            }
        }
        
        if(counter == 3) {
            my_cards[0]--;
            my_cards[1]--;
            my_cards[2]--;
            placeRandomInfantry(card_worth[3]);
        }
        
        // I ignored "JOKERS" here, because their influence is small, but they take lots of lines of code
    }
    
}
