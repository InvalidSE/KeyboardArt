use std::error::Error;
use openrgb::{data::Color, OpenRGB};


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

    println!("controller: {:#?}", (client.get_controller(0).await?).zones);
    println!("controller: {:#?}", (client.get_controller(0).await?).colors); 

    let red = Color::new(255, 0, 255);

    let mut kb_colors = vec![Color::new(0, 0, 0); 107];
    for led in 0..107 {
        kb_colors[led] = Color::new((led as u8)*2, 0, 0);
    }
    // kb_colors[0] = Color::new(255, 0, 0);

    loop {
        kb_colors.rotate_right(1);
        client.update_leds(0, kb_colors.clone()).await?;

        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    }

}



// query and print each controller data
    // for controller_id in 0..controllers {
    //     println!("controller {}: {:#?}", controller_id, client.get_controller(controller_id).await?);

    //     let color = Color::new(255, 0, 0);
    //     let k_id = 0;

    //     client.update_led(controller_id, k_id, color).await?;

    // }

    // for led in 0..107 {
    //     client.update_led(0, led, red).await?;
    // }

        // client.update_leds(0, kb_colors).await?;
