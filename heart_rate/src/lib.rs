use time::PrimitiveDateTime;

const MILLISECONDS_IN_SECOND: f32 = 1_000.0;

pub fn hr_variability_from_hr(
    heart_rates: impl IntoIterator<Item = (PrimitiveDateTime, u8)>,
) -> impl Iterator<Item = (PrimitiveDateTime, std::time::Duration)> {
    heart_rates.into_iter().map(|(time, rate)| {
        (
            time,
            std::time::Duration::from_millis((MILLISECONDS_IN_SECOND / rate as f32) as u64),
        )
    })
}
