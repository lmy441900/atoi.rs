// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client as iota;

fn main() {
    let client = iota::Client::new();
    let healths = client.get_nodes_health();

    client
        .nodes
        .iter()
        .zip(healths.iter())
        .for_each(|(url, resp)| {
            match resp {
                Ok(health) => {
                    println!("{}: {}", url, if *health { "healthy" } else { "unhealthy" })
                }
                Err(err) => println!("{}: {}", url, err),
            };
        });
}
