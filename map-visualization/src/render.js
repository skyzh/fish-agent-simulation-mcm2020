const d3 = require('d3')
const data = require('./assets/2049-9.json').fish
const _ = require('lodash')
const split = 5

module.exports = (svg) => {
    const ctx = d3.select(svg)
    ctx.attr('width', '2000px')
    ctx.attr('height', '2000px')
    let cnt = {}
    data.forEach(e => {
        const nx = Math.floor(e.x / split)
        const ny = Math.floor(e.y / split)
        const key = `${nx},${ny}`
        if (!(key in cnt)) cnt[key] = { x: 0, y: 0, cnt: 0}
        cnt[key].x += e.x
        cnt[key].y += e.y
        cnt[key].cnt += 1
    })
    cnt = _.map(cnt, (d, k) => {
        return {
            x: d.x / d.cnt,
            y: d.y / d.cnt,
            cnt: d.cnt
        }
    })
    
    const fish = ctx.selectAll('.fish').data(cnt)
    fish.enter()
        .append('circle')
        .attr('class', 'fish')
    ctx.selectAll('.fish')
        .attr('cx', d => d.x)
        .attr('cy', d => d.y)
        .attr('fill', d => `rgba(237,85,59,${d.cnt / 30})`)
        .attr('r', d => split / 2.2)
        .attr('stroke', d => `rgba(0,0,0,${d.cnt / 30})`)
        .attr('line-width', 0.5)
    ctx.append('circle')
        .attr('cx', 563)
        .attr('cy', 310)
        .attr('r', 80)
        .attr('stroke', 'rgb(237,85,59)')
        .attr('line-width', 3)
        .attr('fill', 'none')
}
