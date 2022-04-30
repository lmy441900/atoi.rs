//! Dummy responses.

use alloc::vec::Vec;

pub(super) fn respond_dummy(url: &str) -> Vec<u8> {
    match url {
        "https://example.iota.org/health" => b"".as_slice(),
        "https://example.iota.org/api/v2/info" => br#"{
            "name": "HORNET",
            "version": "0.6.0-alpha",
            "status": {
              "isHealthy": true,
              "latestMilestone": {
                "index": 480,
                "timestamp": 1617802102,
                "milestoneId": "0xb59ff329113b0da14343707450cb28d41fa18b295deabc4beb3fc1b6e70f9d9e"
              },
              "confirmedMilestone": {
                "index": 480,
                "timestamp": 1617802102,
                "milestoneId": "0xb59ff329113b0da14343707450cb28d41fa18b295deabc4beb3fc1b6e70f9d9e"
              },
              "pruningIndex": 0
            },
            "metrics": {
              "messagesPerSecond": 17,
              "referencedMessagesPerSecond": 16.8,
              "referencedRate": 98.82352941176471
            },
            "protocol": {
              "networkName": "iota-testnet",
              "bech32HRP": "atoi",
              "tokenSupply": "2779530283277761",
              "protocolVersion": 2,
              "minPoWScore": 1000,
              "rentStructure": {
                "vByteCost": 500,
                "vByteFactorData": 1,
                "vByteFactorKey": 10
              }
            },
            "baseToken": {
              "name": "IOTA",
              "tickerSymbol": "MIOTA",
              "unit": "IOTA",
              "decimals": 0,
              "useMetricPrefix": true
            },
            "features": [
              "PoW"
            ],
            "plugins": [
              "indexer/v1"
            ]
          }"#
        .as_slice(),
        _ => b"".as_slice(),
    }
    .to_vec()
}
