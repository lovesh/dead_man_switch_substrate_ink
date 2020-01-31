# Dead man switch using Substrate and Ink

## Objective:
Implement a dead man's switch, e.g., https://en.wikipedia.org/wiki/Dead_man%27s_switch, using the blokchain.

## Background:  
Aside from playing a prominent role in some of the better, and not so great, novels and movies, a dead man's switch 
plays a crucial but neglected role in "future-proofing" decentralized account management. For example, how does an account 
owner safely share the private key to a decentralized account in case of death without relying on a centralized, "trusted" solution ?  

## References (incomplete):  
https://github.com/deadmenswitch/dms  
https://github.com/Netdex/Seppuku  
https://github.com/eduDorus/dead-man-switch

### Potential solutions
I describe 3 approaches but take the 3rd one (Ink smart contract on Substrate) since its the easiest to develop.

1. Threshold secret sharing.
Since all data on the blockchain in publicly readable, even secret-sharing the data such that only a threshold can reconstitute 
is futile since all shares are readable. A permissioned blockchain might help is a better choice since the shares can be encrypted 
for each node (1 encryption per node) and then when the blockcahin detects that the "man has died" due to lack of heartbeat (no 
pings to the blockchain), the blockchain nodes re-encrypt their shares for the heir's public key and release the shares. An issue 
with this is validator set change, i.e. when existing nodes leave, the secret has to be re-shared. Also, when new nodes join, the 
safe quorum changes, eg, if originally the validator set had size 5, the "man" would have done a 3-of-5 sharing such that at least 3 
nodes are needed to reconstitute which is > 51% of nodes (assuming proof of work blockchain). If the 5 new nodes joined, 3 nodes only 
constitute 30% of the network which we assume can be compromised. 

1. Discreet Log Contracts  
Another approach is to use idea of Discreet Log Contract which was an MIT research group's proposal. 
[Here is my blog post](https://medium.com/coinmonks/conditional-payments-on-bitcoin-using-discreet-log-contracts-eed19e086e3) on that 
but the idea is that a secret can be learned from a signature created by an oracle and the heir can learn the private key of a public key 
which has the funds by using some information that only he has and the signature. The oracle's role is played by the blockchain and the 
signature scheme used is Schonrr which can be thresholdized. Since this approach also uses a threshold cryptosystem, the issues with a dynamic 
validator set has to be dealt with. eg. workflow 
    1. Say the blockchain has secret, public keys (x, X). x is the threshold secret key which no single entity knows and X is the threshold public 
    key which everyone knows.
    1. The blockchain has a "DeadManSwitch" contract which when initialized by the "man", registers him and instructs the validators to do some 
    multi-party computation that emits (like a EVM log) a value R.
    1. The "man" then use the value R and his heirs public key P to generate a key Y and registers it in the contract where funds should be 
    transferred in case of his death.
    1. When the "man" dies, the blockchain publishes a Schnorr signature (using above R as a commitment to a nonce) on a message like "The man died". 
    1. The heir then uses the signature and his secret key (for P) to create a secret key corresponding to Y and withdraw the funds.
    1. In case the "man" wants to change his heir, he should generate new Y using his new heir's public key and register it as step 3.   
This solution is not limited to getting cryptocurrency funds. The secret key for Y could be anything, like an input to a KDF or password generator.  
When the validator set changes such that the threshold public key X has to be changed, all active R will change causing all corresponding Y to change.
I have intentionally omitted the cryptographic details for brevity, please refer the blog post above.

1. Ink smart contract on Substrate
    1. Here the "man" registers a heir in a smart contract and locks up the funds he need his heir to have. 
    1. To signal the smart contract that he is alive, he periodically sends a heartbeat to it which the contract stores 
    and forgets the previously stored heartbeat time.
    1. The heir can anytime try to withdraw the funds. The contract will check whether the man has died by checking for 
    a recent heartbeat and if he's dead, the funds are transferred to the heir's address.
     
    Some improvements that should be made before going to production:
    1. The "man" should be able to update (decrease or increase) the funds (allotted to the heir) any time.
    1. The "man" should be able to set a custom heartbeat frequency and also update it any time. eg. rather then sending a heartbeat each week, he will send 1 each month or each 3 days. 
    1. The contract should be able to charge a small fee to the "man" for its service with each heartbeat.
    1. Support for multiple heirs.