License: Unlicense //TODO set correct license
#esg_fetcher
The esg_fetcher fetches an esg score from an already known
endpoint serving as an oracle

An offchain worker is used to fetch the score of the validator. 
Currently there is no mechanism in place for validators to check 
each other's esg score at the oracle. Instead each validator queries
for their own esg score and submits it on chain in a transaction to be used in the consensus.
These on chain stored values are then used by the pallet responsible
for choosing all validators to be used in aura.