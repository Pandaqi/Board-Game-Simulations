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
public class Edge {
    
    int edgeLoc;
    int i;
    int j;
    int k;
    private int owner;
    
    public Edge(int i, int j, int k, int edgeLoc) {
        this.i = i;
        this.j = j;
        this.k = k;
        this.edgeLoc = edgeLoc;

        this.owner = -1;
    }

    public void build(int i) {
        owner = i;
    }
    
    public boolean isOccupied() {
        return (owner >= 0);
    }
    
    public int getOwner() {
        return owner;
    }
    
    public void setOwner(int newOwner) {
        owner = newOwner;
    }
    
    public boolean isOwner(int who) {
        return (owner == who);
    }
    
    public int[] getEdgeLeft(int num) {
        int[] arr = new int[4];
        
        if(num == 0) {
            if(edgeLoc == 0) {
                arr = new int[]{(i - 1), j, (k + 1), 1};
            } else if(edgeLoc == 1) {
                arr = new int[]{(i - 1), (j + 1), k, 2};
            } else if(edgeLoc == 2) {
                arr = new int[]{i, j, k, 1};
            }
        } else if(num == 1) {
            if(edgeLoc == 0) {
                arr = new int[]{(i - 1), j, (k + 1), 2};
            } else if(edgeLoc == 1) {
                arr = new int[]{i, j, k, 0};
            } else if(edgeLoc == 2) {
                arr = new int[]{(i + 1), j, (k - 1), 0};
            }
        }
        
        return arr;
    }
    
    public int[] getEdgeRight(int num) {
        int[] arr = new int[4];
        
        if(num == 0) {
            if(edgeLoc == 0) {
                arr = new int[]{(i - 1), (j + 1), k, 2};
            } else if(edgeLoc == 1) {
                arr = new int[]{i, j, k, 2};
            } else if(edgeLoc == 2) {
                arr = new int[]{(i + 1), (j - 1), k, 0};
            }
        } else if(num == 1) {
            if(edgeLoc == 0) {
                arr = new int[]{i, j, k, 1};
            } else if(edgeLoc == 1) {
                arr = new int[]{(i + 1), j, (k - 1), 0};
            } else if(edgeLoc == 2) {
                arr = new int[]{(i + 1), (j - 1), k, 1};
            }
        }
        
        return arr;
    }
    
    public int[] getVertexLeft() {
        int[] arr = new int[4];
        
        if(edgeLoc == 0) {
            arr = new int[]{i, j, k, 0};
        } else if(edgeLoc == 1) {
            arr = new int[]{(i - 1), (j + 1), k, 1};
        } else if(edgeLoc == 2) {
            arr = new int[]{(i + 1), j, (k - 1), 0};
        }

        return arr;
    }
    
    public int[] getVertexRight() {
        int[] arr = new int[4];
        
        if(edgeLoc == 0) {
            arr = new int[]{(i - 1), (j + 1), k, 1};
        } else if(edgeLoc == 1) {
            arr = new int[]{(i + 1), j, (k - 1), 0};
        } else if(edgeLoc == 2) {
            arr = new int[]{i, j, k, 1};
        }

        return arr;
    }
    
    public int checkLeft() {
        Board b = SettlersOfCatan.BOARD;
        int weAreLeft = 0;
        
        Edge l1 = b.getEdge(getEdgeLeft(0));
        if(l1 != null) {
            if(l1.isOwner(owner)) {
                weAreLeft++;
            }
        }
                    
        Edge l2 = b.getEdge(getEdgeLeft(1));
        if(l2 != null) {
            if(l2.isOwner(owner)) {
                weAreLeft++;
            }
        }
        
        return weAreLeft;
    }
    
    
    public int checkRight() {
        Board b = SettlersOfCatan.BOARD;
        int weAreRight = 0;
        
        Edge l1 = b.getEdge(getEdgeRight(0));
        if(l1 != null) {
            if(l1.isOwner(owner)) {
                weAreRight++;
            }
        }
                    
        Edge l2 = b.getEdge(getEdgeRight(1));
        if(l2 != null) {
            if(l2.isOwner(owner)) {
                weAreRight++;
            }
        }
        
        return weAreRight;
    }
    
    public boolean streetEquals(Edge s) {
        if(s == null) {
            return false;
        }
        
        if(i == s.i && j == s.j && k == s.k && edgeLoc == s.edgeLoc) {
            return true;
        }
        
        return false;
    }
    
    
}
