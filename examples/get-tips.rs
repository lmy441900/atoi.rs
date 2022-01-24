// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client as iota;

fn main() {
    let client = iota::Client::new();
    let tips = client.get_tips().unwrap();
    tips.iter().for_each(|tip| println!("{}", tip));
}
