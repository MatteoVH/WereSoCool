use event::{Event, Render};
use instrument::{Oscillator, StereoWaveform};
use operations::{Apply, Op};
use settings::get_default_app_settings;

pub fn generate_composition() -> StereoWaveform {
    fn sequence1() -> Op {
        Op::Sequence {
            operations: vec![
                r![
                    (12, 1, 7.0, 0.02, -1.0),
                    (12, 1, 7.0, 0.02, 1.0),
                    (5, 1, 7.0, 0.14, 1.0),
                    (5, 1, 0.0, 0.14, 1.0),
                    (5, 2, 0.0, 0.2, -0.5),
                    (5, 2, 7.0, 0.2, 0.5),
                    (3, 1, 7.0, 0.2, -1.0),
                    (3, 1, 0.0, 0.2, 1.0),
                    (2, 1, 5.0, 0.1, -0.5),
                    (2, 1, 0.0, 0.1, -0.5),
                    (1, 1, 0.0, 1.0, 0.0),
                    (1, 1, 0.0, 1.0, 0.0),
                ],
                r![
                    (10, 1, 2.0, 0.02, 1.0),
                    (10, 1, 0.0, 0.02, -1.0),
                    (6, 1, 2.0, 0.14, 1.0),
                    (6, 1, 0.0, 0.14, -1.0),
                    (12, 5, 0.0, 0.2, 0.5),
                    (12, 5, 2.0, 0.2, 0.5),
                    (3, 1, 2.0, 0.2, -1.0),
                    (3, 1, 0.0, 0.2, -1.0),
                    (15, 8, 5.0, 0.1, 0.5),
                    (15, 8, 0.0, 0.1, 0.5),
                    (9, 8, 0.0, 0.5, 0.0),
                    (9, 8, 0.0, 0.5, 0.0),
                ],
                r![
                    (13, 1, 0.0, 0.02, -1.0),
                    (13, 1, 9.0, 0.02, 1.0),
                    (6, 1, 0.0, 0.14, 1.0),
                    (6, 1, 8.0, 0.14, 1.0),
                    (9, 4, 0.0, 0.2, -0.5),
                    (9, 4, 1.0, 0.2, 0.5),
                    (3, 1, 1.0, 0.2, -1.0),
                    (3, 1, 0.0, 0.2, 1.0),
                    (15, 8, 2.0, 0.1, -0.5),
                    (15, 8, 0.0, 0.1, -0.5),
                    (3, 4, 3.0, 1.0, 0.0),
                    (3, 4, 0.0, 1.0, 0.0),
                ],
                Op::Compose {
                    operations: vec![
                        r![
                            (8, 1, 0.0, 0.02, 1.0),
                            (8, 1, 5.0, 0.02, -1.0),
                            (5, 1, 0.0, 0.14, -1.0),
                            (5, 1, 4.0, 0.14, 1.0),
                            (10, 4, 0.0, 0.2, 0.5),
                            (10, 4, 1.0, 0.2, -0.5),
                            (3, 1, 1.0, 0.2, 1.0),
                            (3, 1, 3.0, 0.2, 1.0),
                            (3, 2, 0.0, 0.1, 0.5),
                            (3, 2, 0.0, 0.1, -0.5),
                            (1, 1, 2.0, 1.0, 0.0),
                            (1, 1, 0.0, 1.0, 0.0),
                            (1, 2, 0.0, 1.0, 0.0),
                            (1, 2, 1.0, 1.0, 0.0),
                        ],
                        Op::Gain { m: 0.4 },
                    ],
                },
                r![
                    (11, 1, 0.0, 0.02, 1.0),
                    (11, 1, 2.0, 0.02, -1.0),
                    (10, 1, 0.0, 0.14, 1.0),
                    (10, 1, 0.0, 0.14, -1.0),
                    (8, 1, 0.0, 0.14, 1.0),
                    (8, 1, 5.0, 0.14, -1.0),
                    (9, 2, 0.0, 0.2, 0.5),
                    (9, 2, 4.0, 0.2, -0.5),
                    (5, 3, 1.0, 0.2, 1.0),
                    (5, 3, 0.0, 0.2, -1.0),
                    (4, 3, 3.0, 0.1, 0.5),
                    (4, 3, 0.0, 0.1, 0.5),
                    (7, 8, 4.0, 1.0, 0.0),
                    (7, 8, 0.0, 1.0, 0.0),
                ],
                r![
                    (12, 1, 0.0, 0.02, 1.0),
                    (12, 1, 2.0, 0.02, -1.0),
                    (10, 1, 0.0, 0.14, -1.0),
                    (10, 1, 0.0, 0.14, 1.0),
                    (8, 1, 0.0, 0.14, -1.0),
                    (8, 1, 5.0, 0.14, 1.0),
                    (9, 2, 0.0, 0.2, 0.5),
                    (9, 2, 4.0, 0.2, -0.5),
                    (3, 1, 1.0, 0.2, -1.0),
                    (3, 1, 0.0, 0.2, 1.0),
                    (2, 1, 3.0, 0.1, 0.5),
                    (2, 1, 0.0, 0.1, 0.5),
                    (5, 3, 4.0, 1.0, 0.0),
                    (5, 4, 0.0, 1.0, 0.0),
                ],
                r![
                    (13, 1, 0.0, 0.02, -1.0),
                    (13, 1, 9.0, 0.02, 1.0),
                    (6, 1, 0.0, 0.14, -1.0),
                    (6, 1, 8.0, 0.14, 1.0),
                    (9, 4, 0.0, 0.2, 0.5),
                    (9, 4, 1.0, 0.2, -0.5),
                    (9, 4, 1.0, 0.2, 1.0),
                    (3, 1, 0.0, 0.2, -1.0),
                    (15, 8, 2.0, 0.1, 0.5),
                    (15, 8, 0.0, 0.1, -0.5),
                    (3, 4, 3.0, 1.0, -0.3),
                    (3, 4, 0.0, 1.0, 0.3),
                ],
                Op::Compose {
                    operations: vec![
                        r![
                            (12, 1, 13.0, 0.02, -1.0),
                            (10, 1, 0.0, 0.02, 1.0),
                            (6, 1, 0.0, 0.14, 1.0),
                            (6, 1, 6.0, 0.14, 1.0),
                            (10, 4, 0.0, 0.2, -0.5),
                            (10, 4, 1.0, 0.2, 0.5),
                            (4, 1, 1.0, 0.2, -1.0),
                            (4, 1, 5.0, 0.2, 1.0),
                            (3, 2, 0.0, 0.1, -0.5),
                            (3, 2, 0.0, 0.1, -0.5),
                            (1, 1, 4.0, 1.0, 0.0),
                            (1, 1, 0.0, 1.0, 0.0),
                            (1, 2, 0.0, 1.0, 0.0),
                            (1, 2, 2.0, 1.0, 0.0),
                            (3, 4, 1.0, 1.0, 0.0),
                            (3, 4, 0.0, 1.0, 0.0),
                            (1, 4, 2.0, 1.0, 0.0),
                            (1, 4, 0.0, 1.0, 0.0),
                        ],
                        Op::Gain { m: 0.2 },
                    ],
                },
                Op::Compose {
                    operations: vec![
                        Op::Sequence {
                            operations: vec![
                                r![
                                    (10, 1, 0.0, 0.02, -1.0),
                                    (10, 1, 0.0, 0.02, -1.0),
                                    (15, 2, 9.0, 0.02, 1.0),
                                    (15, 2, 0.0, 0.14, -1.0),
                                    (9, 2, 8.0, 0.14, 1.0),
                                    (9, 2, 0.0, 0.2, 0.5),
                                    (5, 2, 1.0, 0.2, -0.5),
                                    (5, 2, 1.0, 0.2, 1.0),
                                    (3, 1, 0.0, 0.2, -1.0),
                                    (3, 1, 0.0, 0.2, -1.0),
                                    (2, 1, 2.0, 0.1, 0.5),
                                    (2, 1, 0.0, 0.1, -0.5),
                                    (5, 4, 3.0, 1.0, -0.5),
                                    (5, 4, 0.0, 1.0, 0.5),
                                ],
                                r![
                                    (9, 1, 0.0, 0.02, -1.0),
                                    (9, 1, 0.0, 0.02, -1.0),
                                    (13, 2, 4.0, 0.02, 1.0),
                                    (13, 2, 0.0, 0.14, -1.0),
                                    (12, 5, 3.0, 0.14, 1.0),
                                    (12, 5, 0.0, 0.2, 0.5),
                                    (4, 3, 0.0, 0.2, -0.5),
                                    (4, 3, 1.0, 0.2, 1.0),
                                    (3, 1, 0.0, 0.2, -1.0),
                                    (3, 1, 0.0, 0.2, -1.0),
                                    (15, 8, 2.0, 0.1, 0.5),
                                    (15, 8, 0.0, 0.1, -0.5),
                                    (9, 8, 3.0, 0.6, -0.0),
                                    (9, 8, 0.0, 0.6, 0.0),
                                ],
                                r![
                                    (15, 8, 4.0, 0.4, -0.5),
                                    (15, 8, 4.0, 0.4, 0.5),
                                    (5, 3, 4.0, 0.7, 0.5),
                                    (5, 3, 0.0, 0.7, -0.5),
                                    (5, 6, 5.0, 0.7, -1.0),
                                    (5, 6, 0.0, 0.7, 1.0),
                                ],
                                Op::Compose {
                                    operations: vec![
                                        r![
                                            (5, 3, 4.0, 0.3, 0.5),
                                            (5, 3, 0.0, 0.3, 0.5),
                                            (3, 2, 4.0, 0.5, -0.5),
                                            (3, 2, 0.0, 0.5, -0.5),
                                            (15, 16, 5.0, 0.5, -1.0),
                                            (15, 16, 0.0, 0.5, 1.0),
                                        ],
                                        Op::Gain { m: 0.5 },
                                    ],
                                },
                                Op::Compose {
                                    operations: vec![
                                        r![
                                            (9, 8, 4.0, 0.1, -0.5),
                                            (9, 8, 0.0, 0.1, 0.5),
                                            (3, 2, 4.0, 0.5, 0.5),
                                            (3, 2, 0.0, 0.5, -0.5),
                                            (1, 1, 5.0, 0.5, 0.0),
                                            (1, 1, 0.0, 0.5, 0.0),
                                        ],
                                        Op::Length { m: 0.5 },
                                        Op::Gain { m: 0.4 },
                                    ],
                                },
                                Op::Compose {
                                    operations: vec![
                                        r![
                                            (9, 8, 7.0, 0.1, -0.5),
                                            (9, 8, 0.0, 0.1, 0.5),
                                            (3, 2, 6.0, 0.5, 0.5),
                                            (3, 2, 0.0, 0.5, -0.5),
                                            (1, 1, 5.0, 0.5, 0.0),
                                            (1, 1, 0.0, 0.5, 0.0),
                                        ],
                                        Op::Length { m: 0.5 },
                                        Op::Gain { m: 0.5 },
                                    ],
                                },
                            ],
                        },
                        Op::TransposeM { m: 4.0 / 3.0 },
                        Op::Length { m: 0.7 },
                    ],
                },
            ],
        }
    };

    let main = Op::Sequence {
        operations: vec![sequence1(), Op::Silence { m: 1.5 }],
    };

    let mut oscillator = Oscillator::init(&get_default_app_settings());
    let e = vec![Event::init(120.0, 1.0, 0.0, 5.0)];
    let mut events = main.apply(e);

    events.render(&mut oscillator)
}
