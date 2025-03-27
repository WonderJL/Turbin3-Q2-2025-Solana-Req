use solana_idlgen::idlgen;
idlgen!({
    "version": "0.1.0",
    "name": "Turbin3_prereq",
    "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa",
    "instructions": [
      {
            "name": "complete",
            "accounts": [
                {
                    "name": "signer",
                    "isMut": true,
                    "isSigner": true
                },
                {
                    "name": "prereq",
                    "isMut": true,
                    "isSigner": false,
                    "pda": {
                        "seeds": [
                            {
                                "kind": "const",
                                "value": [ 112, 114, 101, 114, 101, 113]
                            },
                            {
                                "kind": "account",
                                "path": "signer"
                            }
                        ]
                    }
                },
                {
                    "name": "system_program",
                    "isMut": false,
                    "isSigner": false,
                    "address": "11111111111111111111111111111111"
                }
            ],
            "args": [
                {
                    "name": "github",
                    "type": "bytes"
                }
            ]
        },
        {
            "name": "update",
            "accounts": [
                {
                    "name": "signer",
                    "isMut": true,
                    "isSigner": true
                },
                {
                    "name": "prereq",
                    "isMut": true,
                    "isSigner": false,
                     "pda": {
                        "seeds": [
                            {
                                "kind": "const",
                                "value": [210, 203, 168, 103, 251, 233, 204, 6]
                            },
                            {
                                "kind": "account",
                                "path": "signer"
                            }
                        ]
                    }
                },
                {
                    "name": "system_program",
                    "isMut": false,
                    "isSigner": false,
                    "address": "11111111111111111111111111111111"
                }
            ],
            "args": [
                {
                    "name": "github",
                    "type": "bytes"
                }
            ]
        }
    ],
    "accounts": [{
            "name": "SolanaCohort5Account",
            "type": {
                "kind": "struct",
                "fields": [
                    {
                        "name": "github",
                        "type": "bytes"
                    },
                    {
                        "name": "key",
                        "type": "pubkey"
                    }
                ]
            }
        }],
    "metadata": {
       "name": "turbine_prereq",
        "version": "0.1.0",
        "spec": "0.1.0",
        "description": "Created with Anchor",
        "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa"
      }
});
