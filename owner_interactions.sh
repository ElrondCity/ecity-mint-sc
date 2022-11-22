#--------TECHNICAL--------

ADDRESS="erd1qqqqqqqqqqqqqpgqg2hx932h92yxwsx9gs4ukv8n7uad9x89l5nscwc6u7"
OWNER_ADDRESS="erd1hm5dkcrp6xg3573ndl7n4a3x97u4dysa3ef6e7lgee8j3vz5l5nsa34h04"
PRIVATE_KEY=(--keyfile=erd1hm5dkcrp6xg3573ndl7n4a3x97u4dysa3ef6e7lgee8j3vz5l5nsa34h04.json --passfile=.passfile)
PASSFILE=--passfile=.passfile
PROXY=https://devnet-api.elrond.com
CHAIN_ID=D

deploy() {
    erdpy --verbose contract deploy --bytecode output/ecity_test.wasm --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --metadata-not-upgradeable --send --outfile="deploy.interaction.json" || return

    TRANSACTION=$(erdpy data parse --file="deploy.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=ecity_test-address-devnet --value=${ADDRESS}
    erdpy data store --key=ecity_test-deployTransaction-devnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
   echo "Upgrading Smart Contract address: ${ADDRESS}"
   erdpy --verbose contract upgrade ${ADDRESS} --bytecode output/ecity_test.wasm --recall-nonce ${PRIVATE_KEY} --gas-limit=500000000 --proxy=${PROXY} --chain=${CHAIN_ID} --send

   echo ""
   echo "Smart contract address: ${ADDRESS}"
}

# Should be called first multiple times to set the vesting schedule
vestingPush() {
    amount=$(echo "scale=0; (${1}*10^18)/1" | bc -l)

    erdpy --verbose contract call ${ADDRESS} --recall-nonce ${PRIVATE_KEY}\
        --gas-limit=50000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function "episodeVestingPush" \
        --arguments ${amount} \
        --send

    echo $?
}

# Should be called second to set the router address
setRouter() {
    router="0x$(erdpy wallet bech32 --decode ${1})"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce ${PRIVATE_KEY}\
        --gas-limit=50000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function "setRouter" \
        --arguments ${router} \
        --send

    echo $?
}

# Issues the token. Should be called third.
issueToken() {
    price=$(echo "scale=0; (${1}*10^18)/1" | bc -l) # Lets you enter it as 0.05
    token_name="0x$(echo -n ${2} | xxd -p -u | tr -d '\n')"
    token_ticker="0x$(echo -n ${3} | xxd -p -u | tr -d '\n')"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce ${PRIVATE_KEY}\
        --gas-limit=500000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --value ${price} \
        --function "issueToken" \
        --arguments ${price} ${token_name} ${token_ticker} \
        --send

    echo $?
}

# Premint the token and sends it to the given address. Can only be called once.
premint() {
    amount=$(echo "scale=0; (${1}*10^18)/1" | bc -l)
    to="0x$(erdpy wallet bech32 --decode ${2})"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce ${PRIVATE_KEY}\
        --gas-limit=50000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function "premint" \
        --arguments ${amount} ${to} \
        --send

    echo $?
}

# Locks the router address so that it cannot be changed. Acts as a security for the community against Team abuse.
lock_router() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce ${PRIVATE_KEY}\
        --gas-limit=50000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function "lockRouter" \
        --send

    echo $?
}

# This is a public endpoint
mint() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce ${PRIVATE_KEY}\
        --gas-limit=50000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function "mint" \
        --send

    echo $?
}