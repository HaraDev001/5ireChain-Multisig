# Test: Contracts - Test-JS
This is an example of using 5ire-js-api to run and call smart contract.

_This is intended as a reference only._

**1. Run Helloworld Smart contract by 5ire-api**

* Install package 
```shell
npm install 
```

* Run hello-world.ts
```shell
node_modules/.bin/ts-node hello-world.ts
```

* Step init(), deploy_sc(), get_string()
Output like that
```shell
Initiating the API (ignore message "Unable to resolve type B..." and "Unknown types found...")
Initialiation done
Genesis at block: 0xb868b96a831b1abd9013b8eabad73d95a625620b0ec0487b0201ace341fd5b09
Alice: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
Alice Substrate Account: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
Alice Substrate Account (nonce: 16) balance, free: 0x000000000000000000038d7ea4c68000
Alice EVM Account: 0xd43593c715fdd31c61141abd04a99fd6822c8558
Alice EVM Account Balance: 9999999999999935572141759

Creating Smart Contract
Contract creation is Ready
Contract creation is {"inBlock":"0x13a5dbaf6c208dbe4c17f3550f37086a7fda2e297b199044a5313afb1e4cbd70"}
Contract included at blockHash 0x13a5dbaf6c208dbe4c17f3550f37086a7fda2e297b199044a5313afb1e4cbd70
Waiting for finalization... (can take a minute)
Contract creation is {"finalized":"0x13a5dbaf6c208dbe4c17f3550f37086a7fda2e297b199044a5313afb1e4cbd70"}
Contract finalized at blockHash 0x13a5dbaf6c208dbe4c17f3550f37086a7fda2e297b199044a5313afb1e4cbd70
Contract address: 0xcb97b4d1aa269b15af04c493a769e67d71408e91

Retrieving Contract from evm address: 0xcb97b4d1aa269b15af04c493a769e67d71408e91
Alice Substrate Account (nonce: 17) balance, free: 1000000000000000
Contract account code: 0x60806040523480...6c63430008070033
Alice Contract storage key: 0x045c0350b9cf0df39c4b40400c965118df2dca5ce0fbcf0de4aafc099aea4a14
Alice Contract account storage: Hello
```



**2. Get any balance accounts in evm storage**


* Run balance.js

```shell
node balance.ts
```


* Retrieve balance Alice. Output like that

```shell
0xd43593c715fdd31c61141abd04a99fd6822c8558 has balance: 9999999.999999935571806928 e5IRE
```



**3. Deploy smart contract by web3**


* Run smart contract ERC20Token

```shell
node deploy.ts
```




