# Prove L2 block inclusion

Goal: Given an untrusted transaction hash, we prove that the transaction is actually in an L2 block that has been posted to L1

### High level flow

tx hash
 └─► in L2 block ▸ recompute txTrie → matches transactionsRoot
      └─► block header ▸ keccak(RLP(header)) → blockHash
           └─► in batch calldata on L1 ▸ hash frame list → batchHash
                └─► batchHash part of outputRoot pre-image → outputRoot
                     └─► outputRoot stored in L2OutputOracle (Helios-verified)
                          └─► L1 block finalized + >7 days → canonical finality

### Step 1: Verify Transactions Root = transaction is within the L2 block

Get the transaction object (set of deterministic, priK-signed, RLP-encoded bytes produced by the *wallet)*

`GET` transactionReceiptByHash and save block number ✅ and save transaction object 
- Call RPC-getBlockByNumber ✅ and receive a `block` object ✅ with a `block.transactionsRoot` . ✅
- Take the untrusted transaction
- RLP-encode the transaction object
- take the keccak256 hash of the rlp-encoded transaction object, i.e., `let self_derived_hash = hash_keccak256(to_rlp(tx_obj))` 
- `asserteq!(tx_hash, self_derived_hash)` 

With tx_hash
- for transaction in block, rlp_encode(transaction)
- based on rlp encoding, identify branch of transactions trie that corresponds to our transaction
- recompute txTrie
- `asserteq!(block.transactionsRoot, self_derived_root)`