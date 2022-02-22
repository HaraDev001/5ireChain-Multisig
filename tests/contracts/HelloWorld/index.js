const Web3 = require('web3')
const fs = require('fs')

const web3 = new Web3('http://127.0.0.1:9933')

// commented out for now because it's not being used but for the purposes
// of having a reference, we will include it here to inform that developer
// that they can call the ABI model (application binary interface)
// for their contract.
// const ABI = fs.readFileSync('./abi.json');
const BYTECODE = fs.readFileSync('./bytecode.txt', 'utf8')

const main = async () => {
  const keys = JSON.parse(fs.readFileSync('./key.json'))
  const privateKey = keys.private
  const publicKey = keys.public

  const createTransaction = await web3.eth.accounts.signTransaction({
    from: publicKey,
    data: BYTECODE,
    value: '0x00',
    gasPrice: '0x00',
    gas: '0x100000'
  }, privateKey)
  console.log(createTransaction)

  const createReceipt = await web3.eth.sendSignedTransaction(createTransaction.rawTransaction)
  console.log('Contract deployed at address', createReceipt.contractAddress)
}

main()
