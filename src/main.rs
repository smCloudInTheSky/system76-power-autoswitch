extern crate upower_dbus ;
extern crate zbus ;
extern crate system76_power ; 

use futures::stream::StreamExt;
use upower_dbus::UPowerProxy;
use system76_power::client::PowerClient;
use crate::system76_power::Power;

fn apply_governor(on_battery: bool) {
    let client = PowerClient::new();
    if on_battery {
        println!("applying battery profile");
        match client.expect("failed to connect to power daemon").battery() {
            Ok(num) => num,
            Err(e) => println!("{:?}",e),
        };
    } else {
        println!("applying performance profile");
        match client.expect("failed to connect to daemon").performance() {
            Ok(num) => num,
            Err(e) => println!("{:?}",e),
        };
    }
}


fn main() -> zbus::Result<()> {
    futures::executor::block_on(async move {
        let connection = zbus::Connection::system().await?;
        let upower = UPowerProxy::new(&connection).await?;

        apply_governor(upower.on_battery().await?) ;

        let mut stream = upower.receive_on_battery_changed().await;

        while let Some(event) = stream.next().await {
            apply_governor(upower.on_battery().await?) ;
        }
        Ok(())
    })
}
