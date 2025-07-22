#[macro_export]
macro_rules! assert_close {
    ($test:expr, $truth:expr, $threshold:expr) => {
        match ($test, $truth, $threshold) {
            (test, truth, threshold) => {
                let relative_error = (test - truth).abs() / truth.abs();
                if relative_error > threshold {
                    panic!(
                        "{}={} vs {}={}: relative error = {:e} > {:e}",
                        stringify!($test),
                        test,
                        stringify!($truth),
                        truth,
                        relative_error,
                        threshold
                    );
                }
            }
        }
    };
}
