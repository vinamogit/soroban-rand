# soroban-rand

This is work in progress.

This is a proposal for a library that add random capability to soroban.

# Compatibility

The implementation uses the SmallRng, no_std implementation of the rust rand crate. It uses the Xoshiro128PlusPlus algorithm internally, not cryptographicly robust.

# Randomness

Finding a good seed might be difficult in soroban deterministic environment. Here it uses parts of the contract id, the ledger sequence and timestamp and a nonce. 

# Questions

## Does the next random number be guessed?
Knowing the current state of the contract, it might be possible to predicte the next number. 

The range of this guess mainly depend on the time and if there are other call to the same contract during the same ledger.

