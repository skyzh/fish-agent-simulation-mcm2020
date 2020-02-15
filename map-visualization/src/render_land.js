// const data = require('./assets/land.json').land
const data = require('./assets/2003-6-living.json').food
require('d3-contour')
const d3 = require('d3')

module.exports = svg => {
    const densityData = d3.contours()
        .size([1130, 540])
        .thresholds(5) (data);
    d3.select(svg)
        .attr('width', '2000px')
        .attr('height', '2000px')
        .selectAll("path")
        .data(densityData)
        .enter()
        .append("path")
          .attr("d", d3.geoPath())
          .attr("fill", "none")
          .attr("stroke", "#69b3a2")
          .attr("stroke-width", "0.5px")
          .attr("stroke-linejoin", "round")
}
