use std::env;

fn create_http_link(link: &str) {
    let pre = "<html>\n<head>\n";
    let post = "\n</head>\n</html>";
    println!("{}<meta http-equiv=\"refresh\" content=\"0; url={}\" />{}", pre, link, post);
}

fn main() {
    // dump cli arguments to args variable
    let args: Vec<String> = env::args().skip(1).collect();
    // check if only one argument was given
    if args.iter().count() == 1 {
        let link = &args[0];
        create_http_link(link);
    } else {
        eprintln!("You need to provide exactly one URL as a command-line argument.")
    }
}
