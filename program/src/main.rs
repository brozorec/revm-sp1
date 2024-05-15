#![no_main]

use revm_precompile::bn128::{run_add, add::BYZANTIUM_ADD_GAS_COST};
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let input = hex::decode(
        "\
            18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9\
            063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f37266\
            07c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed\
            06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7",
    )
        .unwrap();
    let expected = hex::decode(
        "\
            2243525c5efd4b9c3d3c45ac0ca3fe4dd85e830a4ce6b65fa1eeaee202839703\
            301d1d33be6da8e509df21cc35964723180eed7532537db9ae5e7d48f195c915",
    )
        .unwrap();

    println!("cycle-tracker-start: run_add");
    let (_, res) = run_add(&input, BYZANTIUM_ADD_GAS_COST, 500).unwrap();
    println!("cycle-tracker-end: run_add");

    println!("{:?}", res);

    assert_eq!(res, expected);
}
