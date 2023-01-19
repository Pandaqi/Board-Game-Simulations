== TO DO ==
Make _all_ strats numerical (with only 0-5 options in a scale, 0 usually doing nothing)
 => Make this a scale of usize, I can create a general function to divide by maximum and turn into probability

Create an enum for matching the overwriting strats (which don't mess with score) => simply count many values as "Pass" to balance it out (only start enum counting from 5 or something)

Switch to the new strats. 

Calculate all scores first. Then do _another_ loop comparing other scores to yours (for strategy compare)

* Make Card just a tuple? Is that cheaper/faster than a struct? => Could even be a single integer, but that'd need byte trickery



==== STRATEGIES ====

The Rating strategies (tank, card, first, last) are doubled in the simulation: 

* One for _other_ players
* One for _yourself_

Every strategy has a "Pass" option that simply ignores it.

== RATING TANK ==
(manipulates score based on what a player has in their tank, on its own)

* Add 1 point for every _card_
* Add 1 point for every _color_
* Add as many points as the biggest stack
* Add 1 point for every stack > 2 cards
* Add 1 point for every stuck <= 2 cards

== RATING CARD ==
(manipulates score based on the three colors shown on the top card)

* Add 1 point for every matching _card_
* Add 1 point for every matching _color_

== RATING FIRST ==
(change decisions based on how close / far away people are to winning)

* Add 5 points if close to winning
* Remove 5 points if close to winning

== RATING LAST ==
* Add 5 points if last place
* Remove 5 points if last place

== RATING COMPARE ==
(change decisions based on how this player compares with your score)

* Add 5 points if this player is above X score
* Add 5 points if this player has a significant lead
* Add 5 points if this player has a better score than your own
* Add 5 points if this player is close to you

== ACTION ==
(general rules about which action to take)

* Random
* Always Steal
* Always Score
* Prefer Steal
* Prefer Score
* Steal if somebody matches all three colors => GuaranteedSteal
* Steal if somebody matches two colors