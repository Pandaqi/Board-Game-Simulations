/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package settlersofcatan;

import java.util.Arrays;

/**
 *
 * @author s148698
 */
public class Results {
    
    int[] wins;
    int sims;
    int counter = 0;
    double avgTurnCount = 0.0;
    double longRouteCount = 0.0;
    double knightHoodCount = 0.0;
    
    int[][][][][][][] strats;

    Results(int amountSimulations, int amountPlayers) {
        this.sims = amountSimulations;
        this.wins = new int[amountPlayers];
        
        strats = new int[4+1][4+1][4+1][5+1][4+1][4+1][2+1];
    }
    
    public void addGame(int whoWon, int turnCount, boolean winnerHadRoute, boolean winnerHadKnight, int[] sS /* starter stratgy */, int[] wS /* winner strategy*/) {
        wins[whoWon]++;
        avgTurnCount += (0.0 + turnCount) / sims;
        if(winnerHadRoute) { longRouteCount += 1.0/sims; }
        if(winnerHadKnight) { knightHoodCount += 1.0/sims; }
        
        if(wS[1] > 2) {
            wS[1]--;
            if(wS[1] > 5) {
                wS[1]--;
            }
        }
        
        if(wS[6] > 1) {
            wS[6]--;
        }
        
        strats[sS[0]][sS[1]][wS[0]][wS[1]][wS[2]][wS[4]][wS[6]]++;
    }

    void printResults() {
        // double[] correction = new double[]{0.07, -0.14, 0.14, -0.07};
        double[] correction = new double[]{0.0, 0.0, 0.0, 0.0};
        
        System.out.println();
        // wins
        for(int i = 0; i < wins.length; i++) {
            System.out.println("Player " + i + " | " + ((0.0 + wins[i])/(sims + 0.0) + correction[i]));
        }
        System.out.println();
        // game length
        System.out.println("Average game length: " + avgTurnCount + " turns");
        
        System.out.println();
        // what did the winner do right?
        System.out.println("How often did the winner have the longest route? " + longRouteCount);
        System.out.println("How often did the winner have the largest knighthood? " + knightHoodCount);
        
        /*
        int[] bestStrat = new int[]{0,0,0,0,0,0,0};
        int bestNum = -1;
        // THE HUGE 7-DIMENSIONAL LOOP OF RANDOM STRATEGY!
        for(int a = 1; a < strats.length; a++) {
            for(int b = 1; b < strats[a].length; b++) {
                for(int c = 1; c < strats[a][b].length; c++) {
                    for(int d = 1; d < strats[a][b][c].length; d++) {
                        for(int e = 1; e < strats[a][b][c][d].length; e++) {
                            for(int f = 1; f < strats[a][b][c][d][e].length; f++) {
                                for(int g = 1; g < strats[a][b][c][d][e][f].length; g++) {
                                    int val = strats[a][b][c][d][e][f][g];
                                    System.out.println(a + " | " + b + " | " + c + " | " + d + " | " + e + " | " + f + " | " + g + " | => " + val);
                                    
                                    if(val > bestNum) {
                                        bestStrat = new int[]{a,b,c,d,e,f,g};
                                        bestNum = val;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        System.out.println();
        
        System.out.println("BEST STRATEGY: " + Arrays.toString(bestStrat) + "  with a percentage of  " + (0.0 + bestNum)/(0.0 + sims));
        */
    }
    
}
