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

This is done with R Studio and the R programming language. `ggplot`
library is very helpful in producing high-quality and good-looking figures.
Figures are exported in pdf format, ready for use in LaTeX.
