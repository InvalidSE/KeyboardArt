use std::{error::Error, io};
use openrgb::{data::Color, OpenRGB};
extern crate websocket;
use websocket::client::ClientBuilder;

const CONNECTION: &'static str = "ws://127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let client = OpenRGB::connect().await?;
    let controllers = client.get_controller_count().await?;

    if controllers == 0 {
        println!("No controllers found");
        return Ok(());
    } else {
        println!("Found {} controller(s):", controllers);
        for controller_id in 0..controllers {
            println!("controller {}: {:#?}", controller_id, (client.get_controller(controller_id).await?).name);
        }
    }

    // println!("controller: {:#?}", (client.get_controller(0).await?).zones);
    // println!("controller: {:#?}", (client.get_controller(0).await?).colors); 

    // there will be a json 107 long rgb array that will be read in and used to set the colors of the keyboard, sent from teh websocket server

    // let mut ws_client = ClientBuilder::new(CONNECTION)
    //     .unwrap()
    //     .connect_insecure()
    //     .unwrap();

    let mut kb_colors = vec![Color::new(0, 0, 0); 107];

    let mut input_string = String::new();

    loop {
        // let msg = ws_client.recv_message().unwrap();
        // if msg.is_text() {
        //     let text = msg.into_text().unwrap();
        //     let rgb: Vec<&str> = text.split(",").collect();
        //     for i in 0..107 {
        //         kb_colors[i] = Color::new(rgb[i].parse::<u8>().unwrap(), rgb[i+1].parse::<u8>().unwrap(), rgb[i+2].parse::<u8>().unwrap());
        //         client.update_leds(0, kb_colors.clone()).await?;
        //     }
        // }

        // light up every key in sequence
        for i in 0..107 {
            kb_colors[i] = Color::new(255, 0, 0);
            client.update_leds(0, kb_colors.clone()).await?;
            kb_colors[i] = Color::new(100, 0, 100);

            io::stdin().read_line(&mut input_string).unwrap();

            // tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }

    }

}