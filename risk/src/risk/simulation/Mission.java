/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package risk.simulation;

/**
 *
 * @author s148698
 */
public class Mission {
    
    /**
     * 0 = eliminate a player
     * 1 = conquer specific continents
     * 2 = conquer certain amount of territories
     */
    private int type; 
    private int specSingle;
    private int[] specArr; // array containing specific information (which player to eliminate, which continents to conquer, etc.)
    
    public Mission(int type, int spec) {
        this.type = type;
        this.specArr = new int[] {-1};
        this.specSingle = spec;
    }
    
    public Mission(int type, int[] spec) {
        this.type = type;
        this.specSingle = -1;
        this.specArr = spec;
    }
    
    public int getType() {
        return type;
    }
    
    public int getSpecSingle() {
        return specSingle;
    }
    
    public void setSpecSingle(int newSpec) {
        this.specSingle = newSpec;
    }
    
    public int[] getSpecArr() {
        return specArr;
    }
    
}
