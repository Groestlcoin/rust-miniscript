extern crate miniscript;

use bitcoind::bitcoincore_rpc::RpcApi;
use bitcoind::BitcoinD;
use miniscript::groestlcoin;

pub mod test_util;

// Launch an instance of groestlcoind with
pub fn setup() -> BitcoinD {
    // Create env var BITCOIND_EXE_PATH to point to the ../groestlcoind/bin/groestlcoind binary
    let key = "BITCOIND_EXE";
    let curr_dir_path = std::env::current_dir().unwrap();
    let bitcoind_path = curr_dir_path.join("bin").join("bitcoind");
    std::env::set_var(key, bitcoind_path);

    let exe_path = bitcoind::exe_path().unwrap();
    let bitcoind = bitcoind::BitcoinD::new(exe_path).unwrap();
    let cl = &bitcoind.client;
    // generate to an address by the wallet. And wait for funds to mature
    let addr = cl.get_new_address(None, None).unwrap();
    let blks = cl.generate_to_address(101, &addr).unwrap();
    assert_eq!(blks.len(), 101);

    assert_eq!(
        cl.get_balance(Some(1) /*min conf*/, None).unwrap(),
        groestlcoin::Amount::from_sat(100_000_000 * 50)
    );
    bitcoind
}
