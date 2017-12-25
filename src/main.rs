extern crate time;
use time::Duration;

const DISCLAIMER: &str = "
IMPORTANT NOTE!: This is an UNOFFICIAL and UNSANCTIONED reupload of \
the TLA+ lecture videos with some chapter markers in the video \
description on Lamport's site for ease of viewing on mobile devices, or \
other platforms and connection setups where YouTube is the most optimal \
\"format\". For the original files and original resources, please visit \
http://lamport.azurewebsites.net/video/videos.html.
";

struct Video<'a> {
    name: &'a str,
    description: &'a str,
    original_link: &'a str,
    markers: Vec<Marker<'a>>,
}

struct Marker<'a> {
    name: &'a str,
    timestamp: time::Duration,
}

fn main() {
    let videos = vec![
        Video {
            name: "Introduction to TLA+",
            description: "Explains what TLA+ is and why you might want to use it.  \
                                        It introduces the concept of a state machine.",
            original_link: "http://lamport.azurewebsites.net/video/intro.html",
            markers: vec![
                Marker {
                    name: "Prologue",
                    timestamp: Duration::seconds(5),
                },
                Marker {
                    name: "What is TLA+?",
                    timestamp: Duration::seconds(46),
                },
                Marker {
                    name: "Abstraction",
                    timestamp: Duration::seconds(166),
                },
                Marker {
                    name: "What Engineers Say",
                    timestamp: Duration::seconds(308),
                },
                Marker {
                    name: "What Can TLA+ Check?",
                    timestamp: Duration::seconds(581),
                },
                Marker {
                    name: "The Basic Abstraction",
                    timestamp: Duration::seconds(637),
                },
                Marker {
                    name: "State Machines",
                    timestamp: Duration::seconds(770),
                },
                Marker {
                    name: "A Tiny Example",
                    timestamp: Duration::seconds(865),
                },
                Marker {
                    name: "Epilogue",
                    timestamp: Duration::seconds(1190),
                },
            ],
        },
        Video {
            name: "State Machines in TLA+",
            description: "Shows how a simple state machine is \
                           described in TLA+, giving the first glimpse of a TLA+ specification. ",
            original_link: "http://lamport.azurewebsites.net/video/smintla.html",
            markers: vec![
                Marker {
                    name: "Prologue",
                    timestamp: Duration::seconds(4),
                },
                Marker {
                    name: "What Language Should We Use?",
                    timestamp: Duration::seconds(26),
                },
                Marker {
                    name: "Describing a State Machine with Math",
                    timestamp: Duration::seconds(152),
                },
                Marker {
                    name: "A Nicer Way to Write the Next-State Formula",
                    timestamp: Duration::seconds(530),
                },
                Marker {
                    name: "The Complete TLA+ Spec",
                    timestamp: Duration::seconds(761),
                },
                Marker {
                    name: "Decomposing Large Specs",
                    timestamp: Duration::seconds(839),
                },
                Marker {
                    name: "Epilogue",
                    timestamp: Duration::seconds(908),
                },
            ],
        },
        Video {
            name: "Resources and Tools",
            description: "Describes resources for learning about TLA+.\
                            Explains how to download the Toolbox and shows how \
                             to use it to open a spec, view the pretty-printed \
                              version, and run TLC on it.",
            original_link: "http://lamport.azurewebsites.net/video/video3.html",
            markers: vec![
                Marker {
                    name: "Prologue",
                    timestamp: Duration::seconds(3),
                },
                Marker {
                    name: "TLA+ Resources",
                    timestamp: Duration::seconds(25),
                },
                Marker {
                    name: "Downloading the Toolbox",
                    timestamp: Duration::seconds(103),
                },
                Marker {
                    name: "Creating a Spec",
                    timestamp: Duration::seconds(167),
                },
                Marker {
                    name: "The Pretty-Printer",
                    timestamp: Duration::seconds(276),
                },
                Marker {
                    name: "Running TLC",
                    timestamp: Duration::seconds(315),
                },
                Marker {
                    name: "The TLA+ Proof System",
                    timestamp: Duration::seconds(516),
                },
                Marker {
                    name: "The Unicode Option",
                    timestamp: Duration::seconds(549),
                },
                Marker {
                    name: "Help!",
                    timestamp: Duration::seconds(598),
                },
                Marker {
                    name: "Epilogue",
                    timestamp: Duration::seconds(640),
                },
            ],
        },
        Video {
            name: "Die Hard",
            description: "We save the lives of two Hollywood action heroes. \
             On the way, you will start learning to write TLA+ specs and checking \
            them with the parser and with TLC.",
            original_link: "http://lamport.azurewebsites.net/video/video4.html",
            markers: vec![
                Marker {
                    name: "Prologue",
                    timestamp: Duration::seconds(2),
                },
                Marker {
                    name: "The Die Hard Problem",
                    timestamp: Duration::seconds(25),
                },
                Marker {
                    name: "Getting Started",
                    timestamp: Duration::seconds(67),
                },
                Marker {
                    name: "The Spec",
                    timestamp: Duration::seconds(244),
                },
                Marker {
                    name: "The Next-State Formula",
                    timestamp: Duration::seconds(319),
                },
                Marker {
                    name: "Pouring Between Jugs",
                    timestamp: Duration::seconds(502),
                },
                Marker {
                    name: "Saving Our Heroes",
                    timestamp: Duration::seconds(573),
                },
                Marker {
                    name: "SmallToBig and BigToSmall",
                    timestamp: Duration::seconds(864),
                },
                Marker {
                    name: "Checking Your Definitions",
                    timestamp: Duration::seconds(994),
                },
                Marker {
                    name: "Epilogue",
                    timestamp: Duration::seconds(1152),
                },
            ],
        },
        Video {
            name: "Transaction Commit",
            description: "Commitment, in marriage and database transactions, \
            is specified.  You also learn how to use mathematical functions in specifications.",
            original_link: "http://lamport.azurewebsites.net/video/video5.html",
            markers: vec![
                Marker {
                    name: "Prologue",
                    timestamp: Duration::seconds(2),
                },
                Marker {
                    name: "Weddings",
                    timestamp: Duration::seconds(53),
                },
                Marker {
                    name: "Transaction Commit",
                    timestamp: Duration::seconds(254),
                },
                Marker {
                    name: "The TLA+ Spec",
                    timestamp: Duration::seconds(315),
                },
                Marker {
                    name: "Checking the Spec",
                    timestamp: Duration::seconds(1094),
                },
                Marker {
                    name: "A Parsing Note",
                    timestamp: Duration::seconds(1326),
                },
                Marker {
                    name: "Comments",
                    timestamp: Duration::seconds(1378),
                },
                Marker {
                    name: "Epilogue",
                    timestamp: Duration::seconds(1446),
                },
            ],
        },
        Video {
            name: "Two-Phase Commit",
            description: "How commitment is achieved, in marriage and database \
            transactions.  You also learn about records in TLA+ and some more about using TLC.",
            original_link: "http://lamport.azurewebsites.net/video/video6.html",
            markers: vec![
                Marker {
                    name: "Prologue",
                    timestamp: Duration::seconds(5),
                },
                Marker {
                    name: "Records",
                    timestamp: Duration::seconds(32),
                },
                Marker {
                    name: "Weddings",
                    timestamp: Duration::seconds(117),
                },
                Marker {
                    name: "The TLA+ Spec",
                    timestamp: Duration::seconds(270),
                },
                Marker {
                    name: "The Rest of the Spec",
                    timestamp: Duration::seconds(734),
                },
                Marker {
                    name: "Checking the Spec",
                    timestamp: Duration::seconds(942),
                },
                Marker {
                    name: "Model Values",
                    timestamp: Duration::seconds(1010),
                },
                Marker {
                    name: "Correctness of Two-Phase Commit",
                    timestamp: Duration::seconds(1167),
                },
                Marker {
                    name: "Correctness of Two-Phase Commit",
                    timestamp: Duration::seconds(1248),
                },
            ],
        },
        Video {
            name: "Paxos Commit
",
            description: "Specifies a real fault-tolerant algorithm for \
            committing database transactions.  \
            It explains a few mathematical operations \
            for constructing and combining sets.",
            original_link: "http://lamport.azurewebsites.net/video/video7.html",
            markers: vec![
                Marker {
                    name: "The Algorithm",
                    timestamp: Duration::seconds(41),
                },
                Marker {
                    name: "The Specification",
                    timestamp: Duration::seconds(191),
                },
                Marker {
                    name: "Checking the Spec",
                    timestamp: Duration::seconds(843),
                },
                Marker {
                    name: "Epilogue",
                    timestamp: Duration::seconds(1153),
                },
            ],
        },
        Video {
            name: "Paxos Commit Part 1: Preliminaries",
            description: "A two-part lecture that introduces temporal formulas and explains what it means for one specification to implement another.",
            original_link: "http://lamport.azurewebsites.net/video/video8a.html",
            markers: vec![
                Marker {
                    name: "Prologue",
                    timestamp: Duration::seconds(5),
                },
                Marker {
                    name: "Implication",
                    timestamp: Duration::seconds(37),
                },
                Marker {
                    name: "Ordinary Expressions",
                    timestamp: Duration::seconds(188),
                },
                Marker {
                    name: "Temporal Formulas",
                    timestamp: Duration::seconds(528),
                },
                Marker {
                    name: "Theorems",
                    timestamp: Duration::seconds(833),
                },
                Marker {
                    name: "Epilogue",
                    timestamp: Duration::seconds(931),
                },
            ],
        },
        Video {
            name: "Paxos Commit Part 2: How it Works",
            description: "A two-part lecture that introduces temporal formulas and explains what it means for one specification to implement another.",
            original_link: "http://lamport.azurewebsites.net/video/video8b.html",
            markers: vec![
                Marker {
                    name: "Prologue",
                    timestamp: Duration::seconds(5),
                },
                Marker {
                    name: "The Theorem",
                    timestamp: Duration::seconds(48),
                },
                Marker {
                    name: "Stuttering",
                    timestamp: Duration::seconds(345),
                },
                Marker {
                    name: "Termination and Stopping",
                    timestamp: Duration::seconds(539),
                },
                Marker {
                    name: "Epilogue",
                    timestamp: Duration::seconds(713),
                },
            ],
        },
        Video {
            name: "The Alternating Bit Protocol Part 1: The High Level Specification",
            description: "A two-part lecture that explains liveness, which describes what must happen, and fairness.",
            original_link: "http://lamport.azurewebsites.net/video/video9a.html",
            markers: vec![
                Marker {
                    name: "Prologue",
                    timestamp: Duration::seconds(4),
                },
                Marker {
                    name: "Finite Sequences",
                    timestamp: Duration::seconds(44),
                },
                Marker {
                    name: "What the Protocol Should Accomplish",
                    timestamp: Duration::seconds(338),
                },
                Marker {
                    name: "The High Level Spec",
                    timestamp: Duration::seconds(472),
                },
                Marker {
                    name: "Safety and Liveness",
                    timestamp: Duration::seconds(680),
                },
                Marker {
                    name: "Weak Fairness",
                    timestamp: Duration::seconds(910),
                },
                Marker {
                    name: "Adding Liveness to a Specification",
                    timestamp: Duration::seconds(1060),
                },
                Marker {
                    name: "Epilogue",
                    timestamp: Duration::seconds(1215),
                },
            ],
        },
        Video {
            name: "The Alternating Bit Protocol Part 2: The Protocol",
            description: "A two-part lecture that explains liveness, which describes what must happen, and fairness.",
            original_link: "http://lamport.azurewebsites.net/video/video9b.html",
            markers: vec![
                Marker {
                    name: "Prologue",
                    timestamp: Duration::seconds(4),
                },
                Marker {
                    name: "The Safety Specification",
                    timestamp: Duration::seconds(21),
                },
                Marker {
                    name: "Checking Safety",
                    timestamp: Duration::seconds(394),
                },
                Marker {
                    name: "Liveness",
                    timestamp: Duration::seconds(651),
                },
                Marker {
                    name: "Epilogue",
                    timestamp: Duration::seconds(1069),
                },
            ],
        },
    ];

    for (mut i, video) in videos.iter().enumerate() {
        // Hack, 8 was a double lecture video
        if i >= 8 {
            i = i - 1;
        }
        // Hack, 9 was a double lecture video
        if i >= 9 {
            i = i - 1;
        }
        println!("===BEGIN COPY #{}: Original Name: {}===", i+1, video.name);
        println!("TITLE COPY:\nLamport TLA+ Course Lecture {}: {}\n", i+1, video.name);
        println!("{}", DISCLAIMER);
        println!("Original Description: {}", video.description);
        println!("Original Link: {}", video.original_link);
        println!("\nContents\n");
        let markers = &video.markers;
        for marker in markers {
            println!(
                "{} - {:02}:{:02}",
                marker.name,
                marker.timestamp.num_minutes(),
                marker.timestamp.num_seconds() - marker.timestamp.num_minutes() * 60
            );
        }
        println!("===END COPY: Original Name: {}===", video.name);
    }
}
