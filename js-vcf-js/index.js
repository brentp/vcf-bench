const fs = require('fs')
const VCF = require('@gmod/vcf').default
const { createGunzip } = require('zlib')
const readline = require('readline')

const rl = readline.createInterface({
  input: fs.createReadStream(process.argv[2]).pipe(createGunzip()),
})

let header = []
let elts = []
let parser = undefined

rl.on('line', function (line) {
  if (line.startsWith('#')) {
    header.push(line)
    return
  } else if (!parser) {
    parser = new VCF({ header: header.join('\n') })
  }
  const elt = parser.parseLine(line)
  elts.push(elt.INFO.AN[0])
})

rl.on('close', function () {
  console.log(elts.reduce((a, b) => a + b, 0) / elts.length)
})
