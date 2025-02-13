const d3 = require('d3')
const date = require('./config.js')
const base = require('./base')
const data = require(`${base}/${date}.json`).fish
const _ = require('lodash')
const split = 10

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
        
    const renderSmallFish = () => {
        ctx.selectAll('.fish')
            .attr('cx', d => d.x)
            .attr('cy', d => d.y)
            .attr('fill', d => `rgba(237,85,59,${d.cnt / 30})`)
            .attr('r', split / 10)
            .attr('stroke', d => `rgba(0,0,0,${d.cnt / 30})`)
            .attr('stroke-width', 0.1)
    }

    const renderNormalFish = () => {
        ctx.selectAll('.fish')
            .attr('cx', d => d.x)
            .attr('cy', d => d.y)
            .attr('fill', d => `rgba(237,85,59,${d.cnt / 20 / split})`)
            .attr('r', split / 4)
            .attr('stroke', d => `rgba(100,100,100,${d.cnt / 20 / split})`)
            .attr('stroke-width', split / 20)
    }

    renderNormalFish()
}
