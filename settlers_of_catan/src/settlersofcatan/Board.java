/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package settlersofcatan;

import static java.lang.Math.abs;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.Random;
import java.util.concurrent.ThreadLocalRandom;
import statistics.DiscreteUniformDistribution;
import statistics.Distribution;

/**
 *
 * @author s148698
 */
public class Board {
    
    private Field[][][] fields;
    private Edge[][][][] edges;
    private Vertex[][][][] vertices;
    
    private ArrayList<Field> fieldsArr;
    private ArrayList<Edge> edgesArr;
    private ArrayList<Vertex> verticesArr;
    
    private int[] robber = {0,0,0};
    
    private int BOARD_SIZE;
    private int AXIS_SIZE;
    
    Random rng = new Random();
    Distribution randAxis;
    Distribution leftRight;
    
    int largestKnighthood = 0;
    int largestKnighthoodOwner = -1;

    int largestRoute = 0;
    int largestRouteOwner = -1;
    
    ArrayList<Integer> developmentCards;
    
    Player currentPlayer = null;
    
    public Board() {
        // 0 => forest, 1 => grass, 2 => corn, 3 => brick?, 4 => iron?
        int[] tileTypes = {0,0,0,0,1,1,1,1,2,2,2,2,3,3,3,4,4,4};
        shuffleArray(tileTypes);
        
        int[] diceNums = {2,3,3,4,4,5,5,6,6,8,8,9,9,10,10,11,11,12};
        shuffleArray(diceNums);
        
        // populate the development cards
        developmentCards = new ArrayList<Integer>();
        for(int i = 0; i < 20; i++) {
            developmentCards.add(0);
        }
        for(int i = 0; i < 9; i++) {
            developmentCards.add(1);
        }
        for(int i = 0; i < 5; i++) {
            developmentCards.add(2);
        }
        
        // populate the game board
        AXIS_SIZE = 2 + 1;
        BOARD_SIZE = AXIS_SIZE*2 + 1;
        
        randAxis = new DiscreteUniformDistribution(-(AXIS_SIZE-1), (AXIS_SIZE-1), rng);
        leftRight = new DiscreteUniformDistribution(0, 1, rng);
        
        fields = new Field[BOARD_SIZE][BOARD_SIZE][BOARD_SIZE];
        edges = new Edge[BOARD_SIZE][BOARD_SIZE][BOARD_SIZE][3];
        vertices = new Vertex[BOARD_SIZE][BOARD_SIZE][BOARD_SIZE][2];
        
        fieldsArr = new ArrayList<Field>();
        edgesArr = new ArrayList<Edge>();
        verticesArr = new ArrayList<Vertex>();
        
        int counter = 0;
        
        for(int i = -AXIS_SIZE; i <= AXIS_SIZE; i++) {
            for(int j = -AXIS_SIZE; j <= AXIS_SIZE; j++) {
                for(int k = -AXIS_SIZE; k <= AXIS_SIZE; k++) {
                    
                    // Part of the hexagon!
                    // An empty border is also created around the board
                    if((i + j + k == 0) && !(i == 0 && j == 0 && k == 0)) {
                        if((abs(i) <= (AXIS_SIZE-1) && abs(j) <= (AXIS_SIZE-1) && abs(k) <= (AXIS_SIZE-1))) {
                            // create the field
                            Field f = new Field(i, j, k, tileTypes[counter], diceNums[counter]);
                            fields[i+AXIS_SIZE][j+AXIS_SIZE][k+AXIS_SIZE] = f;
                            fieldsArr.add(f);
                            
                            // go around the field, and insert all 6 vertices
                            // a bit overkill (as we're recreating every vertex 3 times now)
                            // but it ensures no vertices are counted which are OUTSIDE of the game board
                            for(int a = 0; a < 6; a++) {
                                int[] vert = f.getVertex(a);
                                
                                // create left/right vertices
                                // 0 => L, 1 => R
                                if(vertices[vert[0]+AXIS_SIZE][vert[1]+AXIS_SIZE][vert[2]+AXIS_SIZE][vert[3]] == null) {
                                    Vertex v = new Vertex(vert[0], vert[1], vert[2], vert[3]);
                                    vertices[vert[0]+AXIS_SIZE][vert[1]+AXIS_SIZE][vert[2]+AXIS_SIZE][vert[3]] = v;
                                    verticesArr.add(v);
                                }
                            }
                    

                            counter++;
                        }
                    }
                                           
                    // create the east/north/west edges 
                    // 0 => W, 1 => N, 2 => E
                    if((i + j + k == 0)) {
                        for(int a = 0; a < 3; a++) {
                            // there are several cases in which an edge should NOT be included
                            if(i == AXIS_SIZE && a != 0) {
                                continue;
                            } else if(i == -AXIS_SIZE && a != 2) {
                                continue;
                            }
                            
                            if(j == -AXIS_SIZE && a == 2) {
                                continue;
                            } else if(j == AXIS_SIZE) {
                                continue;
                            }
                            
                            if(k == -AXIS_SIZE) {
                                continue;
                            } else if(k == AXIS_SIZE && a == 0) {
                                continue;
                            }
                            
                            Edge e = new Edge(i,j,k,a);
                            edges[i+AXIS_SIZE][j+AXIS_SIZE][k+AXIS_SIZE][a] = e;
                            edgesArr.add(e);
                        }
                    }
                    
                }
            }
        }
        
        ArrayList<Integer> harbours = new ArrayList<Integer>();
        harbours.add(0);
        harbours.add(1);
        harbours.add(2);
        harbours.add(3);
        harbours.add(4);
        harbours.add(-1);
        harbours.add(-1);
        harbours.add(-1);
        harbours.add(-1);
        
        int tempCounter = 0;
        for(int i = 0; i < edgesArr.size(); i++) {
            Edge e = edgesArr.get(i);
            
            // if the left or right vertex is from a tile that's outside the board
            // consider this edge for a trading harbour
            Vertex v = getVertex(e.getVertexLeft());
            Vertex v2 = getVertex(e.getVertexRight());

            if(v != null && v2 != null) {
                if((abs(v.i) >= (AXIS_SIZE) || abs(v.j) >= (AXIS_SIZE) || abs(v.k) >= (AXIS_SIZE)) 
                        || (abs(v2.i) >= (AXIS_SIZE) || abs(v2.j) >= (AXIS_SIZE) || abs(v2.k) >= (AXIS_SIZE))) {
                    
                    int bothAtEdge = 0;
                    if(!(abs(v.i) < (AXIS_SIZE-1) && abs(v.j) <= 1) && !v.isHarbour()) {
                        bothAtEdge++;
                    }
                    
                    if(!(abs(v2.i) < (AXIS_SIZE-1) && abs(v2.j) <= 1) && !v2.isHarbour()) {
                        bothAtEdge++;
                    }
                    
                    if(harbours.size() <= 0) {
                        continue;
                    }
                    
                    if(bothAtEdge == 2) {
                        if((tempCounter % 2) == 0) {
                            int randIndex = (new Random()).nextInt(harbours.size());
                            int harbourType = harbours.get(randIndex);
                            harbours.remove(randIndex);
                            v.setHarbour(harbourType);
                            v2.setHarbour(harbourType);
                        }
                        
                        tempCounter++;
                    }
                    
                    

                }
            }
        }
        
        // SORT the fields so that the best ones come first
        Collections.sort(fieldsArr, new Comparator<Field>(){
            public int compare(Field o1, Field o2){
                if(o1.probValue == o2.probValue) {
                 return 0;
                }
                return o1.probValue < o2.probValue ? -1 : 1;
            }
        });
    }
    
    // Implementing Fisher–Yates shuffle
    private void shuffleArray(int[] ar)
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
    
    public Field getField(int[] field) {
        for(int i = 0; i < 3; i++) {
            if((field[i] + AXIS_SIZE) < 0 || (field[i] + AXIS_SIZE) >= BOARD_SIZE) {
                return null;
            }
        }
        return fields[field[0]+AXIS_SIZE][field[1]+AXIS_SIZE][field[2]+AXIS_SIZE];
    }
    
    
    public Edge getEdge(int[] edge) {
        for(int i = 0; i < 3; i++) {
            if((edge[i] + AXIS_SIZE) < 0 || (edge[i] + AXIS_SIZE) >= BOARD_SIZE) {
                return null;
            }
        }
        
        return edges[edge[0]+AXIS_SIZE][edge[1]+AXIS_SIZE][edge[2]+AXIS_SIZE][edge[3]];
    }
    
    public Vertex getVertex(int[] vertex) {
        for(int i = 0; i < 3; i++) {
            if((vertex[i] + AXIS_SIZE) < 0 || (vertex[i] + AXIS_SIZE) >= BOARD_SIZE) {
                return null;
            }
        }
        
        return vertices[vertex[0]+AXIS_SIZE][vertex[1]+AXIS_SIZE][vertex[2]+AXIS_SIZE][vertex[3]];
    }
    
    public void placeRobber(int[] pos, int whoMovedIt) {
        robber = pos;
        
        // everyone around this field is angry!
        for(int i = 0; i < 6; i++) {
            int owner = getVertex(getField(pos).getVertex(i)).getOwner();
            if(owner >= 0) {
                SettlersOfCatan.PLAYERS[owner].updateRelations(whoMovedIt, -1);
            }
        }
    }
    
    public int[] determineBestRobberPos(int asker, int enemy) {
        for(int i = 0; i < fieldsArr.size(); i++) {
            Field f = fieldsArr.get(i);
            // get who surrounds this field
            if(!f.isPlayerAround(asker)) {
                if(enemy < 0 || f.isPlayerAround(enemy)) {
                    return f.getPos();
                }
            }
        }
        
        // if we get here, it appears we weren't able to find an ideal spot
        // so pick one randomly
        return getRandomFieldPos();
    }
    
    public int[] getRobberPos() {
        return robber;
    }
    
    public int[] getRandomFieldPos() {
        int i, j, k;
        
        do {
            i = (int) randAxis.nextRandom();
            j = (int) randAxis.nextRandom();
            k = 0 - i - j;
        } while(abs(k) > (AXIS_SIZE-1) || (i == 0 && j == 0 && k == 0));
        
        return new int[]{i, j, k};
    }

    public Vertex getRandomVertex() {
        int i;
        int j;
        int k;
        
        do {
            i = (int) randAxis.nextRandom();
            j = (int) randAxis.nextRandom();
            k = 0 - i - j;
        } while(abs(k) > (AXIS_SIZE-1));

        return vertices[i+AXIS_SIZE][j+AXIS_SIZE][k+AXIS_SIZE][(int) leftRight.nextRandom()];
    }

    public Edge[][][][] getAllEdges() {
        return edges;
    }

    public Vertex[][][][] getAllVertices() {
        return vertices;
    }
    
    public Field[][][] getAllFields() {
        return fields;
    }
    
    public void setCurrentPlayer(Player p) {
        currentPlayer = p;
    }
    
    public Player getCurrentPlayer() {
        return currentPlayer;
    }
    
    public Vertex getStartingPosition(int strategy, int player) {
        Vertex v = null;
        switch(strategy) {
            case 1:
                // find the best possible spot (probability-wise)
                // the fieldsArr arraylist is already sorted
                outerLoop:
                for(int i = 0; i < fieldsArr.size(); i++) {
                    Field f = fieldsArr.get(i);
                    // go through all vertices to find one that fits
                    for(int j = 0; j < 6; j++) {
                        v = getVertex(f.getVertex(j));
                        if(v.isAllowed(player)) {
                            break outerLoop;
                        }
                    }
                }
                break;
             
            case 2:
                // use a harbour location
                for(int i = 0; i < verticesArr.size(); i++) {
                    v = verticesArr.get(i);
                    if(v.isHarbour() && v.isAllowed(player)) {
                        break;
                    }
                }
                
                break;
            
            case 3:
                // find a three-way split that touches the most needed resources (wood and clay/brick)
                Vertex bestV = null;
                int bestNum = -1;
                for(int i = 0; i < verticesArr.size(); i++) {
                    Vertex tempV = verticesArr.get(i);
                    if(tempV.isAllowed(player)) {
                        if(tempV.getResourceScore() > bestNum) {
                            bestV = tempV;
                            bestNum = tempV.getResourceScore();
                        }
                    }
                }
                v = bestV;
                break;
            
            case 4:
                // place buildings as far away from other players as possible
                Vertex bestV2 = null;
                double bestNum2 = 0;
                
                Player[] players = SettlersOfCatan.PLAYERS;
                
                int firstPlayer = 0;
                
                for(int i = 0; i < verticesArr.size(); i++) {
                    Vertex tempV = verticesArr.get(i);
                    if(tempV.isAllowed(player)) {
                        double dist = 0;
                        
                        // calculate distance
                        for(int j = 0; j < players.length; j++) {
                            // don't check the current player
                            if(j == player) {
                                continue;
                            }
                            
                            // go through all current buildings (of this player)
                            ArrayList<Vertex> buildings = players[j].buildings;
                            for(int k = 0; k < buildings.size(); k++) {
                                Vertex b = buildings.get(k);
                                dist += b.getDistanceTo(tempV);
                            }
                            
                            if(buildings.size() != 0) {
                                firstPlayer++;
                            }
                        }
                        
                        // check if max
                        if(dist > bestNum2) {
                            bestNum2 = dist;
                            bestV2 = tempV;
                        }
                    }
                }
                v = bestV2;
                
                // if this is the first player
                // pick a random vertex
                if(firstPlayer == 0) {
                    v = verticesArr.get((new Random()).nextInt(verticesArr.size()));
                }
                break;
            
            case 5:
                // place both your own buildings as close to each other as possible
                int amountBuildings = SettlersOfCatan.PLAYERS[player].buildings.size();
                if(amountBuildings == 0) {
                    // just place one randomly
                    boolean spaceOccupied = false;
                    do {
                        v = getRandomVertex();
                        spaceOccupied = !v.isAllowed(player);
                    } while(spaceOccupied);
                } else {      
                    Vertex bestV3 = null;
                    double bestNum3 = 1000;   
                    Vertex myOwnBuilding = SettlersOfCatan.PLAYERS[player].buildings.get(0);
                
                    for(int i = 0; i < verticesArr.size(); i++) {
                        Vertex tempV = verticesArr.get(i);
                        if(tempV.isAllowed(player)) {
                            double dist = 0;
                            dist = myOwnBuilding.getDistanceTo(tempV);
                            
                            if(dist < bestNum3) {
                                dist = bestNum3;
                                bestV3 = tempV;
                            }
                        }
                    }
                    
                    v = bestV3;
                }

                break;
                
            default:
                // if all else fails, build randomly
                boolean spaceOccupied = false;
                do {
                    v = getRandomVertex();
                    spaceOccupied = !v.isAllowed(player);
                } while(spaceOccupied);
        }
        return v;
    }
    
    public int checkStreet(Edge s, int owner, ArrayList<Edge> prevs) {
        // results
        int result1 = 0;
        int result2 = 0;
        int result3 = 0;
        int result4 = 0;
        
        Edge oldStreet = s;
        if(prevs.size() >= 1) {
            oldStreet = prevs.get(prevs.size() - 1);
        }
        
        ArrayList<Edge> newPrevs = new ArrayList<>(prevs);
        newPrevs.add(s);
        
        int dir = -1;
        
        // check first left edge
        Edge l1 = getEdge(s.getEdgeLeft(0));
        if(l1 != null) {
            if(l1.isOwner(owner) && !newPrevs.contains(l1)) {
                result1 = checkStreet(l1, owner, newPrevs);
            }
        }
        
        // check second left edge
        Edge l2 = getEdge(s.getEdgeLeft(1));
        if(l2 != null) {
            if(l2.isOwner(owner) && !newPrevs.contains(l2)) {
                result2 = checkStreet(l2, owner, newPrevs);
            }
        }
        
        // nullify results from direction we came from 
        // (we can't go one direction, and then suddenly turn back another)
        if(oldStreet.streetEquals(l1) || oldStreet.streetEquals(l2)) {
            result1 = 0;
            result2 = 0;
        }
        
        // check first right edge
        Edge r1 = getEdge(s.getEdgeRight(0));
        if(r1 != null) {
            if(r1.isOwner(owner) && !newPrevs.contains(r1)) {
                result3 = checkStreet(r1, owner, newPrevs);
            }
        }
        
        // check second right edge
        Edge r2 = getEdge(s.getEdgeRight(1));
        if(r2 != null) {
            if(r2.isOwner(owner) && !newPrevs.contains(r2)) {
                result4 = checkStreet(r2, owner, newPrevs);
            }
        }
        
        // nullify results again if we came from this side
        if(oldStreet.streetEquals(r1) || oldStreet.streetEquals(r2)) {
            result3 = 0;
            result4 = 0;
        }
        
        // return longest route + "1" length addition for the current street
        int max = Math.max(Math.max(result1, result2), Math.max(result3, result4));
        return 1 + max;

    }
    
}
