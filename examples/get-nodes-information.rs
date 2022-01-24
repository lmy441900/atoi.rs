// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client as iota;

fn main() {
    let client = iota::Client::new();
    let info = client.get_nodes_information();

    client
        .nodes
        .iter()
        .zip(info.iter())
        .for_each(|(url, resp)| {
            match resp {
                Ok(info) => println!("{}: {:?}", url, info),
                Err(err) => println!("{}: {}", url, err),
            };
        });
}
