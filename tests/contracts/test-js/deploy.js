const Web3 = require('web3')
const contractFile = require('./compile')

const bytecode = contractFile.evm.bytecode.object
const abi = contractFile.abi
const privKey = '0x5087cdc9b364f842b856aea161e078ee7cc1f56465f0476e9e26a8ab7f85483d'
const address = '0x544373db034533D23F6D8E998E3e86079050084C'
const web3 = new Web3(new Web3.providers.HttpProvider('http://localhost:9933'))

const deploy = async () => {
  const token = new web3.eth.Contract(abi)
  const network = await web3.eth.net.getId()
  // console.log("IsListening",web3.eth.net.isListening());
  const tokenTx = token.deploy({
    data: bytecode,
    arguments: ['Test new Token', 'e5IRE', 100000000000000000000n]
  })

  const createTransaction = await web3.eth.accounts.signTransaction(
    {
      from: address,
      data: tokenTx.encodeABI(),
      gas: '4294967',
      common: {
        customChain: {
          name: '5ire',
          chainId: 1000,
          networkId: network
        }
      }
    },
    privKey
  )

  const createReceipt = await web3.eth.sendSignedTransaction(
    createTransaction.rawTransaction
  )
  console.log('Contract was sucessfully deployed at address: ', createReceipt.contractAddress)
}

deploy()
