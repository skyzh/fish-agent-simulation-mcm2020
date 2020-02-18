# Fish Agent Simulation for MCM2020

For MCM2020 Problem A, our team proposed an agent-based model to
identify where fish are most likely to be.

This repo contains the complete workflow for running simulation,
analyzing data and visualizing.

## Fish Simulation

This part of work is done with the Rust programming language. Source 
code is located in `png_process` folder.

We grab sea surface temperature data from NASA Earth Observation website,
thus using real world data for analysis.

Here we proposed a living index function, which is composed of five parts:
* temperature of current location
* food availability (more fish clustering in one location causes lower food availability)
* land distance (far from land causes low land distance score)
* age
* random factor ~N(0,0.1)

Fish agents will automatically discover optimal location for themselves.
By simulating this process with real world data we got a reasonable model
for locating fish.

We also proposed a Markov-based global warming model to predict temperature
of a given location at a given time from historical data.

Combining the simulation process and global warming model, we successfully
obtained the most likely location of these fish in the future.

## Visualization of Simulation Result

Data visualization is mainly done with d3.js. With d3-contour library,
it's easy to observe living index function value.

We stack land map, visualization layer and temperature layer from top
to bottom to obtain a map visualization.

Legends are drawn with Apple Keynote.

## Data Analysis

In `log_process`, we use Python to extract useful information from
simulation log. They are sorted into `.csv` files. These results 
are retained in `log_process` folder.

In `coordinate_convert`, we tried to obtain latitude and longitude from
GeoTIFF file. Surprisingly, it's easy to convert pixel position to
earth location with simple arithmetic.

## Charts and Plotting

This is done with R Studio and the R programming language. R Markdown files
are located in `analysis` folder. `ggplot` library is very helpful in
producing high-quality and good-looking figures.
These figures are exported in pdf format, ready for use in LaTeX.

## Related Works

The design of this agent simulation system is greatly inspired
by my previous project [Game Theory on Matrix](https://github.com/skyzh/game-theory-on-matrix) 
(aka. 基于记忆效应的空间囚徒困境中系统合作的演化). In this fish agent
simulation project, I leveraged real-world data and the Rust programming
language to obtain a more stable and pratical model.

## Visualization

Food Index and Load Index

<img width="45%" alt="food_score_explanation" src="https://user-images.githubusercontent.com/4198311/74668549-678c6600-51e0-11ea-9d39-dcd40a3c83da.png"> <img width="45%" alt="land_score_explanation" src="https://user-images.githubusercontent.com/4198311/74668556-6a875680-51e0-11ea-8868-28d43c48ca36.png">

Fish distribution in one month

<img width="1220" alt="2040-8-fish" src="https://user-images.githubusercontent.com/4198311/74668559-6bb88380-51e0-11ea-8d39-2542daffe7fa.png">

Fish distribution animation (Model evaluation result in very early stage)

![animation](https://user-images.githubusercontent.com/4198311/74668562-6e1add80-51e0-11ea-83d1-39383d709c9d.gif)

![output](https://user-images.githubusercontent.com/4198311/74699390-88ce7000-523b-11ea-808b-21e49687cb39.gif)

## License

The simulation program, data analysis scripts and visualization program
is licensed under MIT.

## Credit

Thank my teammates T.T. Tang and R.L. Ye for designing this model in
detail and coming up with ways to test and evaluate this model.
