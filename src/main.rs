#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::Read;

use rand::Rng;
use serde_json::Value;
use serenity::Client;
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

struct Handler;

lazy_static! {
    // JSON Array keyed by command name with value being an array of links.
    static ref COMMANDS: Value = {
        // Read file.
        let mut file = File::open("./messages.json").expect("messages.json File does not exist");
        // Create String to read file to.
        let mut text = String::new();
        // Read file to text variable.
        file.read_to_string(&mut text).expect("Failed reading file messsages.json");

        // Get JSON Value from the text variable.
        serde_json::from_str(&text).expect("Invalid JSON!")
    };

    // String containing the prefix variable.
    static ref PREFIX: String = {
        // Get "PREFIX" variable from the .env file.
        env::var("PREFIX").expect("Expected PREFIX in .env file.")
    };

}


impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        // Create a ThreadRNG to get a random number.
        let mut rng = rand::thread_rng();

        // Check if message starts with the prefix, skip this message if it doesn't.
        if !msg.content.starts_with(&*PREFIX) { return; };
        // Remove the prefix from the message
        let content: String = msg.content.replace(&*PREFIX, "");
        // Split content into args.
        let args: Vec<&str> = content.split(' ').collect();
        let command = args[0];

        // Get the links for the command.
        let links: Option<&Vec<Value>> = COMMANDS[command].as_array();
        // Check if the command is a valid command, simply return if it is not.
        if links.is_none() { return; };
        let links: &Vec<Value> = links.unwrap();
        // Get a random link index.
        let index = rng.gen_range(0, links.len());

        // Retrieve the link at the given index.
        let url = links.get(index);
        // Send the message and check if it was successfully sent.
        if msg.channel_id.send_message(&ctx.http, |message| message.content(url.unwrap().as_str().unwrap())).is_err() {
            return println!("Could not send the message to channel {}.", msg.channel_id.name(&ctx.cache).unwrap());
        }
    }
}


fn main() {
    dotenv::dotenv().ok();

    // Construct client object.
    let mut client = Client::new(env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env file."), Handler).expect("Error creating client");
    // Start the client and check if there is an error starting it.
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {}.", why);
    }
}
