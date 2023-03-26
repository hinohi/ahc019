#[derive(Debug, Copy, Clone, Default)]
pub struct McParams {
    pub mc_run: u64,
    pub erase_small_th: usize,
    pub erase_shared_p: f64,
}

impl McParams {
    pub fn opt(d: u8) -> McParams {
        match d {
            // -0.8665073454427983
            5 => McParams {
                erase_shared_p: 0.7641974640494824,
                erase_small_th: 5,
                mc_run: 45,
            },
            // -1.0465880873030173
            6 => McParams {
                erase_shared_p: 0.5271543071699281,
                erase_small_th: 8,
                mc_run: 100,
            },
            // -1.062290396485194
            7 => McParams {
                erase_shared_p: 0.3526343942727514,
                erase_small_th: 11,
                mc_run: 64,
            },
            // -1.0776700405881754
            8 => McParams {
                erase_shared_p: 0.6445607027301276,
                erase_small_th: 14,
                mc_run: 78,
            },
            // -1.114248273423888
            9 => McParams {
                erase_shared_p: 0.5778111909809597,
                erase_small_th: 21,
                mc_run: 30,
            },
            // -1.2071752742275605
            10 => McParams {
                erase_shared_p: 0.5016439719628357,
                erase_small_th: 24,
                mc_run: 24,
            },
            // -1.047992358709207
            11 => McParams {
                erase_shared_p: 0.5337580963073999,
                erase_small_th: 29,
                mc_run: 41,
            },
            // -1.4402375267601855
            12 => McParams {
                erase_shared_p: 0.5627626458269703,
                erase_small_th: 39,
                mc_run: 47,
            },
            // -1.429345287977688
            13 => McParams {
                erase_shared_p: 0.653866755894219,
                erase_small_th: 48,
                mc_run: 24,
            },
            // -1.6196117306757933
            14 => McParams {
                erase_shared_p: 0.7227385983082774,
                erase_small_th: 60,
                mc_run: 17,
            },
            _ => unreachable!(),
        }
    }
}
