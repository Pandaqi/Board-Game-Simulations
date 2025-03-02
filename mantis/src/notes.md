== TO DO ==
* Instead of PRINTING, save the game data into a list on state, then write that to a LOG FILE
* Card could be a tuple struct => Card(u8, u8, u8) => but I think that's actually bad for performance => could also be an ARRAY, that WOULD be faster 
* Perhaps it's more useful to go -2,2 and _multiply_ these points by some fixed factor, to exaggerate them

==== STRATEGIES ====

Every strategy is NUMERICAL. Usually, they range from -X to X. 
The value 0 is essentially a "pass": do not do anything with this strategy

There is _one_ override strategy that is very likely to "pass". But if it doesn't, it overrides anything before it.

Most (rating) strategies can be doubled in the simulation: one for rating _other players_, one for rating _yourself_