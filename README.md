# Terminal Paper Golf game - Weekend project
## keys:
|         	| map.txt    	| UI         	|
|---------	|------------	|------------	|
| Rough   	| .          	| #          	|
| Fairway 	| f          	| @          	|
| Start   	| x          	| 0          	|
| Slope   	| <, >, ^, v 	| <, >, ^, v 	|
| Sand    	| s          	| *          	|
| Hole    	| o          	| F          	|
| Water   	| w          	| w          	|
| Tree    	| t          	| Y          	|
## Mechanics
- Each round chose a direction (N, S, W, E, NW, NE, SW, SE).
- The strength of the shot is a random d6.
- You can chose to drive or put, the put is always strength 1.
- Shooting from sand incurs a -1 penalty.
- Shooting from fairway incurs a +1 bonus.
- Srength determine how many grid spaces the ball will move on the chosen direction (diagonal is the same as straight).
- If the ball lands on a slope, it rolls.
## TODO
- Refactor some functions (always, uh?)
- Implement bound checks
- Make current ball location not overwrite terrain (it was done this way to make rendering easier).
- Include iron club: Drive can only be used from fairway and its the only one that can send ball over trees.
- Implement DDA to detect tree instersection.
