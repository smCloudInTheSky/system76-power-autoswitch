extern crate system76_power;
extern crate upower_dbus;
extern crate zbus;

use futures::stream::StreamExt;
use system76_power::args::Args;

use upower_dbus::UPowerProxy;

fn apply_governor(on_battery: bool) {
    let battery_on = Args::Profile {
        profile: Some("battery".to_string()),
    };
    let battery_off = Args::Profile {
        profile: Some("performance".to_string()),
    };

    if on_battery {
        println!("applying battery profile");
        match system76_power::client::client(&battery_on)
            .expect("failed to connect to power daemon")
        {
            Ok(_) => println!("profile updated"),
            Err(e) => println!("{:?}", e),
        };
    } else {
        println!("applying performance profile");
        match system76_power::client::client(&battery_off)
            .expect("failed to connect to power daemon")
        {
            Ok(_) => println!("profile updated"),
            Err(e) => println!("{:?}", e),
        };
    }
}

fn main() -> zbus::Result<()> {
    futures::executor::block_on(async move {
        let connection = zbus::Connection::system().await?;
        let upower = UPowerProxy::new(&connection).await?;

        let mut stream = upower.receive_on_battery_changed().await;

        while let Some(_event) = stream.next().await {
            // TODO : fix c'est dégeux utiliser event à la place !!!
            apply_governor(upower.on_battery().await?);
        }
        Ok(())
    })
}
