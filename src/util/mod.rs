use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub mod lockfile;
pub mod logger;
pub mod macros;

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    fn range(&mut self, min: usize, max: usize) -> usize {
        if min >= max {
            return min;
        }
        let r = self.next_u64() as usize;
        min + (r % (max - min))
    }
}

pub fn pad_activity_field(field: &mut Option<String>) {
    if let Some(s) = field {
        let current_len = s.chars().count();
        if current_len >= 128 {
            return;
        }

        let max_len = 128;
        let available = max_len - current_len;

        let mut lcg = Lcg::new();
        let pad_len = if available < 3 {
            available
        } else {
            lcg.range(3, available + 1)
        };

        let padding: String = (0..pad_len).map(|_| ' ').collect();
        s.push_str(&padding);
    }
}

pub fn now() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}
