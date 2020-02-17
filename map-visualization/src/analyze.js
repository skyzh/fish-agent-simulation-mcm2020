const date = require('./config.js')
const base = require('./base')
const data = require(`${base}/${date}.json`).fish

module.exports = () => {
    const isInRange = (x, y) => {
        const dx = 563 - x
        const dy = 310 - y
        const r = 80
        return dx * dx + dy * dy < r * r
    }
    const newInRange = (x, y) => {
        const dx = 592 - x
        const dy = 239 - y
        const r = 80
        return dx * dx + dy * dy < r * r
    }
    let sumx = 0
    let sumy = 0
    let n = 0
    data.forEach(f => {
        if (isInRange(f.x, f.y)) {
            sumx += f.x
            sumy += f.y
            n += 1
        }
    })
    console.log(`${sumx / n},${sumy / n},${n}`)
}
