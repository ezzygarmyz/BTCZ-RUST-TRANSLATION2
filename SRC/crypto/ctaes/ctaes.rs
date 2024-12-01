pub const AES_BLOCK_SIZE: usize = 16;

pub struct AesContext {
    rounds: usize,
    keys: Vec<[u8; AES_BLOCK_SIZE]>,
}

impl AesContext {
    pub fn new(key: &[u8]) -> Self {
        let key_length = key.len();
        let rounds = match key_length {
            16 => 10,
            24 => 12,
            32 => 14,
            _ => panic!("Invalid AES key length: {}", key_length),
        };

        let keys = expand_key(key, rounds);
        AesContext { rounds, keys }
    }

    pub fn encrypt(&self, block: &mut [u8; AES_BLOCK_SIZE]) {
        add_round_key(block, &self.keys[0]);

        for round in 1..self.rounds {
            sub_bytes(block);
            shift_rows(block);
            mix_columns(block);
            add_round_key(block, &self.keys[round]);
        }

        sub_bytes(block);
        shift_rows(block);
        add_round_key(block, &self.keys[self.rounds]);
    }

    pub fn decrypt(&self, block: &mut [u8; AES_BLOCK_SIZE]) {
        add_round_key(block, &self.keys[self.rounds]);

        for round in (1..self.rounds).rev() {
            inv_shift_rows(block);
            inv_sub_bytes(block);
            add_round_key(block, &self.keys[round]);
            inv_mix_columns(block);
        }

        inv_shift_rows(block);
        inv_sub_bytes(block);
        add_round_key(block, &self.keys[0]);
    }
}

fn expand_key(key: &[u8], rounds: usize) -> Vec<[u8; AES_BLOCK_SIZE]> {
    // Key expansion logic
    unimplemented!()
}

fn add_round_key(block: &mut [u8; AES_BLOCK_SIZE], round_key: &[u8; AES_BLOCK_SIZE]) {
    for i in 0..AES_BLOCK_SIZE {
        block[i] ^= round_key[i];
    }
}

fn sub_bytes(block: &mut [u8; AES_BLOCK_SIZE]) {
    // SubBytes logic
    unimplemented!()
}

fn shift_rows(block: &mut [u8; AES_BLOCK_SIZE]) {
    // ShiftRows logic
    unimplemented!()
}

fn mix_columns(block: &mut [u8; AES_BLOCK_SIZE]) {
    // MixColumns logic
    unimplemented!()
}

fn inv_sub_bytes(block: &mut [u8; AES_BLOCK_SIZE]) {
    // Inverse SubBytes logic
    unimplemented!()
}

fn inv_shift_rows(block: &mut [u8; AES_BLOCK_SIZE]) {
    // Inverse ShiftRows logic
    unimplemented!()
}

fn inv_mix_columns(block: &mut [u8; AES_BLOCK_SIZE]) {
    // Inverse MixColumns logic
    unimplemented!()
}
