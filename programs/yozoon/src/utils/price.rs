pub fn is_price_fresh(publish_time: i64, current_time: i64, max_staleness: i64) -> bool {
    current_time - publish_time <= max_staleness
} 