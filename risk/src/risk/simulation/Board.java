/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

package risk.simulation;

/**
 *
 * @author s148698
 */
public class Board {
    
    // The names of all the areas
    public static String[] AREA_NAMES = 
    {
        "Alaska", // 0
        "Northwest Territory", // 1
        "Alberta", // 2
        "Ontario", // 3
        "Quebec", // 4
        "Western United States", // 5
        "Eastern United States", // 6
        "Central America", // 7
        "Greenland", // 8
        
        "Venezuela", // 9
        "Peru", // 10
        "Brazil", // 11
        "Argentina", // 12
        
        "North Africa", // 13
        "Congo", // 14
        "South Africa", // 15
        "Madagascar", // 16
        "East Africa", // 17
        "Egypt", // 18
        
        "Western Europe", // 19
        "Southern Europe", // 20
        "Northern Europe", // 21
        "Great Britain", // 22
        "Iceland", // 23
        "Scandinavia", // 24
        "Russia", // 25
        
        "Middle East", // 26
        "Afghanistan", // 27
        "Ural", // 28
        "Siberia", // 29
        "Yakutsk", // 30
        "Kamchatka", // 31
        "Irkutsk", // 32
        "Mongolia", // 33
        "Japan", // 34
        "China", // 35
        "India", // 36
        "Siam", // 37
        
        "Indonesia", // 38
        "New Guinea", // 39
        "Western Australia", // 40
        "Eastern Australia", // 41
    };
    
    public static String[] CONTINENT_NAMES = 
    {
        "North America",
        "South America",
        "Africa",
        "Europe",
        "Asia",
        "Australia"
    };
    
    // To which continent each area belongs
    public static int[] CONTINENTS = 
    {
        0, 0, 0, 0, 0, 0, 0, 0, 0, // North America
        1, 1, 1, 1, // South America
        2, 2, 2, 2, 2, 2, //Africa
        3, 3, 3, 3, 3, 3, 3, // Europe
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // Asia
        5, 5, 5, 5 // Australia
    };
    
    // How many areas each continent contains (to make calculations easier)
    public static int[] AREAS_PER_CONTINENT = {9, 4, 6, 7, 12, 4};
    
    // How many extra armies you get for owning a continent
    public static int[] ARMIES_PER_CONTINENT = {5, 2, 3, 5, 7, 2};
    
    // The game board, characterized by which area connects with which other area via a direct link
    public static int[][] GAME_BOARD = 
    {
        {1, 2, 31}, // ALASKA
        {0, 2, 3, 8},
        {0, 1, 3, 5},
        {1, 2, 4, 5, 6, 8},
        {3, 6, 8},
        {2, 3, 6, 7},
        {3, 4, 5, 7},
        {5, 6, 9},
        {1, 3, 4, 23}, // GREENLAND
        
        {7, 10, 11},
        {9, 11, 12},
        {9, 10, 12, 13},
        {10, 11},
        
        {11, 14, 18, 19, 20},
        {13, 15, 17},
        {14, 16, 17},
        {15, 17},
        {13, 14, 15, 16, 18},
        {13, 17, 20, 26},
        
        {13, 20, 21},
        {18, 19, 21, 25, 26},
        {19, 20, 22, 24, 25},
        {21, 23, 24},
        {8, 22, 24},
        {21, 22, 23, 25},
        {21, 24, 26, 27, 28}, //RUSSIA
            
        {18, 20, 25, 27},
        {25, 26, 28, 35, 36},
        {25, 27, 29, 35},
        {28, 30, 32, 33, 35},
        {29, 31, 32},
        {0, 30, 32, 33, 34},
        {29, 30, 31, 33},
        {29, 31, 32, 34},
        {33, 31},
        {27, 28, 29, 33, 36, 37},
        {26, 27, 35, 37},
        {35, 36, 38}, // SIAM
        
        {37, 39, 40},
        {38, 40, 41},
        {38, 39, 41},
        {39, 40}     
    };
}
