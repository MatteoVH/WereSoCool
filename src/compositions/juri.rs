use event::{Event, Render};
use instrument::{oscillator::Oscillator, stereo_waveform::StereoWaveform};
use operations::{Apply, Op, Op::*};
use settings::get_default_app_settings;

fn composition() -> Op {
    fn overtones() -> Op {
        r![
            (16, 1, 11.0, 1.0, 0.0),
            (15, 1, 0.0, 1.0, 0.0),
            (14, 3, 6.0, 1.0, 0.0),
            (13, 3, 0.0, 1.0, 0.0),
            (2, 1, 6.0, 1.0, 0.0),
            (2, 1, 0.0, 1.0, 0.0),
            (2, 1, 12.0, 1.0, 0.0),
            (3, 2, 0.0, 1.0, 0.0),
            (3, 2, 5.0, 1.0, 0.0),
            (5, 4, 0.0, 1.0, 0.0),
            (5, 4, 2.0, 1.0, 0.0),
            (1, 1, 3.0, 1.0, 0.0),
            (1, 1, 0.0, 1.0, 0.0),
        ]
    }

    fn sequence1() -> Op {
        sequence![
            r![
                (20, 3, 7.0, 1.0, 0.0),
                (20, 3, 0.0, 1.0, 0.0),
                (10, 3, 7.0, 1.0, 0.0),
                (10, 3, 0.0, 1.0, 0.0),
                (9, 4, 0.0, 1.0, 0.0),
                (9, 4, 0.0, 1.0, 0.0),
                (3, 2, 5.0, 1.0, 0.0),
                (3, 2, 0.0, 1.0, 0.0),
                (3, 2, 2.0, 1.0, 0.0),
                (1, 1, 0.0, 1.0, 0.0),
                (1, 1, 2.0, 1.0, 0.0),
                (1, 2, 1.0, 1.0, 0.0),
                (1, 2, 0.0, 1.0, 0.0),
            ],
            r![
                (15, 2, 11.0, 1.0, 0.0),
                (15, 2, 0.0, 1.0, 0.0),
                (10, 3, 11.0, 1.0, 0.0),
                (10, 3, 0.0, 1.0, 0.0),
                (9, 4, 0.0, 1.0, 0.0),
                (9, 4, 0.0, 1.0, 0.0),
                (3, 2, 7.0, 1.0, 0.0),
                (3, 2, 0.0, 1.0, 0.0),
                (3, 2, 2.0, 1.0, 0.0),
                (1, 1, 0.0, 1.0, 0.0),
                (1, 1, 2.0, 1.0, 0.0),
                (1, 2, 1.0, 1.0, 0.0),
                (1, 2, 0.0, 1.0, 0.0),
            ],
            r![
                (9, 1, 15.0, 1.0, 0.0),
                (9, 1, 0.0, 1.0, 0.0),
                (10, 3, 7.0, 1.0, 0.0),
                (10, 3, 0.0, 1.0, 0.0),
                (9, 4, 0.0, 1.0, 0.0),
                (9, 4, 0.0, 1.0, 0.0),
                (3, 2, 10.0, 1.0, 0.0),
                (3, 2, 0.0, 1.0, 0.0),
                (3, 2, 2.0, 1.0, 0.0),
                (1, 1, 0.0, 1.0, 0.0),
                (1, 1, 2.0, 1.0, 0.0),
                (1, 2, 1.0, 1.0, 0.0),
                (1, 2, 0.0, 1.0, 0.0),
            ],
            r![
                (10, 1, 15.0, 1.0, 0.0),
                (10, 1, 0.0, 1.0, 0.0),
                (11, 3, 7.0, 1.0, 0.0),
                (11, 3, 0.0, 1.0, 0.0),
                (9, 4, 3.0, 1.0, 0.0),
                (9, 4, 0.0, 1.0, 0.0),
                (9, 4, 8.0, 1.0, 0.0),
                (3, 2, 0.0, 1.0, 0.0),
                (3, 2, 2.0, 1.0, 0.0),
                (1, 1, 0.0, 1.0, 0.0),
                (1, 1, 2.0, 1.0, 0.0),
                (1, 2, 1.0, 1.0, 0.0),
                (1, 2, 0.0, 1.0, 0.0),
            ],
            r![
                (12, 1, 15.0, 1.0, 0.0),
                (12, 1, 0.0, 1.0, 0.0),
                (12, 3, 7.0, 1.0, 0.0),
                (12, 3, 0.0, 1.0, 0.0),
                (5, 2, 12.0, 1.0, 0.0),
                (5, 2, 0.0, 1.0, 0.0),
                (5, 2, 6.0, 1.0, 0.0),
                (3, 2, 0.0, 1.0, 0.0),
                (3, 2, 2.0, 1.0, 0.0),
                (1, 1, 0.0, 1.0, 0.0),
                (1, 1, 2.0, 1.0, 0.0),
                (7, 16, 1.0, 1.0, 0.0),
                (7, 16, 0.0, 1.0, 0.0),
            ],
            r![
                (13, 1, 15.0, 1.0, 0.0),
                (13, 1, 0.0, 1.0, 0.0),
                (11, 3, 7.0, 1.0, 0.0),
                (11, 3, 0.0, 1.0, 0.0),
                (5, 3, 0.0, 1.0, 0.0),
                (5, 3, 0.0, 1.0, 0.0),
                (3, 2, 6.0, 1.0, 0.0),
                (3, 2, 0.0, 1.0, 0.0),
                (3, 2, 2.0, 1.0, 0.0),
                (9, 8, 0.0, 1.0, 0.0),
                (9, 8, 2.0, 1.0, 0.0),
                (7, 16, 1.0, 1.0, 0.0),
                (7, 16, 0.0, 1.0, 0.0),
            ],
            r![
                (14, 1, 15.0, 1.0, 0.0),
                (13, 1, 0.0, 1.0, 0.0),
                (12, 3, 7.0, 1.0, 0.0),
                (11, 3, 0.0, 1.0, 0.0),
                (5, 3, 0.0, 1.0, 0.0),
                (5, 3, 0.0, 1.0, 0.0),
                (2, 1, 6.0, 1.0, 0.0),
                (2, 1, 0.0, 1.0, 0.0),
                (3, 2, 2.0, 1.0, 0.0),
                (6, 5, 0.0, 1.0, 0.0),
                (6, 5, 2.0, 1.0, 0.0),
                (2, 5, 1.0, 1.0, 0.0),
                (2, 5, 0.0, 1.0, 0.0),
            ],
            r![
                (15, 1, 15.0, 1.0, 0.0),
                (14, 1, 0.0, 1.0, 0.0),
                (13, 3, 7.0, 1.0, 0.0),
                (12, 3, 0.0, 1.0, 0.0),
                (15, 8, 0.0, 1.0, 0.0),
                (15, 8, 0.0, 1.0, 0.0),
                (3, 2, 9.0, 1.0, 0.0),
                (3, 2, 0.0, 1.0, 0.0),
                (3, 2, 2.0, 1.0, 0.0),
                (5, 4, 0.0, 1.0, 0.0),
                (5, 4, 2.0, 1.0, 0.0),
                (2, 5, 4.0, 1.0, 0.0),
                (2, 5, 0.0, 1.0, 0.0),
            ],
            r![
                (16, 1, 11.0, 1.0, 0.0),
                (15, 1, 0.0, 1.0, 0.0),
                (14, 3, 6.0, 1.0, 0.0),
                (13, 3, 0.0, 1.0, 0.0),
                (2, 1, 6.0, 1.0, 0.0),
                (2, 1, 0.0, 1.0, 0.0),
                (3, 2, 15.0, 1.0, 0.0),
                (3, 2, 0.0, 1.0, 0.0),
                (3, 2, 5.0, 1.0, 0.0),
                (5, 4, 0.0, 1.0, 0.0),
                (5, 4, 2.0, 1.0, 0.0),
                (1, 1, 3.0, 1.0, 0.0),
                (1, 1, 0.0, 1.0, 0.0),
            ],
        ]
    }

    fn sequence2() -> Op {
        sequence![
            TransposeM {m: 9.0/8.0},
            TransposeM {m: 15.0/16.0},
            TransposeM {m: 5.0/4.0},
            TransposeM {m: 4.0/3.0},
            TransposeM {m: 3.0/2.0},
            TransposeM {m: 8.0/5.0},
            TransposeM {m: 7.0/4.0},
            compose![
                TransposeM {m: 15.0/8.0},
                Length {m: 1.75}
            ],
            compose![
                TransposeM {m: 1.0/2.0},
                Length {m: 5.0}
            ],
            Silence {m: 2.0},
            Silence {m: 1.0}
        ]
    }

    fn sequence3() -> Op {
        sequence![
            r![
                (15, 8, 0.0, 1.0, 0.0),
                (15, 8, 0.0, 1.0, 0.0),
                (9, 8, 0.0, 1.0, 0.0),
                (9, 8, 0.0, 1.0, 0.0),
                (9, 16, 0.0, 1.0, 0.0),
                (9, 16, 0.0, 1.0, 0.0),
            ],
            r![
                (2, 1, 0.0, 1.0, 0.0),
                (2, 1, 0.0, 1.0, 0.0),
                (5, 4, 0.0, 1.0, 0.0),
                (5, 4, 0.0, 1.0, 0.0),
                (3, 4, 0.0, 1.0, 0.0),
                (3, 4, 0.0, 1.0, 0.0),
            ],
            r![
                (5, 2, 9.0, 1.0, 0.0),
                (5, 2, 0.0, 1.0, 0.0),
                (1, 1, 0.0, 1.0, 0.0),
                (1, 1, 0.0, 1.0, 0.0),
                (2, 3, 0.0, 1.0, 0.0),
                (2, 3, 0.0, 1.0, 0.0),
            ],
            r![
                (9, 4, 0.0, 1.0, 0.0),
                (9, 4, 0.0, 1.0, 0.0),
                (4, 3, 0.0, 1.0, 0.0),
                (4, 3, 0.0, 1.0, 0.0),
                (3, 4, 1.0, 1.0, 0.0),
                (3, 4, 0.0, 1.0, 0.0),
            ],
            r![
                (2, 1, 0.0, 1.0, 0.0),
                (2, 1, 0.0, 1.0, 0.0),
                (5, 4, 0.0, 1.0, 0.0),
                (5, 4, 0.0, 1.0, 0.0),
                (3, 4, 0.0, 1.0, 0.0),
                (3, 4, 0.0, 1.0, 0.0),
            ],
            Silence {m: 0.1},
            r![
                (28, 6, 0.0, 1.0, 0.0),
                (9, 4, 11.0, 1.0, 0.0),
                (9, 4, 0.0, 1.0, 0.0),
                (4, 3, 4.0, 1.0, 0.0),
                (4, 3, 0.0, 1.0, 0.0),
                (3, 4, 0.0, 1.0, 0.0),
                (3, 4, 0.0, 1.0, 0.0),
            ],
            Silence {m: 0.1},
            r![
                (4, 1, 0.0, 1.0, 0.0),
                (3, 1, 0.0, 1.0, 0.0),
                (5, 2, 0.0, 1.0, 0.0),
                (2, 1, 0.0, 1.0, 0.0),
                (2, 1, 0.0, 1.0, 0.0),
                (5, 4, 0.0, 1.0, 0.0),
                (5, 4, 0.0, 1.0, 0.0),
                (1, 1, 0.0, 1.0, 0.0),
                (3, 4, 0.0, 1.0, 0.0),
                (3, 4, 0.0, 1.0, 0.0),
                (2, 1, 0.0, 1.0, 0.0),
            ],
        ]
    }

    fn section1() -> Op {
        sequence![
            sequence1(),
            compose![
                overtones(),
                sequence2(),
                Length {m: 0.66},
            ],
//        Chords
            compose![
                sequence![
                    compose![
                        AsIs,
                        Length {m: 1.5}
                    ],
                    Silence {m: 3.0},
                    Silence {m: 2.0}
                ],
                sequence3(),
                Gain {m: 2.0},
            ]
        ]
    }

    fn fasts() -> Op {
        fit![
            sequence2() => sequence1(), 8
        ]
    }

    fn fasts2() -> Op {
        fit![
            compose![
                r![
                     (2, 1, 0.0, 1.0, 0.0),
                     (2, 1, 0.0, 1.0, 0.0),
                     (3, 2, 3.0, 1.0, 0.0),
                     (1, 1, 0.0, 1.0, 0.0),
                     (1, 1, 0.0, 1.0, 0.0),
                ],
                sequence2(),
                TransposeM {m: 3.0},

            ] => compose![
                    fasts(),
                    Length {m: 0.45},
                ], 6
        ]
    }

    fn fasts3() -> Op {
        fit![
            compose![
                r![
                     (11, 8, 9.0, 1.0, 0.0),
                     (11, 8, 0.0, 1.0, 0.0),
                     (1, 1, 3.0, 1.0, 0.0),
                     (1, 1, 0.0, 1.0, 0.0),
                ],
                fasts2(),
                TransposeM {m: 8.0/5.0},
                Gain {m: 0.25},
                Reverse,

            ] => compose![
                    fasts2(),
                ], 2
        ]
    }

    fn improvise() -> Op {
        compose![
            sequence![
                compose![
                    r![
                         (2, 1, 0.0, 1.0, 0.0),
                         (1, 1, 0.0, 1.0, 0.0),
                    ],
                    fasts(),
                    Gain {m: 1.2}
                ],
                compose![
                    r![
                         (2, 1, 0.0, 1.0, 0.0),
                         (2, 1, 0.0, 1.0, 0.0),
                         (3, 2, 3.0, 1.0, 0.0),
                         (1, 1, 0.0, 1.0, 0.0),
                         (1, 1, 0.0, 1.0, 0.0),
                    ],
                    fasts(),
                ],
                compose![
                    r![
                         (11, 4, 7.0, 0.75, 0.0),
                         (11, 4, 0.0, 0.75, 0.0),
                         (2, 1, 0.0, 1.0, 0.0),
                         (2, 1, 0.0, 1.0, 0.0),
                         (3, 2, 3.0, 1.0, 0.0),
                         (1, 1, 0.0, 1.0, 0.0),
                         (1, 1, 0.0, 1.0, 0.0),
                    ],
                    fasts(),
                ],
                compose![
                    r![
                         (17, 4, 0.0, 0.75, 0.0),
                         (16, 4, 0.0, 0.75, 0.0),
                         (15, 4, 0.0, 0.75, 0.0),
                         (11, 4, 7.0, 0.75, 0.0),
                         (11, 4, 0.0, 0.75, 0.0),
                         (2, 1, 0.0, 1.0, 0.0),
                         (2, 1, 0.0, 1.0, 0.0),
                         (3, 2, 3.0, 1.0, 0.0),
                         (1, 1, 0.0, 1.0, 0.0),
                         (1, 1, 0.0, 1.0, 0.0),
                    ],
                    fasts(),
                ],
            ],
            Length {m: 0.45}
        ]
    }

    fn improvise2() -> Op {
        overlay![
            compose![
                sequence![

                        compose![
                            r![
                                 (19, 4, 0.0, 0.75, 0.0),
                                 (17, 4, 0.0, 0.75, 0.0),
                                 (16, 4, 0.0, 0.75, 0.0),
                                 (15, 4, 0.0, 0.75, 0.0),
                                 (11, 4, 7.0, 0.75, 0.0),
                                 (11, 4, 0.0, 0.75, 0.0),
                                 (2, 1, 0.0, 1.0, 0.0),
                                 (2, 1, 0.0, 1.0, 0.0),
                                 (3, 2, 3.0, 1.0, 0.0),
                                 (1, 1, 0.0, 1.0, 0.0),
                                 (1, 1, 0.0, 1.0, 0.0),
                            ],
                            fasts(),
                        ],

                ],
                Length {m: 0.45}
            ],
            compose![
                sequence![
                    AsIs,
                    TransposeM {m: 15.0/16.0}
                ],
                fasts3(),
                Length {m: 0.45}
            ]
        ]
    }

    fn improvise3() -> Op {
        overlay![
            compose![
                sequence![
                    compose![
                        r![
                             (19, 4, 0.0, 0.75, 0.0),
                             (17, 4, 0.0, 0.75, 0.0),
                             (16, 4, 0.0, 0.75, 0.0),
                             (15, 4, 0.0, 0.75, 0.0),
                             (11, 4, 7.0, 0.75, 0.0),
                             (11, 4, 0.0, 0.75, 0.0),
                             (2, 1, 0.0, 1.0, 0.0),
                             (2, 1, 0.0, 1.0, 0.0),
                             (3, 2, 3.0, 1.0, 0.0),
                             (1, 1, 0.0, 1.0, 0.0),
                             (1, 1, 0.0, 1.0, 0.0),
                        ],
                        fasts(),
                    ],
                ],
                Length {m: 0.45}
            ],
            fasts2(),
            compose![
                sequence![
                    AsIs,
                    TransposeM {m: 15.0/16.0}
                ],
                fasts3(),
                Length {m: 0.55}
            ]
        ]
    }

    fn final_fasts() -> Op {
        sequence![
            compose![
                r![
                     (27, 4, 0.0, 0.75, 0.0),
                     (27, 4, 9.0, 0.75, 0.0),
                     (24, 4, 0.0, 0.75, 0.0),
                     (21, 4, 0.0, 0.75, 0.0),
                     (19, 4, 0.0, 0.75, 0.0),
                     (17, 4, 0.0, 0.75, 0.0),
                     (16, 4, 0.0, 0.75, 0.0),
                     (15, 4, 0.0, 0.75, 0.0),
                     (11, 4, 7.0, 0.75, 0.0),
                     (11, 4, 0.0, 0.75, 0.0),
                     (2, 1, 0.0, 1.0, 0.0),
                     (2, 1, 0.0, 1.0, 0.0),
                     (3, 2, 3.0, 1.0, 0.0),
                     (1, 1, 0.0, 1.0, 0.0),
                     (1, 1, 0.0, 1.0, 0.0),
                ],
                fasts(),
                Gain {m: 1.10},
                Length {m: 0.40}
            ],
        ]
    }

    fn improvise4() -> Op {
        sequence![
            final_fasts(),
            compose![
                final_fasts(),
                Length {m: 0.85}
            ],
            compose![
                r![
                    (5, 2, 0.0, 0.7, 0.0),
                    (1, 1, 0.0, 0.7, 0.0),
                ],
                final_fasts(),
                TransposeM {m: 6.0/5.0},
                Length {m: 0.7}
            ]
        ]
    }

    fn chords() -> Op {
        sequence![
            compose![
                overtones(),
                Gain {m: 2.0},
                Length {m: 8.0}
            ],
            Silence {m: 3.0},
            compose![
                sequence3(),
                Length {m: 4.0},
                Gain {m: 1.75},
            ],
            sequence1(),
            compose![
                overtones(),
                sequence2(),
                Length {m: 0.66},
            ],
        ]
    }

    sequence![
        section1(),
        improvise(),
        improvise2(),
        improvise3(),
        improvise4(),
        chords(),
        Silence {m: 3.0},
    ]
}

fn oscillator() -> Oscillator {
    Oscillator::init(&get_default_app_settings())
}

fn event() -> Event {
    Event::init(155.563, 0.2, 0.0, 4.0)
}

fn generate_events(event: Event, operation: fn() -> Op) -> Vec<Event> {
    operation().apply(vec![event])
}

pub fn operations() -> Op {
    composition()
}

pub fn events() -> Vec<Event> {
    generate_events(event(), composition)
}

pub fn generate_composition() -> StereoWaveform {
    events().render(&mut oscillator())
}
