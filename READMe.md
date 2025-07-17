# Voting Program

A Solana Anchor program for multiple-choice voting with one-member-one-vote and admin-controlled lifecycle.

## Features

* Multiple options (2–10)
* One wallet, one vote
* Open to all Solana addresses
* Admin creates, opens, and closes proposals

## Instructions

* **initialize\_governance**: setup admin config
* **create\_proposal**: admin creates a draft proposal
* **open\_proposal**: admin activates voting
* **cast\_vote**: any wallet casts a vote
* **close\_proposal**: admin closes voting

## Accounts & PDAs

* **GovernanceConfig** PDA: `[b"governance"]`
* **Proposal** PDA: `[b"proposal", config_pubkey, id]`
* **Ballot** PDA: `[b"ballot", proposal_pubkey, voter_pubkey]`

## Build & Deploy

```bash
anchor build
anchor deploy
```

## Test

```bash
anchor test
```
