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
public class BackupCode {
    
    /*
    FIRST VERSION OF STREET/VILLAGE/CITY BUILDING CODE
    
    // If we have the resources for a STREET
            if(p.checkResources(0)) {
                // Find one we can prolong
                
                ArrayList<Edge> prolong;
                for(int i = 0; i < p.streets.size(); i++) {
                    Edge s = p.streets.get(i);
                    
                    int weAreLeft = 0;
                    int weAreRight = 0;
                    
                    prolong = new ArrayList<Edge>();
                    
                    // if at least one of the edges on the left is free
                    // prolong this street, and use that left edge to do so
                    Edge l1 = BOARD.getEdge(s.getEdgeLeft(0));
                    if(l1 != null) {
                        if(!l1.isOccupied()) {
                            prolong.add(l1);
                        } else if(l1.isOwner(curPlayer)) {
                            weAreLeft++;
                        }
                    }
                    
                    Edge l2 = BOARD.getEdge(s.getEdgeLeft(1));
                    if(l2 != null) {
                        if(!l2.isOccupied()) {
                            prolong.add(l2);
                        } else if(l2.isOwner(curPlayer)) {
                            weAreLeft++;
                        }
                    }
                    
                    // check edges on the right as well
                    Edge r1 = BOARD.getEdge(s.getEdgeRight(0));
                    if(r1 != null) {
                        if(!r1.isOccupied()) {
                            prolong.add(r1);
                        } else if(r1.isOwner(curPlayer)) {
                            weAreRight++;
                        }
                    }
                    
                    Edge r2 = BOARD.getEdge(s.getEdgeRight(1));
                    if(r2 != null) {
                        if(!r2.isOccupied()) {
                            prolong.add(r2);
                        } else if(r2.isOwner(curPlayer)) {
                            weAreRight++;
                        }
                    }
                    
                    // if we're already on both sides - this street shouldn't be prolonged
                    if(weAreLeft == 2 && weAreRight == 2) {
                        continue;
                    }
                    
                    // if we already have a street on one side, check if there's a VILLAGE/CITY between streets
                    boolean buildFirst = false;
                    if(weAreLeft > 0) {
                        Vertex v1 = BOARD.getVertex(s.getVertexLeft());
                        if(v1 != null && !v1.isOccupied()) {
                            buildFirst = true;
                            
                            // get the other vertex
                            v1 = BOARD.getVertex(s.getVertexRight());

                            if(v1 != null) {                                
                                if(v1.isAllowed(curPlayer) && p.checkResources(1)) {
                                    v1.build(0, curPlayer);
                                    p.giveBuilding(v1);
                                    p.updateScore(1);
                                    p.removeResources(new int[]{0,1,2,3});
                                }
                            }
                        }
                    }
                    
                    if(weAreRight > 0) {
                        Vertex v1 = BOARD.getVertex(s.getVertexRight());
                        if(v1 != null && !v1.isOccupied()) {
                            buildFirst = true;
                            
                             // get the other vertex
                            v1 = BOARD.getVertex(s.getVertexLeft());

                            if(v1 != null) {
                                if(v1.isOwner(curPlayer)) {
                                    buildFirst = false;
                                }
                                
                                if(v1.isAllowed(curPlayer) && p.checkResources(1)) {
                                    v1.build(0, curPlayer);
                                    p.giveBuilding(v1);
                                    p.updateScore(1);
                                    p.removeResources(new int[]{0,1,2,3});
                                }
                            }
                        }
                    }
                    
                    // if we should build first (as otherwise prolonging makes no sense)
                    // wait on that (for this particular street)
                    // otherwise, CONTINUE should become BREAK
                    if(buildFirst) {
                        continue;
                    }
                    
                    // if there is opportunity to prolong, do so!
                    if(prolong.size() > 0 && p.checkResources(0)) {
                        Edge pickOne = prolong.get((new Random()).nextInt(prolong.size()));
                        
                        pickOne.build(curPlayer);
                        p.giveStreet(pickOne);
                        p.removeResources(new int[]{0,3});
                        break;
                    }
                }
            }
            
            // If we have the resources for a VILLAGE
            if(p.checkResources(1)) {
                 for(int i = 0; i < p.streets.size(); i++) {
                    Edge s = p.streets.get(i);
                    
                    Vertex v1 = BOARD.getVertex(s.getVertexLeft());
                    if(v1 != null) {
                        if(v1.isAllowed(curPlayer)) {
                            v1.build(0, curPlayer);
                            p.giveBuilding(v1);
                            p.removeResources(new int[]{0,1,2,3});
                            p.updateScore(1);
                            break;
                        }
                    }
                    
                    Vertex v2 = BOARD.getVertex(s.getVertexRight());
                    if(v2 != null) {
                        if(v2.isAllowed(curPlayer)) {
                            v2.build(0, curPlayer);
                            p.giveBuilding(v2);
                            p.removeResources(new int[]{0,1,2,3});
                            p.updateScore(1);
                            break;
                        }
                    }
                 }
            }

            // If we have the resources for a CITY...
            if(p.checkResources(2)) {
               for(int i = 0; i < p.buildings.size(); i++) {
                   Vertex b = p.buildings.get(i);
                   if(b.getType() == 0) {
                       b.upgrade();
                       p.giveBuildingUpgrade();
                       p.updateScore(1);
                       p.removeResources(new int[]{2,2,4,4,4});
                       break;
                   }
               } 
            }
    */
}
