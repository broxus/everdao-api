-- Add migration script here
CREATE TABLE transfers
(
    ton_message_hash           BYTEA,
    ton_transaction_hash       BYTEA,
    contract_address           VARCHAR UNIQUE,
    event_index                INT,
    eth_transaction_hash       VARCHAR,
    user_address               VARCHAR NOT NULL,
    volume_exec                DECIMAL NOT NULL,
    ton_token_address          VARCHAR NOT NULL,
    eth_token_address          VARCHAR NOT NULL,
    transfer_kind              VARCHAR NOT NULL,
    status                     VARCHAR NOT NULL,
    required_votes             INT     NOT NULL,
    confirm_votes              INT     NOT NULL,
    reject_votes               INT     NOT NULL,
    burn_callback_timestamp_lt BIGINT,
    timestamp_block_updated_at INT     NOT NULL,
    timestamp_block_created_at INT     NOT NULL,
    graphql_timestamp          INT,
    chain_id                   INTEGER NOT NULL,
    updated_at                 BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    created_at                 BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    PRIMARY KEY (user_address, timestamp_block_created_at),
    UNIQUE (event_index, eth_transaction_hash)
);

CREATE TABLE relay_events
(
    relay_user_address    VARCHAR NOT NULL,
    contract_address      VARCHAR NOT NULL,
    ton_pub_key           BYTEA   NOT NULL,
    transfer_user_address VARCHAR NOT NULL,
    status                VARCHAR NOT NULL,
    volume_exec           DECIMAL NOT NULL,
    transfer_kind         VARCHAR NOT NULL,
    currency_address      VARCHAR NOT NULL,
    timestamp_block       INT     NOT NULL,
    PRIMARY KEY (relay_user_address, contract_address, ton_pub_key, transfer_user_address)
);


CREATE TABLE vault_info
(
    vault_address      VARCHAR NOT NULL,
    ton_token_address  VARCHAR NOT NULL,
    eth_token_address  VARCHAR NOT NULL,
    ton_currency_scale INTEGER NOT NULL,
    ton_currency       VARCHAR NOT NULL,
    chain_id           INTEGER NOT NULL,
    proxy              VARCHAR NOT NULL,
    PRIMARY KEY (vault_address, ton_token_address, chain_id)
);