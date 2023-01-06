
package stratego;

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
        setBackground(new Color(50, 50, 100));
        
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
        
        Piece[][] b = Stratego.BOARD;
        int SIZE = 80;
        int PADDING = 3;
        int[][] la = null;
        
        if(Stratego.LAST_ACTION != null) {
            la = Stratego.LAST_ACTION;
        }
        
        for(int i = 0; i < b.length; i++) {
            for(int j = 0; j < b[i].length; j++) {
                Piece p = b[i][j];
                
                if(p != null) {
                    if(p.getValue() == -1) {
                        g.setColor(new Color(0,0,255));
                    } else if(p.getOwner() == 0) {
                        g.setColor(new Color(255,0,0));
                    } else if(p.getOwner() == 1) {
                        g.setColor(new Color(50,220,50));
                    }
                    
                    g.fillRect(j*(SIZE+PADDING), i*(SIZE+PADDING), SIZE, SIZE);
                    
                    if(la != null) {
                        for(int a = 0; a < la.length; a++) {
                            if(i == la[a][1] && j == la[a][0]) {
                                g.setColor(new Color(255,255,255));
                                g.setStroke(new BasicStroke(4));
                                g.drawRect(j*(SIZE+PADDING), i*(SIZE+PADDING), SIZE, SIZE); 
                            }
                        }
                    }

                    
                    if(p.getValue() != -1) {
                        g.setColor(new Color(255,255,255));
                        g.setFont(new Font("TimesRoman", Font.PLAIN, 32)); 
                        g.drawString(""+p.getValue(), (int) Math.round((j+0.5)*(SIZE+PADDING)), (int) Math.round((i+0.5)*(SIZE+PADDING)));
                    }
                }

            }
        }
    }
    
    /**
     * Paints the state you give it
     * @param state the system state you want painted
     */
    public void paintState(int state) {
        curState = state;
               
        // draw everything, and automatically save it in an image
        BufferedImage bImg = new BufferedImage(getSize().width, getSize().height, BufferedImage.TYPE_INT_ARGB);
        Graphics2D cg = bImg.createGraphics();
        paintComponent(cg);
        
         // WHEN TESTING - don't create new images all the time!
        // TO DO: remove in final
        if(false) {
            try {
                if(ImageIO.write(bImg, "png", new File("./output_state_" + convertNum(curState) + ".png"))) {
                    System.out.println("Saved Image");
                }
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
        
        // yes, I know, this paints everything twice, but this allows us to automatically see what's going on
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

