const path = require('path')
const fs = require('fs')
const solc = require('solc')

const contractPath = path.resolve(__dirname, 'ERC20Token.sol')
const source = fs.readFileSync(contractPath, 'utf8')
const input = {
  language: 'Solidity',
  sources: {
    'ERC20Token.sol': {
      content: source
    }
  },
  settings: {
    outputSelection: {
      '*': {
        '*': ['*']
      }
    }
  }
}
const tempFile = JSON.parse(solc.compile(JSON.stringify(input)))
const contractFile = tempFile.contracts['ERC20Token.sol'].ERC20Token
module.exports = contractFile
