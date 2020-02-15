const date = require('./config.js')
// const data = require(`./assets/${date}-living.json`).food
const data = require('./assets/land.json').land
require('d3-contour')
const d3 = require('d3')
const _ = require('lodash')

module.exports = svg => {
    console.log(_.max(data), _.min(data))
    const contours = d3.contours()
        .size([1130, 540])
        .thresholds(_.range(0.0, 1.0, 0.1));
    const densityData = contours(data)
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
          .attr("stroke-width", "0.1px")
          .attr("stroke-linejoin", "round")
}
