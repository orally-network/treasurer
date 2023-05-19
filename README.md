# treasurer

## Deploy
```sh
export ENCODED_INIT_CONFIG=$(didc encode '(record {token_addr="338662C6e113aD9CfA4E2e755931643D8Cf1884B"; chain_rpc="https://sepolia.infura.io/v3/d20be327500c45819a1a3b850daec0e2"; siwe_signer_canister=principal "be2us-64aaa-aaaaa-qaabq-cai"; key_name="dfx_test_key"; chain_id=11155111:nat; treasurer="E86C4A45C1Da21f8838a1ea26Fc852BD66489ce9"})')
dfx deploy treasurer --argument "$ENCODED_INIT_CONFIG" --argument-type raw
dfx canister call treasurer init_controllers
```

## Usage
```sh
dfx canister call treasurer register_taxpayer '("service.org wants you to sign in with your Ethereum account:
0xE86C4A45C1Da21f8838a1ea26Fc852BD66489ce9


URI: https://service.org/login
Version: 1
Chain ID: 11155111
Nonce: 00000000
Issued At: 2023-05-04T18:39:24Z", "fa7b336d271b7ed539b6db3034d57be294ef889b42534fa95689afd0989ab6d27878c837a14ed1b4c3ab6b7052180ce87198934cb7712a81ea413fd8ebb29e8c1c")'

dfx canister call treasurer deposit '(record {amount=1:nat; taxpayer="0xE86C4A45C1Da21f8838a1ea26Fc852BD66489ce9"; deposit_type=variant {Erc20}})'
```