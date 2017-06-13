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
                                        }],
                      }];

    for video in videos {
        println!("===BEGIN COPY: Original Name: {}===", video.name);
        println!("{}", DISCLAIMER);
        println!("Original Description: {}", video.description);
        println!("Original Link: {}", video.original_link);
        println!("\nContents\n");
        for marker in video.markers {
            println!("{} - {:02}:{:02}",
                     marker.name,
                     marker.timestamp.num_minutes(),
                     marker.timestamp.num_seconds());
        }
        println!("===END COPY: Original Name: {}===", video.name);
    }
}
