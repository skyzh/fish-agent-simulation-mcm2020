const geo = require('geoutil')
function xyToLongLat(x, y) {
    const nx = x + 1185
    const ny = y + 60
    return [-179.95 + 0.1 * nx, 89.95 - 0.1 * ny]
}

const r = 80000

function analyze(m) {
    const base = require('./base')
    const data1 = require(`${base}/2050-${m}.json`).fish
    const data2 = require(`${base}/2051-${m}.json`).fish
    const isInRange = (x, y) => {
        const pB = xyToLongLat(563, 310)   
        const pA = xyToLongLat(x, y)
        return geo.pointDistance(pA, pB, true)
    }
    const newInRange = (x, y) => {
        const pB = xyToLongLat(592, 239)   
        const pA = xyToLongLat(x, y)
        return geo.pointDistance(pA, pB, true)
    }
    const ran = newInRange
    let sumdist = 0
    let n = 0
    data1.forEach(f => {
        const dist = ran(f.x, f.y)
        if (dist < r) {
            sumdist += dist
            n += 1
        }
    })
    data2.forEach(f => {
        const dist = ran(f.x, f.y)
        if (dist < r) {
            sumdist += dist
            n += 1
        }
    })
    console.log(`${sumdist / n},${n},New,${m},Herring`)
}

for (let i = 1; i <=12; i++) {
    analyze(i)
}
