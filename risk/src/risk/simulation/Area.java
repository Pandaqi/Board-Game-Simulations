
package risk.simulation;

import java.util.ArrayList;

/**
 *
 * @author s148698
 */
public class Area {
    
    private int[] neighbours; // the neighbouring areas
    private int continent; // the continent we belong to
    private int owner; //the player that owns this area
    private String name;
    private int infantry; 
    private int index;
    
    /**
     * Constructor; initialising variables
     * @param name the name of the area
     * @param owner the player owning the area
     * @param continent the continent the area's in
     * @param neighbours the direct neighbouring areas
     * @param index the index of this country (in the board array)
     */
    public Area(String name, int owner, int continent, int[] neighbours, int index) {
        this.name = name;
        this.neighbours = neighbours;
        this.owner = owner;
        this.continent = continent;
        this.infantry = 0;
        this.index = index;
    }
    
    /**
     * Update the current amount of infantry
     * @param change the amount with which to change the infantry
     */
    public void updateInfantry(int change) {
        infantry += change;
    }
    
    /**
     * Gets owner of this area
     * @return owner of the area
     */
    public int getOwner() {
        return owner;
    }
    
    /**
     * Gets the index of this area
     * @return this area's index
     */
    public int getIndex() {
        return index;
    }
    
    /**
     * Gets amount of infantry on this area
     * @return amount of infantry on this area
     */
    public int getInfantry() {
        return infantry;
    }
    
    /**
     * Gets continent this area belongs to
     * @return the continent this area belongs to
     */
    public int getContinent() {
        return continent;
    }
    
    /**
     * Gets direct neighbours of this area
     * @return integer array of direct neighbour areas
     */
    public int[] getNeighbours() {
        return neighbours;
    }
    
    /**
     * Sets player who owns this area
     * @param owner the player that should own this area
     */
    public void setOwner(int owner) {
        this.owner = owner;
    }
    
    /**
     * Sets the amount of infantry directly
     * @param amount the amount of infantry to set this area to
     */
    public void setInfantry(int amount) {
        this.infantry = amount;
    }
    
    
}
