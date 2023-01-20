== TO DO ==
* Instead of PRINTING, save the game data into a list on state, then write that to a LOG FILE
* Card could be a tuple struct => Card(u8, u8, u8) => but I think that's actually bad for performance
* Perhaps it's more useful to go -2,2 and _multiply_ these points by some fixed factor, to exaggerate them
* Print gameplay again
* Create images/video again.
  * For every player, draw the cards they have: as many rectangles (in a column) as the number
  * Outline rectangles to differentiate
  * Draw top card in center (the back)
  * Put number for score to the left of the row of cards
  * Put the assigned RATING at fixed locations around center
  * ROTATE the side players this time.
  * For each turn, make multiple drawings
    * The score they assign to every player
    * Reveal the card + SCORE/STEAL + what the board looks like in the end



Use gameplay and video to check if all the rules are actually followed

==== STRATEGIES ====

Every strategy is NUMERICAL. Usually, they range from -X to X. 
The value 0 is essentially a "pass": do not do anything with this strategy

Most (rating) strategies can be doubled in the simulation: one for rating _other players_, one for rating _yourself_


IDEAS:
* One override strategy: Pass (very likely), Random, Always Steal, Always Score
* Points if player has a _significant_ lead
* Add as many points as the biggest stack (just a 0/1 binary choice)
* Something for matching _two_ colors?