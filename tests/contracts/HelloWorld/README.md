# Test: Contracts - HellWorld Example
This is an example of deploying Solidity Smart Contract in 5ireChain

_This is intended as a reference only._

## Build & Run

To build the chain, execute the following commands from the project root:

```shell
$ cargo build --release
```

To execute the chain, run:

```shell
$ ./target/release/node-5ire --dev
```


## Run Example Using script

**0. Run 5ire node and frontend**
```shell
$ ./target/release/node-5ire --dev
```

```shell
https://polkadot.js.org/apps/
```
**1. Create a key file**
Create a key.json file that contains public and private keys
```shell
{
    "public": "0xd43593c715fdd31c61141abd04a99fd6822c8558",
    "private": "0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a"
}
```
**2. Install packages using node npm**
```shell
npm i
```
**3. Run index.js to deploy contracts**
To finally deploy contracts using web3, run index js script
```shell
node index.js
```

This will deploy contracts on 5ire chain and you will recieve a response somewhat similar to following:
```json
{
  messageHash: '0x2f12013a9241358a577bc091df35199ab7d2b6bc980c053f1070c35256d88fd9',
  v: '0x7f4',
  r: '0x65cdc9f6bda7f846f668359a8a137587d5aea385fab33de1864f6f5188de9d54',
  s: '0x1ef81acc5b4931f33a9d167065f7962cd3a2d458608ef8df4e53c2aa9a87d24c',
  rawTransaction: '0xf906690480831000008080b90619608060405234801561001057600080fd5b506040518060400160405280600581526020017f48656c6c6f0000000000000000000000000000000000000000000000000000008152506000908051906020019061005c929190610062565b50610166565b82805461006e90610105565b90600052602060002090601f01602090048101928261009057600085556100d7565b82601f106100a957805160ff19168380011785556100d7565b828001600101855582156100d7579182015b828111156100d65782518255916020019190600101906100bb565b5b5090506100e491906100e8565b5090565b5b808211156101015760008160009055506001016100e9565b5090565b6000600282049050600182168061011d57607f821691505b6020821081141561013157610130610137565b5b50919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602260045260246000fd5b6104a4806101756000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80633d7403a31461003b578063e21f37ce14610057575b600080fd5b61005560048036038101906100509190610230565b610075565b005b61005f61008f565b60405161006c91906102b2565b60405180910390f35b806000908051906020019061008b92919061011d565b5050565b6000805461009c90610388565b80601f01602080910402602001604051908101604052809291908181526020018280546100c890610388565b80156101155780601f106100ea57610100808354040283529160200191610115565b820191906000526020600020905b8154815290600101906020018083116100f857829003601f168201915b505050505081565b82805461012990610388565b90600052602060002090601f01602090048101928261014b5760008555610192565b82601f1061016457805160ff1916838001178555610192565b82800160010185558215610192579182015b82811115610191578251825591602001919060010190610176565b5b50905061019f91906101a3565b5090565b5b808211156101bc5760008160009055506001016101a4565b5090565b60006101d36101ce846102f9565b6102d4565b9050828152602081018484840111156101ef576101ee61044e565b5b6101fa848285610346565b509392505050565b600082601f83011261021757610216610449565b5b81356102278482602086016101c0565b91505092915050565b60006020828403121561024657610245610458565b5b600082013567ffffffffffffffff81111561026457610263610453565b5b61027084828501610202565b91505092915050565b60006102848261032a565b61028e8185610335565b935061029e818560208601610355565b6102a78161045d565b840191505092915050565b600060208201905081810360008301526102cc8184610279565b905092915050565b60006102de6102ef565b90506102ea82826103ba565b919050565b6000604051905090565b600067ffffffffffffffff8211156103145761031361041a565b5b61031d8261045d565b9050602081019050919050565b600081519050919050565b600082825260208201905092915050565b82818337600083830152505050565b60005b83811015610373578082015181840152602081019050610358565b83811115610382576000848401525b50505050565b600060028204905060018216806103a057607f821691505b602082108114156103b4576103b36103eb565b5b50919050565b6103c38261045d565b810181811067ffffffffffffffff821117156103e2576103e161041a565b5b80604052505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b600080fd5b600080fd5b600080fd5b600080fd5b6000601f19601f830116905091905056fea2646970667358221220875a76213e707b35bd4c5dea6e959986efe795b8355574ca1c8574530fd179fb64736f6c634300080700338207f4a065cdc9f6bda7f846f668359a8a137587d5aea385fab33de1864f6f5188de9d54a01ef81acc5b4931f33a9d167065f7962cd3a2d458608ef8df4e53c2aa9a87d24c',
  transactionHash: '0x14bba46d8e929c6b89f479b1ab394e7335363086b0e3a7dcbf9a670979774597'
}
Contract deployed at address 0x9a8c159417B399AcF4354088Fb4BDf5b990b9EE7

```shell
The contract can now be interacted with using this contract address via web3 or directly on https://polkadot.js.org/apps/
```

## Run Example Manually

**0. Run 5ire node and frontend**
* Run node
```shell
$ ./target/release/node-5ire --dev
```

* Frontend
```shell
https://polkadot.js.org/apps/
```

* Set up customized type in order to view chain state of evm pallet, ethereum pallet
Developer tab of Settings app

**1. Create Hello World Smart Contract**
* Compile HelloWorld smart contract <> in http://remix.ethereum.org/ 
* Bytecode:<>
```cpp
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.3;

// Defines a contract named `HelloWorld`.

contract HelloWorld {

   string public message;

   // Similar to many class-based object-oriented languages, a constructor is a special function that is only executed upon contract creation.
   constructor() {

      // sets the value into the contract's `message` storage variable).
      message = "Hello";
   }

   // A public function that accepts a string argument and updates the `message` storage variable.
   function update(string memory newMessage) public {
      message = newMessage;
   }
}
```

**2. Convert Substrate's account(H256 - 32 bytes) into Ethereum Account (H160 - 20 bytes) in Development**
* Creating Account Alice follow subkey https://docs.substrate.io/v3/tools/subkey/
```shell
subkey inspect "//Alice"
```
** Result
```shell
Secret Key URI `//Alice` is account:
  Secret seed:       0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
  Public key (hex):  0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
  Public key (SS58): 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
  Account ID:        0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
  SS58 Address:      5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```
** Alice's Etherium Account: Use first 20 bytes of the hex encoded from public key Substrate address
```shell
0xd43593c715fdd31c61141abd04a99fd6822c8558
```


**3. Contract Creation**

Use the Polkadot UI **Extrinsics** app to deploy the contract from Alice's account (submit the extrinsic as a signed transaction) using **sudo -> sudo(call)** ->  **evm > create** (Because only root account can be deployed smart contract now)

```shell
source: 0xd43593c715fdd31c61141abd04a99fd6822c8558
init: <raw contract bytecode, a very long hex value>
value: 0
gas_limit: 4294967295
gas_price: 1
nonce: <empty> {None}
```
** Event Trigger
```shell
evm.Created
A contract has been created at given [address]. 
H160
0x8a50db1e0f9452cfd91be8dc004ceb11cb08832f // this is smart contract address
```

```shell
sudo.Sudid
A sudo just took place. [result] 
Result<Null, SpRuntimeDispatchError>
Ok
```

```shell
balances.Withdraw
Some amount was withdrawn from the account (e.g. for transaction fees). [who, value]
AccountId32
ALICE
5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
u128
125,001,765
```


**5. Check Chain State**

Use the **Chain State** app to query **evm > accountCodes** and view the value associated with Alice's account
Disable **include option** : list all of smart contracts that deployed 
**include option** : list specific smart contract with smart contract address parameter 


```shell
[
  [
    [
      0x8a50db1e0f9452cfd91be8dc004ceb11cb08832f   //smart contract address that generate from Alice Account
    ]
    0x608060405234801561001057600080fd5b506004361061... //bytecode of solidity smart contract
  ]
  [
    [
      0xd43593c715fdd31c61141abd04a99fd6822c8558 // Alice's Account
    ]
    
  ]
  [
    [
      0x6be02d1d3665660d22ff9624b7be0551ee1ac91b // Account for Genesis Config
    ]
    
  ]
]
```

Use the **Chain State** app to query **evm > accountStorages** and view the value associated with Alice's account
Disable **include option** : list all of storages in evm
**include option** : list specific storages in evm


```shell
[
  [
    [
      0x8a50db1e0f9452cfd91be8dc004ceb11cb08832f  //smart contract address
      0x0000000000000000000000000000000000000000000000000000000000000000   //index smart contract
    ]
    0x48656c6c6f00000000000000000000000000000000000000000000000000000a  // "0x48656C6C6F" is "Hello" hex encoding
  ]
]
```



**6. Update Message**

** Use the **Developer** -> **Extrinsics** tab to invoke the **update(string memory newMessage)** function on the Hello World contract with **sudo -> sudo(call)** -> **evm > call**
** **input** parameter: ABI encoded for specific function.

*** Go to https://abi.hashex.org/

*** Import ABI code :<>  that is compiled in Remix or in <>

*** Choose function type : **update**  with:

```shell
newMessage: "World"
```
*** Result function selector with **update** function:

```shell
3d7403a300000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000005576f726c64000000000000000000000000000000000000000000000000000000
```

** **sudo ->sudo(call)**  -> **evm > call**
```shell
target: 0x8a50db1e0f9452cfd91be8dc004ceb11cb08832f // smart contract id
source: 0xd43593c715fdd31c61141abd04a99fd6822c8558 // Alice EVM's account
input:0x3d7403a300000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000005576f726c64000000000000000000000000000000000000000000000000000000
value: 0
gas_limit: 4294967295
gas_price: 1
```

** Event Trigger

```shell
evm.Executed
A [contract] has been executed successfully with states applied. 
H160
0x8a50db1e0f9452cfd91be8dc004ceb11cb08832f
```

**6. Check new Message**

Use the **Chain State** app to query **evm > accountStorages** and view the value associated with Alice's account
Disable **include option** : list all of storages in evm
**include option** : list specific storages in evm


```shell
[
  [
    [
      0x8a50db1e0f9452cfd91be8dc004ceb11cb08832f  //smart contract address
      0x0000000000000000000000000000000000000000000000000000000000000000   //index smart contract
    ]
    0x576f726c6400000000000000000000000000000000000000000000000000000a  // "0x576F726C64" is "World" hex encoding
  ]
]
```