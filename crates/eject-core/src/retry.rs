use serde::{Deserialize, Serialize};

/// Retry policy defaults from the plan.
/// - 3s, then 5s, then every 10s until 2-minute wall-clock deadline.
/// - One timer per scope, no overlapping native calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub first_delay_ms: u64,
    pub second_delay_ms: u64,
    pub steady_delay_ms: u64,
    pub deadline_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            first_delay_ms: 3_000,
            second_delay_ms: 5_000,
            steady_delay_ms: 10_000,
            deadline_ms: 120_000,
        }
    }
}

impl RetryPolicy {
    /// Returns the delay before attempt `attempt_index` (0-based) relative to the
    /// previous attempt completion, or None if the deadline would be exceeded.
    /// `elapsed_ms` is wall-clock time since the retry was armed.
    pub fn delay_for_attempt(&self, attempt_index: u32, elapsed_ms: u64) -> Option<u64> {
        if elapsed_ms >= self.deadline_ms {
            return None;
        }
        let delay = match attempt_index {
            0 => self.first_delay_ms,
            1 => self.second_delay_ms,
            _ => self.steady_delay_ms,
        };
        if elapsed_ms.saturating_add(delay) > self.deadline_ms {
            // Allow a final attempt only if we haven't passed deadline yet;
            // the attempt itself must start before/at deadline check in scheduler.
            // Return delay but caller must also enforce deadline.
            Some(delay.min(self.deadline_ms.saturating_sub(elapsed_ms)))
        } else {
            Some(delay)
        }
    }
}

/// Runtime state of an armed retry for a scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetryState {
    pub armed_at_unix_ms: i64,
    pub deadline_at_unix_ms: i64,
    pub attempt_count: u32,
    pub next_attempt_at_unix_ms: Option<i64>,
    pub in_flight: bool,
}

impl RetryState {
    pub fn new(now_unix_ms: i64, policy: RetryPolicy) -> Self {
        let first_delay = policy.first_delay_ms as i64;
        Self {
            armed_at_unix_ms: now_unix_ms,
            deadline_at_unix_ms: now_unix_ms + policy.deadline_ms as i64,
            attempt_count: 0,
            next_attempt_at_unix_ms: Some(now_unix_ms + first_delay),
            in_flight: false,
        }
    }

    pub fn is_expired(&self, now_unix_ms: i64) -> bool {
        now_unix_ms >= self.deadline_at_unix_ms
    }

    pub fn time_remaining_ms(&self, now_unix_ms: i64) -> u64 {
        (self.deadline_at_unix_ms - now_unix_ms).max(0) as u64
    }

    /// Called when an attempt completes with a retryable result.
    /// Schedules the next attempt or returns None if deadline reached.
    pub fn schedule_next(&mut self, now_unix_ms: i64, policy: RetryPolicy) -> Option<i64> {
        self.in_flight = false;
        self.attempt_count = self.attempt_count.saturating_add(1);
        let elapsed = (now_unix_ms - self.armed_at_unix_ms).max(0) as u64;
        if let Some(delay) = policy.delay_for_attempt(self.attempt_count, elapsed) {
            let next = now_unix_ms + delay as i64;
            if next <= self.deadline_at_unix_ms {
                self.next_attempt_at_unix_ms = Some(next);
                return Some(next);
            }
        }
        self.next_attempt_at_unix_ms = None;
        None
    }

    pub fn mark_in_flight(&mut self) {
        self.in_flight = true;
        self.next_attempt_at_unix_ms = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_delays() {
        let p = RetryPolicy::default();
        assert_eq!(p.delay_for_attempt(0, 0), Some(3_000));
        assert_eq!(p.delay_for_attempt(1, 3_000), Some(5_000));
        assert_eq!(p.delay_for_attempt(2, 8_000), Some(10_000));
        assert_eq!(p.delay_for_attempt(5, 60_000), Some(10_000));
    }

    #[test]
    fn deadline_stops_retry() {
        let p = RetryPolicy::default();
        assert_eq!(p.delay_for_attempt(10, 120_000), None);
        assert_eq!(p.delay_for_attempt(10, 121_000), None);
    }

    #[test]
    fn retry_state_schedule() {
        let p = RetryPolicy::default();
        let mut rs = RetryState::new(1_000, p);
        assert_eq!(rs.next_attempt_at_unix_ms, Some(4_000));
        rs.mark_in_flight();
        let next = rs.schedule_next(4_100, p);
        assert_eq!(next, Some(9_100)); // 5s after completion
        assert_eq!(rs.attempt_count, 1);
    }
}