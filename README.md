#Token Contract with Freeze Functionality
This project implements a token contract on the Stellar testnet with additional functionality to freeze and unfreeze token accounts.

#Contract Address
Contract Address: GB6FO362B3FTEOCXMPGAB7WHARIFPDIKZQX7TNAUOWVWZXJ7WOYWGDP3

#How to Interact
To interact with the contract, you can use tools like Stellar Laboratory or Stellar SDKs. Here's how you can use the added functionality:

#Freeze Account
To freeze tokens for a specific account, call the freeze_account function with the target account's address.

#Unfreeze Account
To unfreeze tokens for a specific account, call the unfreeze_account function with the target account's address.

#Transfer Tokens
When transferring tokens between accounts, the contract automatically checks if either the sender's or receiver's account is frozen. If either account is frozen, the transfer will be rejected.
