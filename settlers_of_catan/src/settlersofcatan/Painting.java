
package settlersofcatan;

import java.awt.BasicStroke;
import java.awt.Color;
import java.awt.Font;
import java.awt.Graphics;
import java.awt.Graphics2D;
import java.awt.Polygon;
import java.awt.image.BufferedImage;
import java.io.File;
import java.io.IOException;
import static java.lang.Math.round;
import java.util.ArrayList;
import java.util.Collections;
import javax.imageio.ImageIO;
import javax.swing.JPanel;

public class Painting extends JPanel {
    
    int curState;
    double paintingWidth;
    double paintingHeight;
    
    /**
     * Constructor that starts at an empty state, and with a white background
     */
    public Painting() {
        curState = -1;
        setBackground(new Color(25, 25, 50));
        
    }
    
    /**
     * Paints this component
     * @param gOld the graphics object to use for painting
     */
    @Override
    public void paintComponent(Graphics gOld) {
        super.paintComponent(gOld);
        Graphics2D g = (Graphics2D) gOld;
        
        // get the width and height of the window (this allows the visualization to work at any damn scale you want)
        paintingWidth = (getSize().width + 0.0);
        paintingHeight = (getSize().height + 0.0);
        
        Color[] playerColors = {new Color(255,100,100), new Color(100,255,100), 
            new Color(100,100,255), new Color(225, 255, 100), 
            new Color(255,100,255), new Color(100,255,255),
            new Color(255,255,255), new Color(100,100,100)};
        
        Color[] streetColors = {new Color(200,50,50), new Color(50,200,50), 
            new Color(50,50,200), new Color(200, 200, 50), 
            new Color(200,50,200), new Color(50, 200, 200),
            new Color(200, 200, 200), new Color(50, 50, 50)};
        
        Color[] tileColors = {new Color(50, 150, 50), new Color(100,200,100),
            new Color(200, 200, 100), new Color(255,150,50), new Color(50,50,50)};
        
        Board BOARD = SettlersOfCatan.BOARD;
        
        Field[][][] FIELDS = BOARD.getAllFields();

        int polSize = 150;
        int radius = (int) round(polSize * (50.0/87.0));
        
        for(int a = 0; a < FIELDS.length; a++) { 
            for(int b = 0; b < FIELDS[a].length; b++) {
                for(int c = 0; c < FIELDS[a][b].length; c++){
                    Field tempF = FIELDS[a][b][c];
                    if(tempF == null) {
                        continue;
                    }
                    
                    Polygon p = new Polygon();
                    double cX = paintingWidth*0.5 + tempF.i * (polSize * 76.0/87.0);
                    double cY = paintingHeight*0.5 + (-0.5 * tempF.i - tempF.j) * polSize;
                    for (int i = 0; i < 6; i++)
                        p.addPoint((int) (cX + radius * Math.cos(i * 2 * Math.PI / 6)),
                          (int) (cY + radius * Math.sin(i * 2 * Math.PI / 6))); 
                    
                    // set color based on tile type
                    Color col = tileColors[tempF.tileType];
                    
                    g.setColor(col);
                    
                    g.fillPolygon(p); 
                    
                    g.setColor(new Color(255, 255, 255));
                    
                    // to get current font: g.getFont().getFontName()
                    // DICE NUMBERS:
                    g.setFont(new Font("TimesRoman", Font.PLAIN, 32)); 
                    g.drawString("" + tempF.diceNum, (int) cX - 8, (int) cY + 8);
                    
                    
                    //g.setColor(new Color(255,255,255));
                    //g.drawString(tempF.i + "," + tempF.j + "," + tempF.k, (int) cX, (int) cY);
                }
            }
        }
        
        Edge[][][][] EDGES = BOARD.getAllEdges();
        Vertex[][][][] VERTICES = BOARD.getAllVertices();
        
         for(int a = 0; a < EDGES.length; a++) { 
            for(int b = 0; b < EDGES[a].length; b++) {
                for(int c = 0; c < EDGES[a][b].length; c++){
                    for(int d = 0; d < 3; d++) {
                        Edge tempE = EDGES[a][b][c][d];  
                        
                        if(tempE == null || !tempE.isOccupied()) {
                            continue;
                        }
                        
                        
                        double cX = paintingWidth*0.5 + tempE.i * (polSize * 76.0/87.0);
                        double cY = paintingHeight*0.5 + (-0.5 * tempE.i - tempE.j) * polSize;
                        
                        int DIR = tempE.edgeLoc + 3;
                        
                        double[] startPoint = {(cX + radius * Math.cos(DIR * 2 * Math.PI / 6)),
                          (cY + radius * Math.sin(DIR * 2 * Math.PI / 6))}; 
                        
                        DIR++;
                        double[] endPoint = {(cX + radius * Math.cos(DIR * 2 * Math.PI / 6)),
                          (cY + radius * Math.sin(DIR * 2 * Math.PI / 6))};
                        
                        double[] vector = {(endPoint[0] - startPoint[0])*0.2, (endPoint[1] - startPoint[1])*0.2};
                        
                        g.setStroke(new BasicStroke(13));
                        g.setColor(streetColors[tempE.getOwner()]);
                        
                        g.drawLine((int) round(startPoint[0] + vector[0]), (int) round(startPoint[1] + vector[1]), 
                                (int) round(endPoint[0] - vector[0]), (int) round(endPoint[1] - vector[1]));
                        
                        //g.setFont(new Font("TimesRoman", Font.PLAIN, 12));         
                        //g.drawString("(" + tempE.i + "," + tempE.j + "," + tempE.k + "," + tempE.edgeLoc + ")", (int) round((startPoint[0] + endPoint[0])*0.5), (int) round((startPoint[1] + endPoint[1])*0.5));
                        
                        
                        //g.setColor(new Color(255,255,255));
                        //g.drawString("(" + tempE.i + "," + tempE.j + "," + tempE.k + "," + tempE.edgeLoc + ")", startPoint[0], startPoint[1]);
                        
                    }
                }
            }
         }
         
         for(int a = 0; a < VERTICES.length; a++) { 
            for(int b = 0; b < VERTICES[a].length; b++) {
                for(int c = 0; c < VERTICES[a][b].length; c++){
                    for(int d = 0; d < 2; d++) {
                        Vertex tempV = VERTICES[a][b][c][d];
                        
                        if(tempV == null) {
                            continue;
                        }
                        
                        double cX = paintingWidth*0.5 + tempV.i * (polSize * 76.0/87.0);
                        double cY = paintingHeight*0.5 + (-0.5 * tempV.i - tempV.j) * polSize;
                        
                        int DIR = (tempV.edgeLoc+1)*3;
                        
                        int[] startPoint = {(int) (cX + radius * Math.cos(DIR * 2 * Math.PI / 6)),
                          (int) (cY + radius * Math.sin(DIR * 2 * Math.PI / 6))};
                        
                        // FOR TESTING
                        //int rad2 = 10;
                        //g.fillArc((int) round(startPoint[0] - rad2), (int) round(startPoint[1] - rad2), rad2*2, rad2*2, 0, 360);
                        
                        // DRAW HARBOUR (if applicable)
                        if(tempV.isHarbour()) {
                            if(tempV.getHarbour() == -1) {
                                g.setColor(new Color(150, 150, 50));
                            } else {      
                                g.setColor(tileColors[tempV.getHarbour()]);
                            }
                            
                            double[] explodeAway = {startPoint[0] - paintingWidth*0.5, startPoint[1] - paintingHeight*0.5};
                            
                            int rad = 10;
                            g.fillArc((int) round(startPoint[0] - rad), (int) round(startPoint[1] - rad), rad*2, rad*2, 0, 360);
                            
                            // if somebody occupies the harbour, draw a ship nearby
                            if(tempV.isOccupied()) {
                                // bottom
                                g.fillArc((int) round(startPoint[0] - rad + 0.15 * explodeAway[0]), (int) round(startPoint[1] - rad + 0.15*explodeAway[1]), rad*2, rad*2, 180, 180);
                                
                                // pole
                                g.fillRect((int) round(startPoint[0] - rad + 0.15 * explodeAway[0] + rad - 2), (int) round(startPoint[1] - rad + 0.15*explodeAway[1] - 5), 4, 16);
                                
                                // flag
                                g.fillRect((int) round(startPoint[0] - rad + 0.15 * explodeAway[0] + rad - 2), (int) round(startPoint[1] - rad + 0.15*explodeAway[1] - 5), 16, 10);
                                
                            }
                            
                            
                            /*
                            g.setStroke(new BasicStroke(2));
                            g.setColor(new Color(100, 100, 20));
                            g.drawArc((int) startPoint[0] - rad, (int) startPoint[1] - rad, rad*2, rad*2, 0, 360);
                            */
                            
                        }
                        
                        if(tempV == null || !tempV.isOccupied()) {
                            continue;
                        }

                        
                        g.setColor(playerColors[tempV.getOwner()]);
                        Polygon p = new Polygon();
                        // small village
                        int width = 15;
                        int height = 9;
                        if(tempV.building == 0) {
                            p.addPoint(startPoint[0] - width, startPoint[1] + height);
                            p.addPoint(startPoint[0] + width, startPoint[1] + height);
                            
                            p.addPoint(startPoint[0] + width, startPoint[1] - height);
                            p.addPoint(startPoint[0], (int) round(startPoint[1] - height - height*2));
                            p.addPoint(startPoint[0] - width, startPoint[1] - height);
                        } else if(tempV.building == 1) {
                            p.addPoint(startPoint[0] - 2*width, startPoint[1] + height);
                            p.addPoint(startPoint[0] + width, startPoint[1] + height);
                            
                            p.addPoint(startPoint[0] + width, (int) round(startPoint[1] - height*1.5));
                            p.addPoint(startPoint[0], (int) round(startPoint[1] - height - height*2.5));
                            p.addPoint(startPoint[0] - width, (int) round(startPoint[1] - height*1.5));
                            p.addPoint(startPoint[0] - 2*width, (int) round(startPoint[1] - height*1.5));
                        }
                        g.fillPolygon(p);
                        
                        g.setStroke(new BasicStroke(2));
                        g.setColor(streetColors[tempV.getOwner()]);
                        g.drawPolygon(p);
                    }
                }
            }
         }
         
        // Draw the robber
        int[] robPos = BOARD.getRobberPos();
        double cX = paintingWidth*0.5 + robPos[0] * (polSize * 76.0/87.0);
        double cY = paintingHeight*0.5 + (-0.5 * robPos[0] - robPos[1]) * polSize;
        g.setColor(new Color(0,0,0));
        int rad = 10;
        g.fillArc((int) cX - rad, (int) cY - rad, rad*2, rad*2, 0, 360);
         
        // Draw info on players
        Player[] ALL_PLAYERS = SettlersOfCatan.PLAYERS;
        int amountPlayers = ALL_PLAYERS.length;
        
        int handWidth = 150;
        int handHeight = 130;
        int cardWidth = 80;
        int[][] playerHandPos = new int[amountPlayers][2];
        playerHandPos[0] = new int[]{20, 20};
        playerHandPos[1] = new int[]{(int) paintingWidth - 20 - handWidth - cardWidth, 20};
        playerHandPos[2] = new int[]{(int) paintingWidth - 20 - handWidth - cardWidth, (int) paintingHeight - 20 - handHeight};
        playerHandPos[3] = new int[]{20, (int) paintingHeight - 20 - handHeight};
        
        // Draw player cards
        for(int i = 0; i < amountPlayers; i++) {
            Player p = ALL_PLAYERS[i];
            
            int initial = handHeight;
            int offset = 40;
            if(i >= 2) {
                offset = -40;
                initial = 25;
            } 
            
            g.setColor(playerColors[i]);
            g.setFont(new Font("FF Quadraat", Font.PLAIN, 32)); 
            g.drawString("PLAYER " + i + "   (" + p.getScore() + ")", (int) round(playerHandPos[i][0]), playerHandPos[i][1] + initial + offset);
            
            g.setColor(streetColors[i]);
            if(BOARD.largestRouteOwner == p.num) {
                g.setColor(new Color(200,200,200));
            }
            g.drawString(" + " + p.largeRoute + " ROUTE", (int) round(playerHandPos[i][0]), playerHandPos[i][1] + initial + offset*2);
            
            g.setColor(streetColors[i]);
            if(BOARD.largestKnighthoodOwner == p.num) {
                g.setColor(new Color(200,200,200));
            }
            g.drawString(" + " + p.getKnights() + " KNIGHTS", (int) round(playerHandPos[i][0]), playerHandPos[i][1] + initial + offset*3);
            
            if(p.getEnemy() >= 0) {
                g.setColor(streetColors[i]);
                g.drawString(" + ", (int) round(playerHandPos[i][0]), playerHandPos[i][1] + initial + offset*4);

                g.setFont(new Font("FF Quadraat", Font.PLAIN, 16)); 
                g.drawString("hates Player " + p.getEnemy(), (int) round(playerHandPos[i][0]) + 38, playerHandPos[i][1] + initial + offset*4 - 6);
            }

            
            if(p.countTotal(p.cards) > 0) {
                int counter = 0;
                float increment = handWidth / p.countTotal(p.cards);
                for(int j = 0; j < 5; j++) {
                    for(int k = 0; k < p.cards[j]; k++) {
                        g.setColor(tileColors[j]);
                        g.fillRect((int) round(playerHandPos[i][0] + increment*counter), playerHandPos[i][1], cardWidth, handHeight);

                        g.setColor(new Color(15,15,15));
                        g.drawRect((int) round(playerHandPos[i][0] + increment*counter), playerHandPos[i][1], cardWidth, handHeight);
                        counter++;
                    }

                }
            }
            
        }
        
        // Tell us what the current player has done!
        Player curPlayer = BOARD.getCurrentPlayer();
        g.setColor(new Color(225, 225, 225));
        g.setFont(new Font("TimesRoman", Font.PLAIN, 12)); 
        ArrayList<String> msg = curPlayer.getTurnMessage();
        for(int i = 0; i < msg.size(); i++) {
           g.drawString(msg.get(i), 20, playerHandPos[0][1] + 350 + i*14);
        }

        
    }
    
    private String expandCards(ArrayList<Integer> c) {
        return Collections.frequency(c, 0) + " | " + 
                Collections.frequency(c, 1) + " | " + 
                Collections.frequency(c, 2) + " | " + 
                Collections.frequency(c, 3) + " | " + 
                Collections.frequency(c, 4);
    }
    
    /**
     * Converts X position of unit to actual position in pixels
     */
    private int scalePos(int x) {
        return (int) Math.round(x * paintingWidth);
    }
    
    /**
     * Paints the state you give it
     * @param state the system state you want painted
     */
    public void paintState(int state) {
        curState = state;
        
        // WHEN TESTING - don't create new images all the time!
        // TO DO: remove in final
        // curState = 0;
               
        // draw everything, and automatically save it in an image
        BufferedImage bImg = new BufferedImage(getSize().width, getSize().height, BufferedImage.TYPE_INT_ARGB);
        Graphics2D cg = bImg.createGraphics();
        paintComponent(cg);
        try {
            if(ImageIO.write(bImg, "png", new File("./output_state_" + convertNum(curState) + ".png"))) {
                System.out.println("Saved Image");
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
        
        // yes, I know, this paints everything twice, but this allows us to automatically see what's going on
        repaint();
    }
    
    /**
     * Paints the previous state
     */
    public void paintPreviousState() {
        curState--;
        if(curState < 0) {
            curState = 0;
        }
        repaint();
    }
    
    /**
     * Paints the next state
     */
    public void paintNextState() {
        curState++;
        repaint();
    }
    
    /**
     * Puts leading zeroes in front of a number, for clarity
     * @param n the number to convert
     * @return the number with leading zeroes
     */
    public String convertNum(int n) {
        String s = n + "";
        if(n < 10) {
            s = "0"+s;
        }
        if(n < 100) {
            s = "0"+s;
        }
        return s;
    }
}

