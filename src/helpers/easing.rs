use bevy::ecs::component::Component;
use num_traits::Float;
use std::f32::consts::PI;

/// Easing functions encapsulated in an enum.
/// Each variant represents a different easing curve.
#[derive(Component, Copy, Clone, Default, Debug, PartialEq)]
pub enum Easing {
    #[default]
    Linear,
    // Quad
    QuadIn,
    QuadOut,
    QuadInOut,
    // Cubic
    CubicIn,
    CubicOut,
    CubicInOut,
    // Quart
    QuartIn,
    QuartOut,
    QuartInOut,
    // Quint
    QuintIn,
    QuintOut,
    QuintInOut,
    // Sine
    SineIn,
    SineOut,
    SineInOut,
    // Expo
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    // Circ
    CircIn,
    CircOut,
    CircInOut,
    // Back
    BackIn,
    BackOut,
    BackInOut,
    // Bounce
    BounceIn,
    BounceOut,
    BounceInOut,
    // Elastic
    ElasticIn,
    ElasticOut,
    ElasticInOut,
}

impl Easing {
    /// Applies the easing function for a given time `t`
    /// `t` is expected to be a value between 0.0 and 1.0.
    pub fn ease<F: Float>(self, t: F) -> F {
        match self {
            Easing::Linear => t,

            // Quad
            Easing::QuadIn => t * t,
            Easing::QuadOut => {
                let one = F::one();
                one - (one - t) * (one - t)
            }
            Easing::QuadInOut => {
                let half = F::from(0.5).unwrap();
                let one = F::one();
                let two = F::from(2.0).unwrap();
                if t < half {
                    two * t * t
                } else {
                    let t_rem = one - t;
                    one - two * two * t_rem * t_rem
                }
            }

            // Cubic
            Easing::CubicIn => t * t * t,
            Easing::CubicOut => {
                let one = F::one();
                let t1 = one - t;
                one - t1 * t1 * t1
            }
            Easing::CubicInOut => {
                let half = F::from(0.5).unwrap();
                let one = F::one();
                let four = F::from(4.0).unwrap();
                if t < half {
                    four * t * t * t
                } else {
                    let t_rem = one - t;
                    one - four * t_rem * t_rem * t_rem
                }
            }

            // Quart
            Easing::QuartIn => t * t * t * t,
            Easing::QuartOut => {
                let one = F::one();
                let t1 = one - t;
                one - t1 * t1 * t1 * t1
            }
            Easing::QuartInOut => {
                let half = F::from(0.5).unwrap();
                let one = F::one();
                let eight = F::from(8.0).unwrap();
                if t < half {
                    eight * t * t * t * t
                } else {
                    let t_rem = one - t;
                    one - eight * t_rem * t_rem * t_rem * t_rem
                }
            }

            // Quint
            Easing::QuintIn => t * t * t * t * t,
            Easing::QuintOut => {
                let one = F::one();
                let t1 = one - t;
                one - t1 * t1 * t1 * t1 * t1
            }
            Easing::QuintInOut => {
                let half = F::from(0.5).unwrap();
                let one = F::one();
                let sixteen = F::from(16.0).unwrap();
                if t < half {
                    sixteen * t * t * t * t * t
                } else {
                    let t_rem = one - t;
                    one - sixteen * t_rem * t_rem * t_rem * t_rem * t_rem
                }
            }

            // Sine
            Easing::SineIn => {
                let one = F::one();
                let pi_2 = F::from(PI / 2.0).unwrap();
                one - (t * pi_2).cos()
            }
            Easing::SineOut => {
                let pi_2 = F::from(PI / 2.0).unwrap();
                (t * pi_2).sin()
            }
            Easing::SineInOut => {
                let pi = F::from(PI).unwrap();
                -((pi * t).cos() - F::one()) / F::from(2.0).unwrap()
            }

            // Expo
            Easing::ExpoIn => {
                if t == F::zero() {
                    F::zero()
                } else {
                    F::from(2.0)
                        .unwrap()
                        .powf(F::from(10.0).unwrap() * t - F::from(10.0).unwrap())
                }
            }
            Easing::ExpoOut => {
                if t == F::one() {
                    F::one()
                } else {
                    F::one() - F::from(2.0).unwrap().powf(-F::from(10.0).unwrap() * t)
                }
            }
            Easing::ExpoInOut => {
                let half = F::from(0.5).unwrap();
                let ten = F::from(10.0).unwrap();
                let twenty = F::from(20.0).unwrap();
                let two = F::from(2.0).unwrap();
                if t == F::zero() {
                    F::zero()
                } else if t == F::one() {
                    F::one()
                } else if t < half {
                    two.powf(twenty * t - ten) / two
                } else {
                    (two - two.powf(-twenty * t + ten)) / two
                }
            }

            // Circ
            Easing::CircIn => {
                let one = F::one();
                one - (one - t.powi(2)).sqrt()
            }
            Easing::CircOut => {
                let one = F::one();
                (one - (t - one).powi(2)).sqrt()
            }
            Easing::CircInOut => {
                let half = F::from(0.5).unwrap();
                let one = F::one();
                let two = F::from(2.0).unwrap();
                if t < half {
                    (one - (one - (two * t).powi(2)).sqrt()) / two
                } else {
                    ((one - (-(two * t) + two).powi(2)).sqrt() + one) / two
                }
            }

            // Back
            Easing::BackIn => {
                let c1 = F::from(1.70158).unwrap();
                let c3 = c1 + F::one();
                c3 * t * t * t - c1 * t * t
            }
            Easing::BackOut => {
                let c1 = F::from(1.70158).unwrap();
                let c3 = c1 + F::one();
                let one = F::one();
                let t1 = t - one;
                one + c3 * t1.powi(3) + c1 * t1.powi(2)
            }
            Easing::BackInOut => {
                let c1 = F::from(1.70158).unwrap();
                let c2 = c1 * F::from(1.525).unwrap();
                let half = F::from(0.5).unwrap();
                let two = F::from(2.0).unwrap();
                if t < half {
                    ((two * t).powi(2) * ((c2 + F::one()) * two * t - c2)) / two
                } else {
                    (((two * t - two).powi(2) * ((c2 + F::one()) * (t * two - two) + c2)) + two)
                        / two
                }
            }

            // Bounce
            Easing::BounceIn => F::one() - Easing::BounceOut.ease(F::one() - t),
            Easing::BounceOut => {
                let n1 = F::from(7.5625).unwrap();
                let d1 = F::from(2.75).unwrap();

                if t < F::one() / d1 {
                    n1 * t * t
                } else if t < F::from(2.0).unwrap() / d1 {
                    let mut t2 = t;
                    t2 = t2 - (F::from(1.5).unwrap() / d1);
                    n1 * t2 * t2 + F::from(0.75).unwrap()
                } else if t < F::from(2.5).unwrap() / d1 {
                    let mut t2 = t;
                    t2 = t2 - (F::from(2.25).unwrap() / d1);
                    n1 * t2 * t2 + F::from(0.9375).unwrap()
                } else {
                    let mut t2 = t;
                    t2 = t2 - (F::from(2.625).unwrap() / d1);
                    n1 * t2 * t2 + F::from(0.984375).unwrap()
                }
            }
            Easing::BounceInOut => {
                let half = F::from(0.5).unwrap();
                let one = F::one();
                let two = F::from(2.0).unwrap();
                if t < half {
                    (one - Easing::BounceOut.ease(one - two * t)) / two
                } else {
                    (one + Easing::BounceOut.ease(two * t - one)) / two
                }
            }

            // Elastic
            Easing::ElasticIn => {
                let c4 = (F::from(2.0).unwrap() * F::from(PI).unwrap()) / F::from(3.0).unwrap();
                if t == F::zero() {
                    F::zero()
                } else if t == F::one() {
                    F::one()
                } else {
                    -F::from(2.0)
                        .unwrap()
                        .powf(F::from(10.0).unwrap() * t - F::from(10.0).unwrap())
                        * ((t * F::from(10.0).unwrap() - F::from(10.75).unwrap()) * c4).sin()
                }
            }
            Easing::ElasticOut => {
                let c4 = (F::from(2.0).unwrap() * F::from(PI).unwrap()) / F::from(3.0).unwrap();
                if t == F::zero() {
                    F::zero()
                } else if t == F::one() {
                    F::one()
                } else {
                    F::from(2.0).unwrap().powf(-F::from(10.0).unwrap() * t)
                        * ((t * F::from(10.0).unwrap() - F::from(0.75).unwrap()) * c4).sin()
                        + F::one()
                }
            }
            Easing::ElasticInOut => {
                let c5 = (F::from(2.0).unwrap() * F::from(PI).unwrap()) / F::from(4.5).unwrap();
                let half = F::from(0.5).unwrap();
                let two = F::from(2.0).unwrap();
                let ten = F::from(10.0).unwrap();
                let twenty = F::from(20.0).unwrap();
                if t == F::zero() {
                    F::zero()
                } else if t == F::one() {
                    F::one()
                } else if t < half {
                    -(two.powf(twenty * t - ten)
                        * ((twenty * t - F::from(11.125).unwrap()) * c5).sin())
                        / two
                } else {
                    (two.powf(-twenty * t + ten)
                        * ((twenty * t - F::from(11.125).unwrap()) * c5).sin())
                        / two
                        + F::one()
                }
            }
        }
    }
}

/// Adds an easing extension method to `Float` types.
pub trait EaseExt: Float {
    /// Applies the specified easing function to the float value.
    /// `self` is expected to be a value between 0.0 and 1.0.
    fn ease(self, easing_type: Easing) -> Self;
}

impl<F: Float> EaseExt for F {
    fn ease(self, easing_type: Easing) -> F {
        easing_type.ease(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_bounds() {
        let t_values = [0.0, 0.25, 0.5, 0.75, 1.0];
        let easings = [
            Easing::Linear,
            Easing::QuadIn,
            Easing::QuadOut,
            Easing::QuadInOut,
            Easing::CubicIn,
            Easing::CubicOut,
            Easing::CubicInOut,
            Easing::QuartIn,
            Easing::QuartOut,
            Easing::QuartInOut,
            Easing::QuintIn,
            Easing::QuintOut,
            Easing::QuintInOut,
            Easing::SineIn,
            Easing::SineOut,
            Easing::SineInOut,
            Easing::ExpoIn,
            Easing::ExpoOut,
            Easing::ExpoInOut,
            Easing::CircIn,
            Easing::CircOut,
            Easing::CircInOut,
            Easing::BackIn,
            Easing::BackOut,
            Easing::BackInOut,
            Easing::BounceIn,
            Easing::BounceOut,
            Easing::BounceInOut,
            Easing::ElasticIn,
            Easing::ElasticOut,
            Easing::ElasticInOut,
        ];

        for easing in easings.iter() {
            for &t in t_values.iter() {
                let result = easing.ease(t as f32);
                assert!(
                    result.is_finite(),
                    "Easing {:?} at t={} should be finite, but was {}",
                    easing,
                    t,
                    result
                );

                // Check that t=0 gives 0 and t=1 gives 1 for most easing functions
                if t == 0.0 {
                    assert!(
                        (result - 0.0).abs() < 1e-6,
                        "Easing {:?} should start at 0, but was {}",
                        easing,
                        result
                    );
                }
                if t == 1.0 {
                    assert!(
                        (result - 1.0).abs() < 1e-6,
                        "Easing {:?} should end at 1, but was {}",
                        easing,
                        result
                    );
                }
            }
        }
    }
}
