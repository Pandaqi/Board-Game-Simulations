/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package settlersofcatan;

import static java.lang.Math.abs;
import java.util.ArrayList;
import java.util.Random;

/**
 *
 * @author s148698
 */
public class Field {
    
    int tileType;
    int diceNum;
    int i;
    int j;
    int k;
    
    public int probValue;
    
    public Field(int i, int j, int k, int tileType, int diceNum) {
        this.i = i;
        this.j = j;
        this.k = k;
        this.tileType = tileType;
        this.diceNum = diceNum;
        
        probValue = abs(7 - diceNum);
    }
    
    public int getTileType() {
        return tileType;
    }
    
    public int getDiceNum() {
        return diceNum;
    }
    
    // selects one of the six vertices of this field
    public int[] getVertex(int num) {
        int[] arr;
        switch(num) {
            case 0:
                arr = new int[]{i, j, k, 1};
                break;
                
            case 1:
                arr = new int[]{(i + 1), (j - 1), k, 0};
                break;
                
            case 2:
                arr = new int[]{(i - 1), j, (k + 1), 1};
                break;
                
            case 3:
                arr = new int[]{i, j, k, 0};
                break;
            
            case 4:
                arr = new int[]{(i - 1), (j + 1), k, 1};
                break;
                
            case 5:
                arr = new int[]{(i + 1), j, (k - 1), 0};
                break;
            
                // unnecessary default because JAVA keeps pissing me off with warnings
            default:
                arr = new int[]{i, j, k, 1};
        }
        return arr;
    }
    
    public boolean isPlayerAround(int p) {
        Board b = SettlersOfCatan.BOARD;
        for(int i = 0; i < 6; i++) {
            if(b.getVertex(getVertex(i)).getOwner() == p) {
                return true;
            }
        }
        return false;
    }
    
    public int[] getPos() {
        return new int[]{i, j, k};
    }
    
    public int pickVictim(int whoAsks) {
        // go through all vertices of this field
        // and pick a random player
        
        Board b = SettlersOfCatan.BOARD;
        ArrayList<Integer> victims = new ArrayList<Integer>();
        
        // we move clockwise from the middle right, just as Sin/Cos circle
        for(int j = 0; j < 6; j++) {
            Vertex v1 = b.getVertex(getVertex(j));
            if(v1.isOccupied() && !v1.isOwner(whoAsks)) {
                victims.add(v1.getOwner());
            }
        }
        
        if(victims.size() > 0) {
            return victims.get((new Random()).nextInt(victims.size()));
        }
        
        return -1;
        
    }
}
