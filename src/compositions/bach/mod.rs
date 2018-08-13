mod phrases;
use event::{Event, Render};
use instrument::{oscillator::Oscillator, stereo_waveform::StereoWaveform};
use operations::{Op, Operate};
use ratios::{Pan, R};
use settings::get_default_app_settings;

pub fn generate_composition() -> StereoWaveform {
    let rs = r![
        (1, 1, 0.0, 0.2, 0.0),
        (1, 1, 0.0, 1.0, 0.0),
        (1, 1, 0.0, 1.0, 0.0),
        (1, 1, 0.0, 1.0, 0.0),
//
//        (1, 1, 0.0, 1.0, 0.0),
//        (1, 1, 0.0, 1.0, 0.0),
//        (1, 1, 0.0, 1.0, 0.0),
//
//        (1, 1, 0.0, 1.0, 0.0),
//        (1, 1, 0.0, 0.2, 0.0),
//        (1, 1, 0.0, 1.0, 0.0),
//        (1, 1, 0.0, 0.2, 0.0),
////
//        (1, 1, 0.0, 1.0, 0.0),
//        (1, 1, 0.0, 0.2, 0.0),
//        (1, 1, 0.0, 1.0, 0.0),
//        (1, 1, 0.0, 0.2, 0.0),
    ];

    let sequence1 = Op::Sequence {
        operations: vec![
            phrases::bach::m0_24(),
            Op::Compose { operations: vec![
                Op::Sequence { operations: vec![
                phrases::bach::m24_37(),
                phrases::bach::m34_37(),
            ]},
                Op::Gain { m: 1.6}
            ]}
        ],
    };

    let mut oscillator = Oscillator::init(rs.clone(), &get_default_app_settings());
    let e = vec![Event::new(290.0, rs.clone(), 0.14, 0.35)];
    let mut events = sequence1.apply(e);

    events.render(&mut oscillator)
}
