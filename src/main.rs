use clap::Parser;
use itertools::Itertools;
use log::{debug, error, info, trace};

const BANNER: &str = " ______     __  __     ______     ______     ______   __  \n
/\\  == \\   /\\ \\/\\ \\   /\\  ___\\   /\\  ___\\   /\\__  _\\ /\\ \\   \n
\\ \\  __<   \\ \\ \\_\\ \\  \\ \\___  \\  \\ \\___  \\  \\/_/\\ \\/ \\ \\ \\  \n
 \\ \\_\\ \\_\\  \\ \\_____\\  \\/\\_____\\  \\/\\_____\\    \\ \\_\\  \\ \\_\\ \n
  \\/_/ /_/   \\/_____/   \\/_____/   \\/_____/     \\/_/   \\/_/";

#[derive(Parser, Debug)]
#[clap(name = "Russti")]
#[clap(author = "Developped by @LeoFVO <leofvo@proton.me>")]
#[clap(version = "1.0")]
#[clap(about = "Russti, blazingly fast ssti scanner.", long_about = None)]
pub struct Cli {
    /// The target IP or domain to scan.
    #[clap(short, long)]
    url: String,

    /// The payload to trigger potential SSTI.
    #[clap(short, long)]
    payload: String,

    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    /// Custom user-agent to use
    #[clap(long, default_value = "russti 1.0.0")]
    pub user_agent: String,

    /// The http method to use.
    #[clap(short = 'X', value_parser, arg_enum, default_value = "GET")]
    pub http_method: reqwest::Method,
}

enum Engine {
    // PHP
    Twigg,
    Smarty,
    // JAVA
    FreeMarker,
    Velocity,
    Thymeleaf,
    SpringView,
    Pebble,
    Jinjava,
    // NODEJS
    Jade,
    Handlebars,
    JsRender,
    PugJs,
    NunJucks,
    // PYTHON
    Jinja,
    Tornado,
    Mako,
    // RUBY
    Erb,
    Slim,
    // .NET
    Razor,
    //PERL
    Mojolicious,
    // GO
}

struct Payload {
    value: String,
    expected: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    println!("{}\n\n\n", BANNER);
    println!("Starting scan on {}", cli.url);

    // Initialize the client with default configuration.
    trace!("Initialized client with user-agent: {}", cli.user_agent);
    let client = reqwest::Client::builder()
        .user_agent(cli.user_agent)
        .build()?;
    let request: reqwest::RequestBuilder;

    let payload = Payload {
        value: "#{999*42}".to_string(),
        expected: "41958".to_string(),
    };

    let replaced_paylaod: String = cli.payload.replace("SSTI", &payload.value);
    let builded_payload: (&str, &str) = replaced_paylaod.split("=").collect_tuple().unwrap();

    // builded_payload: (&str, &str) = builded_payload.split("=").collect_tuple().unwrap();

    match cli.http_method {
        reqwest::Method::GET => {
            debug!("Using GET method");
            request = client.get(cli.url).query(&[builded_payload]);
        }
        reqwest::Method::POST => {
            debug!("Using POST method");

            request = client.post(cli.url).form(&[builded_payload]);
        }
        reqwest::Method::PUT => {
            debug!("Using PUT method");

            request = client.put(cli.url).form(&[builded_payload]);
        }
        reqwest::Method::PATCH => {
            debug!("Using PATCH method");

            request = client.patch(cli.url).form(&[builded_payload]);
        }
        reqwest::Method::DELETE => {
            debug!("Using DELETE method");

            request = client.delete(cli.url).form(&[builded_payload])
        }
        _ => {
            error!("Unsupported method");
            panic!("Unsupported method");
        }
    }

    let response = request.send().await?;

    println!("{:#?}", response.text().await?.contains(&payload.expected));

    Ok(())
}
