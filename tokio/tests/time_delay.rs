#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]

use tokio::time::{self, Duration, Instant};

macro_rules! assert_elapsed {
    ($now:expr, $ms:expr) => {{
        let elapsed = $now.elapsed();
        let lower = ms($ms);

        // Handles ms rounding
        assert!(
            elapsed >= lower && elapsed <= lower + ms(1),
            "actual = {:?}, expected = {:?}",
            elapsed, lower);
    }};
}

#[tokio::test]
async fn immediate_delay() {
    time::pause();

    let now = Instant::now();

    // Ready!
    time::delay_until(now).await;
    assert_elapsed!(now, 0);
}

#[tokio::test]
async fn delayed_delay_level_0() {
    time::pause();

    for &i in &[1, 10, 60] {
        let now = Instant::now();

        time::delay_until(now + ms(i)).await;

        assert_elapsed!(now, i);
    }
}

#[tokio::test]
async fn sub_ms_delayed_delay() {
    time::pause();

    for _ in 0..5 {
        let now = Instant::now();
        let deadline = now + ms(1) + Duration::new(0, 1);

        time::delay_until(deadline).await;

        assert_elapsed!(now, 1);
    }
}

#[tokio::test]
async fn delayed_delay_wrapping_level_0() {
    time::pause();

    time::delay_for(ms(5)).await;

    let now = Instant::now();
    time::delay_until(now + ms(60)).await;

    assert_elapsed!(now, 60);
}


fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}
