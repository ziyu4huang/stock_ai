use crate::data::types::{AimingLevel, AimingResult, VolumeSpike, EnergyPulse, PulsePattern};

/// Compute the 3-tier aiming score.
///
/// Score breakdown:
/// - Base: spike z-score mapped to 0-60
/// - Geo bonus: +0 to +20 (broker concentration)
/// - Sector bonus: +0 to +10 (sector cluster)
/// - Order book bonus: +0 to +10
/// - Penalties: various deductions
/// - Total clamped 0-100
pub fn compute_aiming(
    spike: Option<&VolumeSpike>,
    pulse: Option<&EnergyPulse>,
    geo_bonus: i32,
    sector_count: usize,
    orderbook_imbalance: f64,
) -> AimingResult {
    // Base score from spike
    let base = match spike {
        Some(s) => {
            let z = s.z_score;
            if z >= 6.0 { 60 }
            else if z >= 5.0 { 50 }
            else if z >= 4.0 { 40 }
            else { 30 } // z >= 3.0 threshold
        }
        None => 0,
    };

    // Energy pulse confirmation bonus
    let pulse_bonus = match pulse.map(|p| &p.pattern) {
        Some(PulsePattern::Staircase) => 10,
        Some(PulsePattern::Accumulation) => 5,
        Some(PulsePattern::Distribution) => 0,
        Some(PulsePattern::Single) => -5, // likely noise
        _ => 0,
    };

    // Sector cluster bonus
    let sector_bonus = if sector_count >= 3 { 10 } else if sector_count >= 2 { 5 } else { 0 };

    // Order book imbalance bonus (bid/ask ratio)
    let ob_bonus = if orderbook_imbalance > 2.0 {
        10
    } else if orderbook_imbalance > 1.5 {
        7
    } else if orderbook_imbalance > 1.0 {
        3
    } else {
        0
    };

    let total = (base + pulse_bonus + geo_bonus + sector_bonus + ob_bonus).clamp(0, 100);

    AimingResult {
        base_score: base,
        geo_bonus,
        sector_bonus,
        orderbook_bonus: ob_bonus,
        pulse_bonus,
        penalty: 0,
        total_score: total,
        level: AimingLevel::from_score(total),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::VolumeSpike;

    fn make_spike(z: f64) -> VolumeSpike {
        VolumeSpike {
            symbol: "2330.TW".into(),
            timestamp: 1000,
            volume: 5000,
            mean: 1000.0,
            sigma: 300.0,
            z_score: z,
        }
    }

    #[test]
    fn test_aiming_critical() {
        let spike = make_spike(5.5);
        let result = compute_aiming(Some(&spike), None, 20, 3, 2.5);
        assert_eq!(result.level, AimingLevel::Critical);
    }

    #[test]
    fn test_aiming_low() {
        let result = compute_aiming(None, None, 0, 0, 0.5);
        assert_eq!(result.level, AimingLevel::Low);
    }

    #[test]
    fn test_aiming_medium() {
        let spike = make_spike(3.2);
        // base=30 + geo=10 + sector=5 + ob=7 = 52 → Medium
        let result = compute_aiming(Some(&spike), None, 10, 2, 1.6);
        assert_eq!(result.level, AimingLevel::Medium, "score={}", result.total_score);
    }
}
