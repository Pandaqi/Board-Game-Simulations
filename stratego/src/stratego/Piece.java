/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package stratego;

import java.util.ArrayList;
import java.util.Random;

/**
 *
 * @author s148698
 */
public class Piece {
    
    private int value;
    private int[] pos;
    private int owner;
    
    private ArrayList<int[]> moves;
    private ArrayList<Piece> enemies;
    private int ID;
    
    // water spots have value -1, and that's all we need to know
    public Piece(int value) {
        this.value = value;
    }
    
    public Piece(int value, int[] pos, int owner) {
        this.value = value;
        this.pos = pos;
        this.owner = owner;
        
        this.ID = (new Random()).nextInt(10000);
    }
    
    public int getValue() {
        return value;
    }
    
    public int[] getPos() {
        return pos;
    }
    
    public void setPos(int X, int Y) {
        pos = new int[]{X,Y};
    }
    
    public int getOwner() {
        return owner;
    }
    
    public int getID() {
        return ID;
    }
    
    public boolean isMovable() {
        // flags and bombs aren't movable
        if(getValue() == 11 || getValue() == 0) {
            return false;
        }
        
        // "up", "left", "down", "right"
        int[][] dirs = new int[][]{{0,1},{1,0},{0,-1},{-1,0}};
        
        moves = new ArrayList<int[]>();
        enemies = new ArrayList<Piece>();
        
        // scouts ("verkenners") can move as far as they want in a straight line
        int moveDist = 1;
        if(getValue() == 2) {
            moveDist = 9;
        }
                
        // check all directions; if its off the board, or there's another piece there, continue
        // if not, add it to the possible moves list (we'll use that later when we actually move)
        // also immediately add it to the enemies list (this is all very efficient, I'm proud of myself)
        boolean isMovable = false;
        for(int a = 0; a < 4; a++) {
            for(int b = 1; b <= moveDist; b++) {
                int[] d = dirs[a];

                int tempX = pos[0] + d[0]*b;
                int tempY = pos[1] + d[1]*b;

                if(tempY < 0 || tempY >= 10) {
                    break;
                } else if(tempX < 0 || tempX >= 10) {
                    break;
                } else if(Stratego.BOARD[tempY][tempX] != null) {
                    Piece p = Stratego.BOARD[tempY][tempX];
                    if(p.getValue() != -1 && p.getOwner() != getOwner()) {
                        enemies.add(p);
                        moves.add(new int[]{tempX, tempY});
                        isMovable = true;
                    }
                    break;
                } else {
                    enemies.add(null);
                    moves.add(new int[]{tempX, tempY});
                    isMovable = true;
                }
            }
        }
        
        return isMovable;
    }
    
    public int move() {
        // select random location
        int index = (new Random()).nextInt(moves.size());
        int[] move = moves.get(index);
        
        int result = 0;
        
        // we've encountered an enemy!
        if(enemies.get(index) != null) {
            result = attack(enemies.get(index));
        } else {        
            // also update the board
            // first the new position (otherwise we might lose this one to garbage collection)
            Stratego.BOARD[move[1]][move[0]] = this;
            // then remove the old one
            Stratego.BOARD[pos[1]][pos[0]] = null;

            // go to new location
            pos = move;
        }

        return result;
    }
    
    public int attack(Piece e) {
        int result = 0;
        int val = e.getValue();
        
        if(val == 11) {
            // it's the FLAG
            result = 11;
        } else if(val == 0) {
            // a BOMB destroys us ... 
            // unless we're a miner
            if(getValue() == 3) {
                // THEY ARE DEAD
                Stratego.removePiece(this, e, 0);
            } else {
                // WE ARE DEAD
                Stratego.removePiece(this, e, 1);
            }
        } else if(getValue() == 1 && val == 10) {
            // we are a SPY and we meet a MARSHAL
            // THEY ARE DEAD
            Stratego.removePiece(this, e, 0);
        } else {
            // otherwise, just compare NUMBERS
            if(val > getValue()) {
                // if they have the higher number
                // WE ARE DEAD
                Stratego.removePiece(this, e, 1);
            } else if(val == getValue()) {
                // if we are equal
                // WE ARE DEAD + THEY ARE DEAD
                Stratego.removePiece(this, e, 2);
            } else {
                // if we have the higher number
                // THEY ARE DEAD
                Stratego.removePiece(this, e, 0);
            }
        }
        
        return result;
    }

    /*
               
            switch (d) {
                case 0:
                    vector = new int[]{0,1};
                    break;
                case 1:
                    vector = new int[]{1,0};
                    break;
                case 2:
                    vector = new int[]{0,-1};
                    break;
                default:
                    vector = new int[]{-1,0};
                    break;
            }

            tempY += vector[1];
            tempX += vector[0];
    */
    
}
