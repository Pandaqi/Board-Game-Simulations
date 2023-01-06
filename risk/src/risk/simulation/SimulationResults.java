/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

package risk.simulation;

import java.io.File;
import java.io.IOException;
import java.io.PrintStream;
import java.util.ArrayList;
import java.util.Arrays;

/**
 *
 * @author s148698
 */
public class SimulationResults {
    
    int amount_sim;
    int amount_players;
    int[] total_armies;
    int[] winners;
    int[] amount_turns;
    int[][] total_ranking;
    int[][][] total_ranking_advanced;
    double[] starting_pos;
    PrintStream fw;
    
    public SimulationResults(int amount_sim, int amount_players) {
        this.amount_sim = amount_sim;
        this.amount_players = amount_players;
        
        total_armies = new int[amount_sim];
        winners = new int[amount_sim];
        amount_turns = new int[amount_sim];
        starting_pos = new double[42];
        total_ranking = new int[amount_players][amount_players];
        total_ranking_advanced = new int[6][amount_players][amount_players];
        
        try { 
            fw = new PrintStream(new File("simulationResults.r"));
        } catch(IOException e){
            System.out.println("I'm afraid your file could not be written at this time.");
        }
        
    }
    
    public void printFile(String text) {
        fw.print(text);
    }
    
    public void printFileLine(String text) {
        fw.println(text);
    }
    
    public void closeFile() {
        fw.close();
    }
    
    public void logCurrentSituation(ArrayList<Area> all_areas, int cur_turn) {
        if(cur_turn == 1) {
           printFileLine("results <- list()");
           printFileLine("armies <- list()");
        }
        
        // Log the current position on the board
        printFile("results[[" + cur_turn + "]] <- c(");
        for(int i = 0; i < all_areas.size(); i++) {
            printFile(Integer.toString(all_areas.get(i).getOwner()));
            if(i != all_areas.size()-1) {
                printFile(", ");
            }
        }
        printFile(")");
        
        printFileLine("");
        
        // Log the amount of armies on each area
        printFile("armies[[" + cur_turn + "]] <- c(");
        for(int i = 0; i < all_areas.size(); i++) {
            printFile(Integer.toString(all_areas.get(i).getInfantry()));
            if(i != all_areas.size()-1) {
                printFile(", ");
            }
        }
        printFile(")");
        
        printFileLine("");
    }
    
    public void displayResults(ArrayList<Area> all_areas) {            
        System.out.println("Average amount of armies: " + getAverageArmies());
        System.out.println("Player with most wins: " + getMostWins());
        System.out.println("Average amount of turns per game: " + getAverageTurns());
        
        System.out.println();
        
        //displayBestStartingPos();
        System.out.println("Winning distribution: " + Arrays.toString(getWinDistribution()));
        
        System.out.println();
        
        displayRankingDistribution();
        //displayRankingDistributionAdvanced();
    }
    
    public void setTotalArmies(int n, int i) {
        total_armies[i] = n;
    }
    
    public void setWinner(int n, int i, Player[] players) {
        winners[i] = n;
        
        if(i == 0) {
            printFileLine("winner <- \"Player " + (n+1) + "\"");
            printFileLine("mission <- \"" + displayMission(players[n]) + "\"");
            printFileLine("");
            closeFile();
        }
    }
    
    public String displayMission(Player p) {
        if(p.getMission().getType() == 0) {
            return "Defeat the army of player " + (p.getMission().getSpecSingle() + 1);
        } else if(p.getMission().getType() == 1) {
            int[] specArr = p.getMission().getSpecArr();
            String temp = "";
            for(int i = 0; i < specArr.length; i++) {
                if(specArr[i] == 1) {
                    temp += Board.CONTINENT_NAMES[i];
                    temp += ", ";
                }
            }
            temp = temp.substring(0, temp.length()-2);
            return "Conquer the continents " + temp;
        } else {
            return "Conquer " + p.getMission().getSpecSingle() + " territories";
        }
    }
    
    public void setAmountTurns(int n, int i) {
        amount_turns[i] = n;
    }
    
    public void setBestStartingPos(int[] arr) {
        for(int i = 0; i < starting_pos.length; i++) {
            starting_pos[i] += arr[i] * (1.0 / amount_sim);
        }
    }
    
    public void displayBestStartingPos() {
        System.out.println("Best starting pos: ");
        for(int i = 0; i < starting_pos.length; i++) {
            System.out.println(" => " + Board.AREA_NAMES[i] + " - " + roundNum(starting_pos[i]));
        }
    }
    
    public void setRanking(Player[] arr) {
        for(int i = 0; i < arr.length; i++) {
            total_ranking[i][arr[i].getNum()]++;
        }
    }
    
    public void setRanking(Player[] arr, int strat) {
        for(int i = 0; i < arr.length; i++) {
            total_ranking_advanced[strat-1][i][arr[i].getNum()]++;
        }
    }
    
    public void displayRankingDistribution() {
        String table = "<table><tr><td></td>";
        for(int i = 0; i < amount_players; i++) {
            table += "<td>Speler " + (i + 1) + "</td>";
        }
        table += "</tr>";
        for(int i = 0; i < total_ranking.length; i++) {
            String temp_stuff = "<tr><td>Positie " + (i + 1) + "</td>";
            for(int j = 0; j < amount_players; j++) {
                temp_stuff += "<td>" + total_ranking[i][j] + "</td>";
            }
            temp_stuff += "</tr>";
            table += temp_stuff;
        }
        table += "</table>";
        System.out.println(table);
    }
    
    public void displayRankingDistributionAdvanced() {
        for(int a = 0; a < total_ranking_advanced.length; a++) {
            System.out.println("Strategie " + (a+1));
            String table = "<table><tr><td></td>";
            for(int i = 0; i < amount_players; i++) {
                table += "<td>Speler " + (i + 1) + "</td>";
            }
            table += "</tr>";
            for(int i = 0; i < total_ranking_advanced[a].length; i++) {
                String temp_stuff = "<tr><td>Positie " + (i + 1) + "</td>";
                for(int j = 0; j < amount_players; j++) {
                    temp_stuff += "<td>" + total_ranking_advanced[a][i][j] + "</td>";
                }
                temp_stuff += "</tr>";
                table += temp_stuff;
            }
            table += "</table>";
            System.out.println(table);
        }
    }
    
    public double roundNum(double n) {
        return Math.round(n * 1000.0) / 1000.0;
    }
    
    public double getAverageArmies() {
        int sum = 0;
        for(int i = 0; i < amount_sim; i++) {
            sum += total_armies[i];
        }
        return 1.0 * sum / amount_sim;
    }
    
    public int getMostWins() {
        return findPopular(winners);
    }
    
    public double[] getWinDistribution() {
        double[] winDist = new double[amount_players];
        for(int i = 0; i < winners.length; i++) {
            winDist[winners[i]] += (1.0 / amount_sim);
        }
        for(int i = 0; i < winDist.length; i++) {
            winDist[i] = roundNum(winDist[i]);
        }
        return winDist;
    }
    
    public double getAverageTurns() {
        int sum = 0;
        for(int i = 0; i < amount_sim; i++) {
            sum += amount_turns[i];
        }
        return 1.0 * sum / amount_sim;
    }
    
    // This is not my code, but from Stack Overflow.
    public int findPopular(int[] a) {

        if (a == null || a.length == 0)
            return 0;

        Arrays.sort(a);

        int previous = a[0];
        int popular = a[0];
        int count = 1;
        int maxCount = 1;

        for (int i = 1; i < a.length; i++) {
            if (a[i] == previous)
                count++;
            else {
                if (count > maxCount) {
                    popular = a[i-1];
                    maxCount = count;
                }
                previous = a[i];
                count = 1;
            }
        }

        return count > maxCount ? a[a.length-1] : popular;

    }
    
}
