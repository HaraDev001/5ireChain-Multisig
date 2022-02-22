import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from '@polkadot/keyring/types';
import * as web3Utils from 'web3-utils';
import { u8aToHex } from '@polkadot/util';

// Provider is set to localhost for development
const wsProvider = new WsProvider("ws://localhost:9944");

// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

const HELLO_BYTECODES = require("./HelloWorld.json").object;

// Setup the API and Alice Account
async function init(){
	console.log(`Initiating the API (ignore message "Unable to resolve type B..." and "Unknown types found...")`);

	// Initiate the API
	const api = await ApiPromise.create({
		provider: wsProvider,
		types: {
			// mapping the actual specified address format
			Address: "MultiAddress",
			// mapping the lookup
			LookupSource: "MultiAddress",
			Account: {
				nonce: "U256",
				balance: "U256"
			},
			Transaction: {
				nonce: "U256",
				action: "String",
				gas_price: "u64",
				gas_limit: "u64",
				value: "U256",
				input: "Vec<u8>",
				signature: "Signature"
			},
			Signature: {
				v: "u64",
				r: "H256",
				s: "H256"
			}
		}
	});
	console.log(`Initialiation done`);
	console.log(`Genesis at block: ${api.genesisHash.toHex()}`);

	//generate key for alice and bob
	const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
	const bob = keyring.addFromUri('//Bob', { name: 'Bob default' });

	//Get public key
	const pubAlice = u8aToHex(alice.publicKey);
	const pubBob = u8aToHex(bob.publicKey);

	console.log(`Alice: ${u8aToHex(alice.publicKey)}`);
	const { nonce, data: balance } = await api.query.system.account(alice.address);
	console.log(`Alice Substrate Account: ${alice.address}`);
	console.log(`Alice Substrate Account (nonce: ${nonce}) balance, free: ${balance.free.toHex()}`);

	//const aliceEvmAccount = "0xd43593c715fdd31c61141abd04a99fd6822c8558";
	//Truncate Address into 20 bytes
	const aliceEvmAccount = pubAlice.slice(0,42);
	const bobEvmAccount = pubBob.slice(0,42);
	
	console.log(`Alice EVM Account: ${aliceEvmAccount}`);
    console.log(`Alice EVM Account Balance: ${await api.rpc.eth.getBalance(aliceEvmAccount)}`);
	//const evmData = (await api.query.evm.accountStorages(aliceEvmAccount, "0x045c0350b9cf0df39c4b40400c965118df2dca5ce0fbcf0de4aafc099aea4a14")) as any;
	//console.log(`Alice EVM Account (nonce: ${evmData.nonce}) balance: ${evmData.balance}`);
    //console.log(evmData);
	return { api, alice, bob, aliceEvmAccount, bobEvmAccount };
}

// Create the Hello world contract from ALICE
async function deploySc(api: ApiPromise, alice: KeyringPair, aliceEvmAccount:string) {

	console.log(`\nCreating Smart Contract`);
	//console.log(`\nBytecode: ${HELLO_BYTECODES}`);
	// params: [bytecode, initialBalance, gasLimit, gasPrice],
	// tx: api.tx.evm.create

	const bytecodeHex = `0x${HELLO_BYTECODES}`;
	const transaction = await api.tx.evm.create(aliceEvmAccount,bytecodeHex, 0, 4294967295, 1, null);

	const contract = new Promise<{ block: string, address:string}>((resolve, reject) => {
		transaction.signAndSend(alice, (result) => {
			console.log(`Contract creation is ${result.status}`);
			if (result.status.isInBlock) {
				console.log(`Contract included at blockHash ${result.status.asInBlock}`);
				console.log(`Waiting for finalization... (can take a minute)`);
			} else if (result.status.isFinalized) {
				const contractAddress = (
                    result.events?.find(
                        event => event?.event?.index.toHex() === "0x1801"
                    )?.event.data[0].toString()
                );
				console.log(`Contract finalized at blockHash ${result.status.asFinalized}`);
				console.log(`Contract address: ${contractAddress}`);
			
			
				resolve({
					block: result.status.asFinalized.toString(),
					address: contractAddress
				});
			
			}
		});
	});
	return Promise.resolve(contract);
}


async function getString(api: ApiPromise, alice: KeyringPair, contractAddress: string) {

	console.log(`\nRetrieving Contract from evm address: ${contractAddress}`);

	// Retrieve Alice account with new nonce value
	const { nonce, data: balance } = await api.query.system.account(alice.address);
	console.log(`Alice Substrate Account (nonce: ${nonce}) balance, free: ${balance.free}`);

	const accountCode = (await api.query.evm.accountCodes(contractAddress)).toString();
	console.log(`Contract account code: ${accountCode.substring(0, 16)}...${accountCode.substring(accountCode.length - 16)}`);

	// Computing Contract Storage Slot, using slot 0 and alice EVM account
	const aliceEvmAccount = u8aToHex(alice.publicKey).slice(0,42);
	const slot = "0";
	const mapStorageSlot = slot.padStart(64, '0');
	const mapKey = aliceEvmAccount.toString().substring(2).padStart(64, '0');

	const storageKey = web3Utils.sha3('0x'.concat(mapKey.concat(mapStorageSlot)));
	console.log(`Alice Contract storage key: ${storageKey}`);

	const accountStorage = (await api.query.evm.accountStorages(contractAddress, "0x0000000000000000000000000000000000000000000000000000000000000000")).toString();
	
	console.log(`Alice Contract account storage: ${hexToUtf8(accountStorage.slice(2,accountStorage.length))}`);
	return;
}


function hexToUtf8(s:string)
{
  return decodeURIComponent(
     s.replace(/\s+/g, '') // remove spaces
      .replace(/[0-9a-f]{2}/g, '%$&') // add '%' before each 2 characters
  );
}


async function main() {
	const { api, alice, bob , aliceEvmAccount , bobEvmAccount} = await init();

	console.log(bob)
	console.log(bobEvmAccount)

	// step 1: Creating the contract from ALICE
	const contractAccountAlice = await deploySc(api, alice , aliceEvmAccount);

	await getString(api, alice, contractAccountAlice.address);
}

main().catch(console.error).then(() => process.exit(0));