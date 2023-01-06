/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package settlersofcatan;

import static java.lang.Math.pow;
import static java.lang.Math.round;
import static java.lang.Math.sqrt;
import java.util.Random;
import statistics.DiscreteUniformDistribution;
import statistics.Distribution;

/**
 *
 * @author s148698
 */
public class Vertex {
    
    int edgeLoc;
    int i;
    int j;
    int k;
    int building;
    private int owner;
    
    int harbour;

    public Vertex(int i, int j, int k, int edgeLoc) {
        this.i = i;
        this.j = j;
        this.k = k;
        this.edgeLoc = edgeLoc;
        
        this.building = -1;
        this.owner = -1;
        this.harbour = -2;
    }
    
    public void build(int what, int who) {
        building = what;
        owner = who;
    }
    
    public int getType() {
        return building;
    }
    
    public boolean isHarbour() {
        return (harbour > -2);
    }
    
    public int getHarbour() {
        return harbour;
    }
    
    public void setHarbour(int val) {
        harbour = val;
    }
    
    public void upgrade() {
        building = 1;
    }
    
    public boolean isOwner(int who) {
        return (owner == who);
    }
    
    public int[] getVertex(int which) {
        int[] arr = new int[4];
        
        Random rng = new Random();
        Distribution dist = new DiscreteUniformDistribution(0, 2, rng);

        int rand = which;
        if(which < 0) {
            rand = (int) dist.nextRandom();
        }
        
        switch(rand) {
            case 0:
                // UP
                // If left => get upper left hexagon, use R
                // If right => get upper right hexagon, use L
                if(edgeLoc == 0) {
                    arr = new int[]{(i - 1), (j + 1), k, 1};
                } else {
                    arr = new int[]{(i + 1), j, (k - 1), 0};
                }
                break;
            
            case 1:
                // SIDE
                // If left => get far left hexagon, use R
                // If right => get far right hexagon, use L
                if(edgeLoc == 0) {
                    arr = new int[]{(i - 2), (j + 1), (k + 1), 1};
                } else {
                    arr = new int[]{(i + 2), (j - 1), (k - 1), 0};
                }
                break;
                
            case 2:
                // DOWN
                // If left => get lower left hexagon, use R
                // If right => get lower right hexagon, use L
                if(edgeLoc == 0) {
                    arr = new int[]{(i - 1), j, (k + 1), 1};
                } else {
                    arr = new int[]{(i + 1), (j - 1), k, 0};
                }
                break;
                
            default:
                break;
        }
        
        return arr;
    }
    
    public int[] getEdge(int which) {
        int[] arr = new int[4];
        
        Random rng = new Random();
        Distribution dist = new DiscreteUniformDistribution(0, 2, rng);

        int rand = which;
        if(which < 0) {
            rand = (int) dist.nextRandom();
        }

        switch (rand) {
            case 0:
                // UP
                // If left => get own hexagon, use W
                // If right => get own hexagon, use E
                if(edgeLoc == 0) {
                    arr = new int[]{i, j, k, 0};
                } else {
                    arr = new int[]{i, j, k, 2};
                }   
                break;
                
            case 1:
                // SIDE
                // If left => get lower left hexagon, use N
                // If right => get lower right hexagon, use N
                if(edgeLoc == 0) {
                    arr = new int[]{(i - 1), j, (k + 1), 1};
                } else {
                    arr = new int[]{(i + 1), (j - 1), k, 1};
                }   
                break;
                
            case 2:
                // DOWN
                // If left => get lower left hexagon, use E
                // If right => get lower right hexagon, use W
                if(edgeLoc == 0) {
                    arr = new int[]{(i - 1), j, (k + 1), 2};
                } else {
                    arr = new int[]{(i + 1), (j + 1), k, 0};
                }   
                break;
                
            default:
                break;
        }
        
        return arr;
    }
    
    public boolean isAllowed(int whoAsks) {
        // check if this one is currently occupied
        if(isOccupied()) {
            return false;
        }
        
        Board b = SettlersOfCatan.BOARD;
        // check if a neighboring edge is occupied
        // for some reason, I thought it mattered if the edge was your own or not - it doesn't
        /*
        for(int a = 0; a < 3; a++) {
            Edge e = b.getEdge(getEdge(a));
            if(e == null) {
                continue;
            }
            
            if(e.isOccupied() && !e.isOwner(whoAsks)) {
                return false;
            }
        }*/
        
        // check if a neighboring vertex is occupied
        for(int a = 0; a < 3; a++) {
            Vertex v = b.getVertex(getVertex(a));
            if(v == null) {
                continue;
            }
            
            if(v.isOccupied()) {
                return false;
            }
        }
        
        
        return true;
    }
    
    public int checkField(Field f, int checkNum) {
        if(f == null) {
            return  -1;
        }
        
        if(checkNum >= 2 && checkNum != f.getDiceNum()) {
            return -1;
        }
        
        int[] robPos = SettlersOfCatan.BOARD.getRobberPos();
        if(f.i == robPos[0] && f.j == robPos[1] && f.k == robPos[2]) {
            return -1;
        }
        
        return f.getTileType();
    }
    
    public int getNumFields() {
        // a bit of a hacky way to do it, but hey
        int[] arr = getResources(-1);
        int count = 0;
        for(int i = 0; i < arr.length; i++) {
            if(arr[i] >= 0) {
                count++;
            }
        }
        return count;
    }
    
    public int[] getFieldByNum(int num) {
        switch(num) {
            case 0:
                // our own field
                return new int[]{i,j,k};
            
            case 1:
                // upper field
                if(edgeLoc == 0) {
                    return new int[]{(i - 1), (j + 1), k};
                } else {
                    return new int[]{(i + 1), j, (k - 1)};
                }
                
            case 2:
                // lower field
                if(edgeLoc == 0) {
                    return new int[]{(i - 1), j, (k + 1)};
                } else {
                    return new int[]{(i + 1), (j - 1), k};
                }
                
        }
        // default return value
        return new int[]{i, j, k};
    }
    
    public int[] getResources(int checkNum) {
        int[] arr = new int[3];
        Board b = SettlersOfCatan.BOARD;
        // go through all three fields connected by this vertex

        // our own field
        Field f = b.getField(new int[]{i,j,k});
        arr[0] = checkField(f, checkNum);
        
        // upper field
        if(edgeLoc == 0) {
            f = b.getField(new int[]{(i - 1), (j + 1), k});
        } else {
            f = b.getField(new int[]{(i + 1), j, (k - 1)});
        }
        arr[1] = checkField(f, checkNum);
        
        // lower field
        if(edgeLoc == 0) {
            f = b.getField(new int[]{(i - 1), j, (k + 1)});
        } else {
            f = b.getField(new int[]{(i + 1), (j - 1), k});
        }
        arr[2] = checkField(f, checkNum);
        
        return arr;
    }
    
    public int getResourceScore() {
        int[] arr = getResources(-1);
        boolean wood = false;
        boolean clay = false;
        int score = 0;
        
        for(int i = 0; i < arr.length; i++) {
            int val = arr[i];
            if(val == 0) {
                wood = true;
                score += 3;
            } else if(val == 3) {
                clay = true;
                score += 3;
            } else if(val == 1 || val == 2) {
                score += 2;
            } else {
                score += 1;
            }
        }
        
        if(wood && clay) {
            return score;
        }
        return 0;
    }
    
    public boolean isOccupied() {
        return (owner >= 0);
    }
    
    public int getOwner() {
        return owner;
    }
    
    public double getDistanceTo(Vertex target) {
        double[] myPos = new double[]{i - 0.5 + edgeLoc, j, k};
        double[] targetPos = new double[]{target.i - 0.5 + target.edgeLoc, target.j, target.k};
        
        double distance = sqrt(pow(myPos[0] - targetPos[0], 2) + pow(myPos[1] - targetPos[1], 2) + pow(myPos[2] - targetPos[2], 2));
        
        return distance;
    }
    
}
