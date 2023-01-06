require(rgdal)
require(ggplot2)
require(rgeos)

listOfNames <- c("Alaska", "Northwest Territory", "Alberta", "Ontario", "Quebec", "Western United States", "Eastern United States", "Central America", "Greenland", "Venezuela", "Peru", "Brazil", "Argentina", "North Africa", "Congo", "South Africa", "Madagascar", "East Africa", "Egypt", "Western Europe", "Southern Europe", "Northern Europe", "Great Britain", "Iceland", "Scandinavia", "Russia", "Middle East", "Afghanistan", "Ural", "Siberia", "Yakutsk", "Kamchatka", "Irkutsk", "Mongolia", "Japan", "China", "India", "Siam", "Indonesia", "New Guinea", "Western Australia", "Eastern Australia")

source("simulationResults.R")

# Read SHAPEFILE.shp from the current working directory (".")
map <- readOGR(dsn = ".", layer = "Areas")

centroids.df <- as.data.frame(coordinates(map))
names(centroids.df) <- c("long", "lat")
true_centroids <- gCentroid(map,byid=TRUE)

for(i in 1:length(results)) {
  data <- data.frame(Name = listOfNames,
             R = results[[i]], 
             A = armies[[i]])
  data$player = with(data, R)
  data$armies = with(data, A)
  
  map.data <- data.frame(id = rownames(map@data), Name = map@data$Name)
  map.data <- merge(map.data,data)
  
  map.df   <- fortify(map)
  map.df   <- merge(map.df,map.data)
  
  map.data_copy <- map.data
  names_correct_order <- map.data_copy$armies[order(as.numeric(as.character(map.data_copy$id)), decreasing = FALSE)]
  names_correct_order <- 20 / max(names_correct_order) * names_correct_order
  
  world.label <- data.frame(
    name <- names_correct_order,
    coordinates(map)
  )
  
  temp_plot <- ggplot(map.df,aes(x = long,y = lat,group = group))+
    geom_polygon(aes(fill=player),color="grey20")+labs(x=paste("Turn ", i), y="")+
    scale_fill_gradientn(colours=c("lightgreen", "#FF2222", "#2222FF", "yellow", "orange", "#22AAFF"), limits=c(0,amount_players-1), breaks = 0:(amount_players-1), guide = "legend")+
     geom_point(data = world.label, aes(X1, X2, group = 0), size = name, col = "#444444")+
    geom_text(aes(x = 80, y = 0, label = paste("Strategies used: ", strategies_used, sep = "")))+
    theme(legend.spacing.y = unit(20, "mm"))
    # geom_text(aes(X1, X2, label = name, group = 0), data = world.label)

  temp_plot
  
  ggsave(plot = temp_plot, filename = paste("Plots/plot_turn_", formatC(i, width = 3, flag = "0"), ".png", sep = ""), device = png, width= 1200, height = 675, limitsize = FALSE)
}

# display end results, over the end situation on the board
for(i in length(results):(length(results)+10)) {
  temp_plot <- temp_plot <- ggplot(map.df,aes(x = long,y = lat,group = group))+
    geom_polygon(aes(fill=player, alpha = 0.5),color="grey20")+labs(x="Last Turn", y="")+
    scale_fill_gradientn(colours=c("lightgreen", "#FF2222", "#2222FF", "yellow", "orange", "#22AAFF"), limits=c(0,amount_players-1), breaks = 0:(amount_players-1), guide = "legend")+
    geom_point(data = world.label, aes(X1, X2, group = 0), size = name, col = "#444444")+
    geom_text(aes(x = 50, y = 0, label = paste(winner, " WINS!")))+
    geom_text(aes(x = 80, y = -30, label = paste("Mission: ", mission)))+
    theme(legend.spacing.y = unit(20, "mm"))
  
  ggsave(plot = temp_plot, filename = paste("Plots/plot_turn_", formatC(i+1, width = 3, flag = "0"), ".png", sep = ""), device = png, width= 1200, height = 675, limitsize = FALSE)
}

# FOR CREATING VIDEO: 
# Open cmd line
# Navigate to directory with plots
# Execute this ffmpeg -framerate 18 -i plot_turn_%03d.png output.mp4


# dev.print(file = paste("Plots/plot_turn_", i, ".png", sep = ""), device=png, width=800)


#geom_text(aes(label = Name, x = long, y = lat))


# scale_fill_manual(values = c("green", "red", "blue", "yellow", "orange", "purple"), breaks = c(0.5, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5))

# scale_fill_gradient2(low="red",high="blue",mid="white",limits=c(0, 4))
# scale_fill_gradient2(low="red",high="blue",mid="white",limits=c(0, 4))



