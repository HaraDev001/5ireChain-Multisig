
# Test: Contracts - Calculator Example
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
```json
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
```shell
{
  messageHash: '0xe980b53eb08a0e3bf561bb63b70ea282aea958e838167ecb1b5ecd33942eaf62',
  v: '0x7f4',
  r: '0x788d388e8879963db378eb06cdda72d97d100988aa32a8e9cd7e1ca38e56beae',
  s: '0xb9b9e2b8a64e8169ea86d2310a03c7b4071324bac79029f3b9820949d517553',
  rawTransaction: '0xf902f90580831000008080b902a9608060405234801561001057600080fd5b50610289806100206000396000f30060806040526004361061006d576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff168063771602f714610072578063a391c15b146100a9578063b67d77c5146100e0578063c8a4ac9c14610117578063de2927891461014e575b600080fd5b34801561007e57600080fd5b506100a76004803603810190808035906020019092919080359060200190929190505050610179565b005b3480156100b557600080fd5b506100de6004803603810190808035906020019092919080359060200190929190505050610186565b005b3480156100ec57600080fd5b50610115600480360381019080803590602001909291908035906020019092919050505061023a565b005b34801561012357600080fd5b5061014c6004803603810190808035906020019092919080359060200190929190505050610247565b005b34801561015a57600080fd5b50610163610254565b6040518082815260200191505060405180910390f35b8082016000819055505050565b600081111515610224576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252602c8152602001807f546865207365636f6e6420706172616d657465722073686f756c64206265206c81526020017f6172676572207468616e2030000000000000000000000000000000000000000081525060400191505060405180910390fd5b808281151561022f57fe5b046000819055505050565b8082036000819055505050565b8082026000819055505050565b600080549050905600a165627a7a72305820b9474b5fe9ce6c370482137a739c6665f995be7e659bed12e086af5e2bce645600298207f4a0788d388e8879963db378eb06cdda72d97d100988aa32a8e9cd7e1ca38e56beaea00b9b9e2b8a64e8169ea86d2310a03c7b4071324bac79029f3b9820949d517553',
  transactionHash: '0x8da27e50606278f1c20de2f991ceda5ff0b63cd0fe51441d1b7da75d2ec7b5a1'
}
Contract deployed at address 0x91210e5418335B91E3a2Bf54a1Bd1704c1456F37
```

The contract can now be interacted with using this contract address via web3 or directly on https://polkadot.js.org/apps/


## Run Example Manually


**0. Run 5ire node and frontend**
```shell
$ ./target/release/node-5ire --dev
```

```shell
https://polkadot.js.org/apps/
```


**1. Create Smart Contract to perform calculator operations**
Compile smart contract for calculator in http://remix.ethereum.org/ or any other compiler

```cpp
pragma solidity ^0.4.24;

contract Calculator {
    uint c;

    function add(uint a, uint b) public {
        c = a + b;
    }

    function sub(uint a, uint b) public {
        c = a - b;
    }

    function mul(uint a, uint b) public {
        c = a * b;
    }

    function div(uint a, uint b) public {
        require(b > 0, "The second parameter should be larger than 0");

        c = a / b;
    }

    function getResult() public view returns (uint x) {
        return c;
    }
}
```

**2. Convert Substrate's account(H256 - 32 bytes) into Etherium Account (H160 - 20 bytes) in Development **

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

Use the Polkadot UI **Extrinsics** app to deploy the contract from Alice's account (submit the extrinsic as a signed transaction) using **evm > create2**

```shell
source: 0xd43593c715fdd31c61141abd04a99fd6822c8558
init: <raw contract bytecode, a very long hex value>
salt: 0x0
value: 0
gas_limit: 1000000
gas_price: 1
nonce: <empty> {None}
```
** Event Trigger
```shell
evm.Created
A contract has been created at given [address]. 
H160
0xec393aa3fecdd7c19c9421f1e9228be0bc60ad7c // this is smart contract id
```

**4. Perform Operations**
Use https://abi.hashex.org/ to get bytes for performing desired operation on smart contract. Here we will perform division of 40 by 2.

Use the **Extrinsics** app to query **evm > call** and view the value associated with Alice's account

```shell
source: 0xd43593c715fdd31c61141abd04a99fd6822c8558
target: 0xec393aa3fecdd7c19c9421f1e9228be0bc60ad7c
input: 0xa391c15b00000000000000000000000000000000000000000000000000000000000000280000000000000000000000000000000000000000000000000000000000000002
value: 0
gasLimit: 1000000
gasPrice: 1
nonce: <empty>
```

**5. Check operation result**

Use the **Chain State** app to query **evm > accountStorage** and view the value associated with Bob's account

** first parameter: smart contract id
```shell
0xec393aa3fecdd7c19c9421f1e9228be0bc60ad7c
```

** Second parameter: storage slot

```shell
<empty>
```
** Result: Calculation
```shell
0x0000000000000000000000000000000000000000000000000000000000000014 (20)
```
