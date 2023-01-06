
package stratego;

import java.awt.BorderLayout;
import java.awt.Container;
import java.awt.event.ActionListener;
import javax.swing.JButton;
import javax.swing.JFrame;

public class Frame extends JFrame {
    
    Painting panel;
   
    /**
     * Frame constructor; does the default stuff, such as setting size, title, and adding components
     * @param width the width of the window
     * @param height the height of the window
     */
    public Frame(int width, int height) {
        setTitle("Simulation Visualization");
        setSize(width, height);
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        
        Container pane = getContentPane();
        pane.setLayout(new BorderLayout()); // a bit redundant, but whatever
        
        panel = new Painting();
        pane.add(panel, BorderLayout.CENTER);
        
        setVisible(true);
    }
    
    /**
     * Returns the graphics panel within this frame
     * @return the graphics panel within this frame
     */
    public Painting getPainting() {
        return panel;
    }
    
}
