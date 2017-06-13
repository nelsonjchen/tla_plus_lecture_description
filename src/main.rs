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
    let videos = vec![Video {
                          name: "Introduction to TLA+",
                          description: "Explains what TLA+ is and why you might want to use it.  \
                                        It introduces the concept of a state machine.",
                          original_link: "http://lamport.azurewebsites.net/video/intro.html",
                          markers: vec![Marker {
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
                                        }],
                      },
                      Video {
                          name: "State Machines in TLA+",
                          description: "Shows how a simple state machine is \
                           described in TLA+, giving the first glimpse of a TLA+ specification. ",
                          original_link: "http://lamport.azurewebsites.net/video/smintla.html",
                          markers: vec![Marker {
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
                                        }],
                      },
                      Video {
                          name: "Resources and Tools",
                          description: "Describes resources for learning about TLA+.\
                            Explains how to download the Toolbox and shows how \
                             to use it to open a spec, view the pretty-printed \
                              version, and run TLC on it.",
                          original_link: "http://lamport.azurewebsites.net/video/video3.html",
                          markers: vec![Marker {
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
                                        }],
                      },
                      Video {
                          name: "Resources and Tools",
                          description: "Describes resources for learning about TLA+.\
                            Explains how to download the Toolbox and shows how \
                             to use it to open a spec, view the pretty-printed \
                              version, and run TLC on it.",
                          original_link: "http://lamport.azurewebsites.net/video/video3.html",
                          markers: vec![Marker {
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
                                        }],
                      }];

    for (i, video) in videos.iter().enumerate() {
        println!("===BEGIN COPY #{}: Original Name: {}===", i, video.name);
        println!("{}", DISCLAIMER);
        println!("Original Description: {}", video.description);
        println!("Original Link: {}", video.original_link);
        println!("\nContents\n");
        let markers = &video.markers;
        for marker in markers {
            println!("{} - {:02}:{:02}",
                     marker.name,
                     marker.timestamp.num_minutes(),
                     marker.timestamp.num_seconds() - marker.timestamp.num_minutes() * 60);
        }
        println!("===END COPY: Original Name: {}===", video.name);
    }
}