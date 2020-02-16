const d3 = require('d3')
const _ = require('lodash')
require('d3-contour')
const geo = require('geoutil')
const W = 1130
const H = 540
const centerX = 563
const centerY = 310

function xyToLongLat(x, y) {
    const nx = x + 1185
    const ny = y + 60
    return [-179.95 + 0.1 * nx, 89.95 - 0.1 * ny]
}

module.exports = svg => {
    const ctx = d3.select(svg)
    /*
    ctx.append('circle')
        .attr('cx', 563)
        .attr('cy', 310)
        .attr('r', 80)
        .attr('stroke', 'rgb(237,85,59)')
        .attr('line-width', 3)
        .attr('fill', 'none')
    */
    const data = []
    const pA = xyToLongLat(centerX, centerY)
    for (let y = 0; y < H; y++) {
        for (let x = 0; x < W; x++) {  
                const pB = xyToLongLat(x, y)   
                data.push(geo.pointDistance(pA, pB, true))
        }
    }
    const densityData = d3.contours()
        .size([1130, 540])
        .thresholds(_.range(300*1000, 1000*1000, 100*1000)) (data)

    const densityDataOuter = d3.contours()
        .size([1130, 540])
        .thresholds([900 * 1000]) (data)

    d3.select(svg)
        .attr('width', '2000px')
        .attr('height', '2000px')
        .selectAll(".center-path")
        .data(densityData)
        .enter()
        .append("path")
          .attr("d", d3.geoPath())
          .attr("fill", "none")
          .attr("stroke", "rgb(48,107,84)")
          .attr("stroke-width", "1px")
          .attr("stroke-linejoin", "round")
          .attr('stroke-dasharray', '5 5')
    d3.select(svg)
        .attr('width', '2000px')
        .attr('height', '2000px')
        .selectAll(".center-path")
        .data(densityDataOuter)
        .enter()
        .append("path")
        .attr("d", d3.geoPath())
        .attr("fill", "none")
        .attr("stroke", "rgb(48,107,84)")
        .attr("stroke-width", "1px")
        .attr("stroke-linejoin", "round")

    ctx.append('circle')
        .attr('cx', 563)
        .attr('cy', 310)
        .attr('r', 80)
        .attr('stroke', '#ed553b')
        .attr('stroke-width', "1px")
        .attr('fill', 'none')
}
