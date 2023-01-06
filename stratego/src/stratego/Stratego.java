/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package stratego;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.Random;
import java.util.concurrent.ThreadLocalRandom;
import statistics.DiscreteUniformDistribution;
import statistics.Distribution;

/**
 *
 * @author s148698
 */
public class Stratego {
    
      // Implementing random array shuffle algorithm
     // Shuffles "in place" => so the array is altered, and this function doesn't return anything
      static void shuffleArray(int[] ar)
      {
        // If running on Java 6 or older, use `new Random()` on RHS here
        Random rnd = ThreadLocalRandom.current();
        for (int i = ar.length - 1; i > 0; i--)
        {
          int index = rnd.nextInt(i + 1);
          // Simple swap
          int a = ar[index];
          ar[index] = ar[i];
          ar[i] = a;
        }
      }
      
    static Piece[][] BOARD;
    static ArrayList<ArrayList<Piece>> PIECES;
    static int[][] LAST_ACTION;
    
    boolean CREATE_VISUALS = true;
    
    // 4 rows, 10 columns, 12 distinct pieces)
    float[][][] boardResults = new float[4][10][12];
    
    void performSimulation(int numSims) {
        // array holding all the pieces (in their correct amounts)
        // before placing these pieces, this array is always sorted randomly
        int[] allPieces = new int[]{0,0,0,0,0,0,1,2,2,2,2,2,2,2,2,3,3,3,3,3,4,4,4,4,5,5,5,5,6,6,6,6,7,7,7,8,8,9,10,11};
        
        // perform "numSims" amount of simulations
        for(int i = 0; i < numSims; i++) {
            Painting PAINTING = null;
            if(CREATE_VISUALS) {
                Frame frame = new Frame(1500, 900);
                PAINTING = frame.getPainting();
            }
            
            // create an empty game board (10 x 10)
            BOARD = new Piece[10][10];
            
            // remove the unavailable "water spots" in the centre
            // these are designated with a "-1"
            BOARD[4][2] = new Piece(-1);
            BOARD[4][3] = new Piece(-1);
            BOARD[5][2] = new Piece(-1);
            BOARD[5][3] = new Piece(-1);
            
            BOARD[4][6] = new Piece(-1);
            BOARD[4][7] = new Piece(-1);
            BOARD[5][6] = new Piece(-1);
            BOARD[5][7] = new Piece(-1);
            
            int[][] startingSetup = new int[2][40];
            PIECES = new ArrayList<ArrayList<Piece>>();
            PIECES.add(new ArrayList<Piece>());
            PIECES.add(new ArrayList<Piece>());
            
            /***** PLAYER B (opposite the table) *****/
            // SHUFFLE all the pieces
            shuffleArray(allPieces);
            
            // Save this setup (for result gathering)
            System.arraycopy(allPieces, 0, startingSetup[1], 0, allPieces.length);
            
            // randomly throw down pieces
            int counter = 0;
            for(int j = 0; j < 4; j++) {
                for(int k = 0; k < 10; k++) {
                    // create new piece; add it to the board
                    Piece np = new Piece(allPieces[counter], new int[]{k, j}, 1);
                    BOARD[j][k] = np;
                    PIECES.get(1).add(np);
                    // add new piece to this player's list of pieces
                    counter++;
                }
            }
            
            
            /***** PLAYER A (close to us; we're "playing" player A) *****/
            // SHUFFLE all the pieces
            shuffleArray(allPieces);
            
            // Save this setup (for result gathering)
            System.arraycopy(allPieces, 0, startingSetup[0], 0, allPieces.length);
            
            // randomly throw down pieces for player B (opposite the table)
            counter = 0;
            for(int j = 6; j < 10; j++) {
                for(int k = 0; k < 10; k++) {
                    Piece np = new Piece(allPieces[counter], new int[]{k, j}, 0);
                    BOARD[j][k] = np;
                    PIECES.get(0).add(np);
                    counter++;
                }
            }
            
            /****** PLAY THE GAME *****/            
            int playerWon = -1;
            int curPlayer = 0;
            int turnCount = 0;
            while(playerWon < 0) {
                // go through all pieces, and check if they're movable
                ArrayList<Piece> tempPieces = new ArrayList<Piece>();
                for(int a = 0; a < PIECES.get(curPlayer).size(); a++) {
                    Piece p = PIECES.get(curPlayer).get(a);
                    if(p.isMovable()) {
                        tempPieces.add(p);
                    }
                }
                
                // if no piece is movable => this player lost!
                if(tempPieces.size() <= 0) {
                    playerWon = (curPlayer + 1) % 2;
                    //System.out.println("Player " + curPlayer + " ran out of moves ...");
                    break;
                }
                
                // find one of our pieces
                // check if it's movable
                Piece getPiece = tempPieces.get((new Random()).nextInt(tempPieces.size()));
                
                // move it
                int result = getPiece.move();
                if(getPiece != null) {
                    LAST_ACTION = new int[][]{getPiece.getPos()};
                }
                
                // if we attacked the flag, we won!
                if(result == 11) {
                    playerWon = curPlayer;
                    //System.out.println("Player " + curPlayer + " found the flag");
                    break;
                }
                
                // give turn to next player
                curPlayer = (curPlayer + 1) % 2;
        
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
                turnCount++;
                //printBoard(BOARD);
            }
            
            // ADD winning player's starting position to the results
            for(int a = 0; a < 4; a++) {
                for(int b = 0; b < 10; b++) {
                    int num = startingSetup[playerWon][a*10 + b];
                    if(playerWon == 0) {
                        // just copy the values 
                        boardResults[a][b][num]++;
                    } else if(playerWon == 1) {
                        // for the other player, we need to flip the position (around the X-axis)
                        boardResults[3-a][b][num]++;
                    }
                }
            }
            
            if(i % 10000 == 0) {
                System.out.println("Sim: " + i);
            }

            //printBoard(BOARD);
            //System.out.println("Who won? " + playerWon);
        }
        
        // DISPLAY END RESULTS
        printBoardResults(boardResults);
    }
    
    static void removePiece(Piece a, Piece b, int whoWon) {
        if(whoWon == 2) {
            System.out.println("REmove equals!");
            BOARD[a.getPos()[1]][a.getPos()[0]] = null;
            BOARD[b.getPos()[1]][b.getPos()[0]] = null;
            
            int owner = b.getOwner();
            // remove b from owner's arraylist
            for(int i = 0; i < PIECES.get(owner).size(); i++) {
                if(PIECES.get(owner).get(i).getID() == b.getID()) {
                    PIECES.get(owner).remove(i);
                    break;
                }
            }
            
            owner = a.getOwner();
            // remove a from owner's arraylist
            for(int i = 0; i < PIECES.get(owner).size(); i++) {
                if(PIECES.get(owner).get(i).getID() == a.getID()) {
                    PIECES.get(owner).remove(i);
                    break;
                }
            }
        } else if(whoWon == 0) {
            // if piece a won, piece b is removed, and a takes its place
            BOARD[b.getPos()[1]][b.getPos()[0]] = a;
            BOARD[a.getPos()[1]][a.getPos()[0]] = null;
            
            a.setPos(b.getPos()[0], b.getPos()[1]);
            LAST_ACTION = new int[][]{a.getPos()};
            
            int owner = b.getOwner();
            // remove b from owner's arraylist
            for(int i = 0; i < PIECES.get(owner).size(); i++) {
                if(PIECES.get(owner).get(i).getID() == b.getID()) {
                    PIECES.get(owner).remove(i);
                    break;
                }
            }
        } else if(whoWon == 1) {
            // if piece b won, piece a is removed (nothing moves)
            BOARD[a.getPos()[1]][a.getPos()[0]] = null;
            LAST_ACTION = new int[][]{b.getPos()};
            
            int owner = a.getOwner();
            // remove a from owner's arraylist
            for(int i = 0; i < PIECES.get(owner).size(); i++) {
                if(PIECES.get(owner).get(i).getID() == a.getID()) {
                    PIECES.get(owner).remove(i);
                    break;
                }
            }
        }
    }
    
    public int getPopularElement(float[] a) {
        float val = a[0];
        int key = 0;
        for(int i = 1; i < a.length; i++) {
            if(a[i] > val) {
                val = a[i];
                key = i;
            }
        }
        return key;
    }
    
    void printBoardResults(float[][][] b) {
        // ALTERNATIEF: we gaan per SOORT alle vakjes langs, en bekijken waar deze het beste scoort, en vullen zo de gaten op
        // Bijvoorbeeld: we maken een lijst van alle resultaten voor VERKENNERS, sorteren deze (hoog naar laag), en vullen de eerste 8 vakjes met de verkenner
        String[] names = new String[]{"Bom ", "Spion", "Verk.", "Mineur", "Serg.", "Luit.", "Kap.", "Majoor", "Kol.", "Gen.", "Maars.", "Vlag"};
        
        int[][] finalBoard = new int[4][10];
        int[] amountPerPiece = new int[]{6, 1, 8, 5, 4, 4, 4, 3, 2, 1, 1, 1};
        
        // for each piece
        for(int i = 0; i < 12; i++) {
            // create a list of its results
            int[][] res = new int[40][3];
            
            // go through the board
            for(int j = 0; j < 4; j++) {
                for(int k = 0; k < 10; k++) {
                    // save the position, plus the result
                    res[j*10 + k] = new int[]{j, k, (int) Math.round(b[j][k][i])};
                }
            }
            
            // now sort it from high to low (based on result; third element)
            Arrays.sort(res, new Comparator<int[]>() {
                @Override
                public int compare(final int[] entry1, final int[] entry2) {
                    int val1 = entry1[2];
                    int val2 = entry2[2];
                    return val1 > val2 ? -1 : val1 == val2 ? 0 : 1;
                }
            });
            
            // and pick the highest (for the amount of pieces in the game)
            for(int a = 0; a < amountPerPiece[i]; a++) {
                int y = res[a][0];
                int x = res[a][1];
                finalBoard[y][x] = i;
                
                // also, disable these spots for future pieces
                // if they are at -1, they're certainly not the best
                for(int z = 0; z < 12; z++) {
                    b[y][x][z] = -1;
                }
            }
        }
        
        System.out.println(Arrays.deepToString(finalBoard));
        
        // PRINT!
        for(int i = 0; i < finalBoard.length; i++) {
            for(int j = 0; j < finalBoard[i].length; j++) {
                System.out.print(names[finalBoard[i][j]] + " \t | ");
            }
            System.out.print("\n");
        }
        
        // HTML TABLE!
        System.out.println("<table>");
        for(int i = 0; i < finalBoard.length; i++) {
            System.out.print("<tr>");
            for(int j = 0; j < finalBoard[i].length; j++) {
                System.out.print("<td>" + names[finalBoard[i][j]] + "</td>");
            }
            System.out.print("</tr>\n");
        }
        System.out.println("</table>");
    }
    
    // Printing the current board in a sensible manner
    void printBoard(Piece[][] board) {
        // array to convert a piece's NUMBER to its NAME
        // 0 => bom, 1 => spion, etc. (11 => vlag)
        String[] names = new String[]{"Bom ", "Spion", "Verk.", "Mineur", "Serg.", "Luit.", "Kap.", "Majoor", "Kol.", "Gen.", "Maars.", "Vlag"};
        
        for(int i = 0; i < board.length; i++) {
            for(int j = 0; j < board[i].length; j++) {
                Piece p = board[i][j];
                if(p == null) {
                    System.out.print("________\t| ");
                    continue;
                }
                
                String name;
                if(p.getValue() == -1) {
                    name = "WATER";
                } else {
                    name = names[p.getValue()];
                }
                
                System.out.print(name + "(" + p.getOwner() + ") \t | ");
            }
            System.out.print("\n");
        }
        System.out.println();
    }

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        // TODO code application logic here
        Stratego sim = new Stratego();
        
        int numberSimulations = 1;
        sim.performSimulation(numberSimulations);
    }
    
}
