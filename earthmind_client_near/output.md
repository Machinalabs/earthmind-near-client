TRANSACTION STATUS: RpcTransactionResponse { final_execution_outcome: Some(FinalExecutionOutcome(FinalExecutionOutcome { status: SuccessValue(''), transaction: SignedTransactionView { signer_id: AccountId("halcalag.testnet"), public_key: ed25519:7YoZgn4eT6UPLceVSE1qToPQma26mABwuqVKKurR78Zd, nonce: 168711926000002, receiver_id: AccountId("halcalag.testnet"), actions: [DeployContract { code: [124, 177, 251, 97, 5, 226, 158, 98, 90, 160, 126, 43, 7, 165, 97, 186, 238, 188, 157, 51, 45, 184, 26, 89, 204, 114, 177, 89, 192, 80, 27, 88] }], signature: ed25519:3LS4xscVaJdGwZV8s21CmTunD7CjXJ6cEqWAZ7fhkfH6bPcExcU5nE2gZ1AbfPvU4dVCqWBhzmok21rpJhbqH3Bv, hash: 3jnfi85GhTcX3ZdPxF2EtS7vQddAwzAAooGwmffK5fgD }, transaction_outcome: ExecutionOutcomeWithIdView { proof: [MerklePathItem { hash: GL1Z9hfGTLCwatbNZfdWn9CPq9fjdRFXY55saWimiXpg, direction: Right }], block_hash: rp7tS3ZwYeDAjbia8WnnhnNtBTzLgX2gHVZtRWWDX34, id: 3jnfi85GhTcX3ZdPxF2EtS7vQddAwzAAooGwmffK5fgD, outcome: ExecutionOutcomeView { logs: [], receipt_ids: [66zASKGJHFTaLfWGN3SVxCKztCpcZhMPob5Waz9JtmL1], gas_burnt: 10848388551673, tokens_burnt: 1084838855167300000000, executor_id: AccountId("halcalag.testnet"), status: SuccessReceiptId(66zASKGJHFTaLfWGN3SVxCKztCpcZhMPob5Waz9JtmL1), metadata: ExecutionMetadataView { version: 1, gas_profile: None } } }, receipts_outcome: [ExecutionOutcomeWithIdView { proof: [MerklePathItem { hash: Cb7Kqz4dmWZ5cV9etKqSN3nC1yfgPeXkXxGqhAAZteij, direction: Left }], block_hash: rp7tS3ZwYeDAjbia8WnnhnNtBTzLgX2gHVZtRWWDX34, id: 66zASKGJHFTaLfWGN3SVxCKztCpcZhMPob5Waz9JtmL1, outcome: ExecutionOutcomeView { logs: [], receipt_ids: [], gas_burnt: 100337430858688, tokens_burnt: 10033743085868800000000, executor_id: AccountId("halcalag.testnet"), status: SuccessValue(''), metadata: ExecutionMetadataView { version: 3, gas_profile: Some([]) } } }] })), final_execution_status: Final }
TRANSACTION RpcTransactionResponse {
    final_execution_outcome: Some(
        FinalExecutionOutcome(
            FinalExecutionOutcome {
                status: SuccessValue(''),
                transaction: SignedTransactionView {
                    signer_id: AccountId(
                        "hasselalcalag.testnet",
                    ),
                    public_key: ed25519:AbAKj1EWxKkVtYoZDJbnj9q7cPQJSbwzmnLZPijToarj,
                    nonce: 168739530000001,
                    receiver_id: AccountId(
                        "halcalag.testnet",
                    ),
                    actions: [
                        FunctionCall {
                            method_name: "set_greeting",
                            args: FunctionArgs(
                                [
                                    123,
                                    34,
                                    103,
                                    114,
                                    101,
                                    101,
                                    116,
                                    105,
                                    110,
                                    103,
                                    34,
                                    58,
                                    34,
                                    104,
                                    97,
                                    115,
                                    115,
                                    101,
                                    108,
                                    97,
                                    108,
                                    99,
                                    97,
                                    108,
                                    97,
                                    103,
                                    34,
                                    125,
                                ],
                            ),
                            gas: 30000000000000,
                            deposit: 0,
                        },
                    ],
                    signature: ed25519:4BsXGiE3VRkffW7FW5n8HDa5yugYCYDzgxFzWHUtUZ1tELjMYUxrqModb6aJY2gdqoG9mpEsehqrqjAKf8arFdrx,
                    hash: 3xn4mDr5v5H2AG8EBRvZHFc6VqKRKcWufUaRwK2pZCqs,
                },
                transaction_outcome: ExecutionOutcomeWithIdView {
                    proof: [],
                    block_hash: nSDTWhAwEA2gKkWdt7FWV2zPdRxxyBReUthcfCuN7wa,
                    id: 3xn4mDr5v5H2AG8EBRvZHFc6VqKRKcWufUaRwK2pZCqs,
                    outcome: ExecutionOutcomeView {
                        logs: [],
                        receipt_ids: [
                            EMNWc16mneqK8Zy1227U9bwAhefjbhNLJT4MB6CJuthW,
                        ],
                        gas_burnt: 309966848600,
                        tokens_burnt: 30996684860000000000,
                        executor_id: AccountId(
                            "hasselalcalag.testnet",
                        ),
                        status: SuccessReceiptId(EMNWc16mneqK8Zy1227U9bwAhefjbhNLJT4MB6CJuthW),
                        metadata: ExecutionMetadataView {
                            version: 1,
                            gas_profile: None,
                        },
                    },
                },
                receipts_outcome: [
                    ExecutionOutcomeWithIdView {
                        proof: [],
                        block_hash: HWWvEvr9zkzsd6Pmmp6YnMUFvSyuBuUfQX8zmhMHkR5P,
                        id: EMNWc16mneqK8Zy1227U9bwAhefjbhNLJT4MB6CJuthW,
                        outcome: ExecutionOutcomeView {
                            logs: [
                                "EVENT_JSON:{\"standard\":\"emip001\",\"version\":\"1.0.0\",\"event\":\"register_greeting\",\"data\":[{\"account\":\"hasselalcalag.testnet\"}]}",
                            ],
                            receipt_ids: [
                                AWdYv8De8ewWjiVGCejjTGNHmHyCauyzhA7i5ZZMCC8X,
                            ],
                            gas_burnt: 2991868011957,
                            tokens_burnt: 299186801195700000000,
                            executor_id: AccountId(
                                "halcalag.testnet",
                            ),
                            status: SuccessValue(''),
                            metadata: ExecutionMetadataView {
                                version: 3,
                                gas_profile: Some(
                                    [
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "BASE",
                                            gas_used: 3177217332,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "CONTRACT_LOADING_BASE",
                                            gas_used: 35445963,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "CONTRACT_LOADING_BYTES",
                                            gas_used: 1687674154465,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "LOG_BASE",
                                            gas_used: 3543313050,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "LOG_BYTE",
                                            gas_used: 1636650084,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "READ_CACHED_TRIE_NODE",
                                            gas_used: 2280000000,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "READ_MEMORY_BASE",
                                            gas_used: 10439452800,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "READ_MEMORY_BYTE",
                                            gas_used: 574001283,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "READ_REGISTER_BASE",
                                            gas_used: 7551495558,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "READ_REGISTER_BYTE",
                                            gas_used: 5815158,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "STORAGE_READ_BASE",
                                            gas_used: 56356845750,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "STORAGE_READ_KEY_BYTE",
                                            gas_used: 154762665,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "STORAGE_READ_VALUE_BYTE",
                                            gas_used: 56110050,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "STORAGE_WRITE_BASE",
                                            gas_used: 64196736000,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "STORAGE_WRITE_EVICTED_BYTE",
                                            gas_used: 321173070,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "STORAGE_WRITE_KEY_BYTE",
                                            gas_used: 352414335,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "STORAGE_WRITE_VALUE_BYTE",
                                            gas_used: 527315163,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "TOUCHING_TRIE_NODE",
                                            gas_used: 177121515186,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "UTF8_DECODING_BASE",
                                            gas_used: 3111779061,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "UTF8_DECODING_BYTE",
                                            gas_used: 36155979396,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "WASM_INSTRUCTION",
                                            gas_used: 25303038024,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "WRITE_MEMORY_BASE",
                                            gas_used: 11215179444,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "WRITE_MEMORY_BYTE",
                                            gas_used: 204282900,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "WRITE_REGISTER_BASE",
                                            gas_used: 11462089944,
                                        },
                                        CostGasUsed {
                                            cost_category: "WASM_HOST_COST",
                                            cost: "WRITE_REGISTER_BYTE",
                                            gas_used: 262307916,
                                        },
                                    ],
                                ),
                            },
                        },
                    },
                    ExecutionOutcomeWithIdView {
                        proof: [],
                        block_hash: FTrHC7Gz2T51WGgzcrDFEQRaiSivQBkLKkPXC7J1V9qg,
                        id: AWdYv8De8ewWjiVGCejjTGNHmHyCauyzhA7i5ZZMCC8X,
                        outcome: ExecutionOutcomeView {
                            logs: [],
                            receipt_ids: [],
                            gas_burnt: 223182562500,
                            tokens_burnt: 0,
                            executor_id: AccountId(
                                "hasselalcalag.testnet",
                            ),
                            status: SuccessValue(''),
                            metadata: ExecutionMetadataView {
                                version: 3,
                                gas_profile: Some(
                                    [],
                                ),
                            },
                        },
                    },
                ],
            },
        ),
    ),
    final_execution_status: Final,
}
RESULT CHANGES Ok(RpcStateChangesInBlockResponse { block_hash: HWWvEvr9zkzsd6Pmmp6YnMUFvSyuBuUfQX8zmhMHkR5P, changes: [] })
