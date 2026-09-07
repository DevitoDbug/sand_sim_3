## Sand Simulation
- A falling-sand simulation made with Rust + Macroquad. 
- This version is much more performant that what i had in [v1]( https://github.com/DevitoDbug/sand_sim ) and 
[v2]( https://github.com/DevitoDbug/sand_sim_2 ) mostly because all of the state used for the board is in a single vector. 
- The other implementations had the sand particles maintain their own state, it was getting slower the more sand 
you spawn into the board.

## Screen grab
![board](https://github.com/DevitoDbug/sand_sim_3/blob/master/docs/screen_cast.gif?raw=true)
