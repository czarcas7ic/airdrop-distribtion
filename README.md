# Rust Airdrop Distribution

## Instructions

```
cd $HOME
git clone https://github.com/czarcas7ic/airdrop-distribtion.git
cd $HOME/airdrop-distribtion
```

Ensure you have your airdrop.csv. This file is obtained from osmosisd and contains a list of addresses and how much each address has staked.

Provide the path to this csv as well as the total number of tokens that will be distributed by your airdrop. In this example, I am distributing 5000025
tokens to all users listed in my airdrop.csv file:

```
cargo run /Users/czar/Desktop/airdrop.csv 5000025
```
