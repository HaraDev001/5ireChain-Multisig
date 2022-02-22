const Web3 = require('web3')

const web3 = new Web3(new Web3.providers.HttpProvider('http://localhost:9933'))
// Alice evm account
const GENESIS_ACCOUNT = '0xd43593c715fdd31c61141abd04a99fd6822c8558'

async function getBalance (account) {
  const balance = await web3.eth.getBalance(account)
  const balanceOfEther = web3.utils.fromWei(balance, 'ether')
  console.log(`${account} has balance: ${balanceOfEther} e5IRE`)
}

getBalance(GENESIS_ACCOUNT)
