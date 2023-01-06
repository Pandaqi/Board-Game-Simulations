
package settlersofcatan;

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
                        
        JButton button = new JButton("Go Back");
        pane.add(button, BorderLayout.WEST);
        
        JButton button2 = new JButton("Go Forward");
        pane.add(button2, BorderLayout.EAST);
        
        Event e = new Event(0);
        button.addActionListener(e);
        
        Event e2 = new Event(1);
        button2.addActionListener(e2);
        
        setVisible(true);
    }
    
    /**
     * Returns the graphics panel within this frame
     * @return the graphics panel within this frame
     */
    public Painting getPainting() {
        return panel;
    }
    
    /**
     * Event listener for a click on the button
     * Type = 0 means the "Go Backward" button was pressed
     * Type = 1 means the "Go Forward" button was pressed
     */
    public class Event implements ActionListener {
        
        int type = 0;
        
        public Event(int type) {
            this.type = type;
        }

        @Override
        public void actionPerformed(java.awt.event.ActionEvent e) {
            if(type == 0) {
                getPainting().paintPreviousState();   
            } else if(type == 1) {
                getPainting().paintNextState();
            }
        }

    }
    
}
