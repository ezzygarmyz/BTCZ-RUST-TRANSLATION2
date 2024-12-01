mod addrman;

use addrman::AddrMan;

fn main() {
    let addrman = AddrMan::load_from_file("peers.dat-minus-d-byte-header")
        .expect("Failed to load AddrMan data");

    addrman.display_addresses();
}
