/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package settlersofcatan;

/**
 *
 * @author s148698
 */
public class TradeDeal {
    
    int[] FROM; // what LEAVES FROM the player
    int[] TO; // what is GIVEN TO the player
    int player;
    int giver;
    
    int[] leftover; // any cards that the PLAYER could get rid off, can be used as leverage
    
    public TradeDeal(int player, int giver) {
        FROM = new int[5];
        TO = new int[5];
        
        this.player = player;
        this.giver = giver;
    }
    
    public void addFrom(int what) {
        FROM[what]++;
    }
    
    public void addTo(int what) {
        TO[what]++;
    }
    
    public int getTotal() {
        int sum = 0;
        for(int i = 0; i < 5; i++) {
            sum += TO[i];
        }
        return sum;
    }
    
    public void saveLeftoverResources(int[] res) {
        leftover = res;
    }
    
    public void requestExtra() {
        // requests an extra resource, because the deal is so good
        for(int i = 0; i < leftover.length; i++) {
            if(leftover[i] >= 1) {
                FROM[i]++;
                break;
            }
        }
    }
    
}
