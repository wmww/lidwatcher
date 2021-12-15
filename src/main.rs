use dbus::{
    arg,
    blocking::{
        stdintf::org_freedesktop_dbus::Properties, // allows us to call get
        Connection,
    },
    Message,
};
use std::error::Error;
use std::time::Duration;

type GenericResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct PropertiesChangedHappened {
    pub sender: String,
}

impl arg::AppendAll for PropertiesChangedHappened {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.sender, i);
    }
}

impl arg::ReadAll for PropertiesChangedHappened {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(PropertiesChangedHappened { sender: i.read()? })
    }
}

impl dbus::message::SignalArgs for PropertiesChangedHappened {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

fn watch_for_lid() -> GenericResult<()> {
    let conn = Connection::new_system()?;
    let proxy = conn.with_proxy(
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower",
        Duration::from_secs(1),
    );
    //let _prop: arg::PropMap = proxy.get("org.freedesktop.UPower", "LidIsClosed")?;
    let _id = proxy.match_signal(|h: SignalHappened, _: &Connection, _: &Message| {
        println!("Hello happened from sender: {}", h.sender);
        true
    })?;
    loop {
        conn.process(Duration::from_secs(1))?;
    }
}

fn main() {
    // NOTE: sudo systemd-inhibit --what=handle-lid-switch sleep 30 can inhibit idle, which we need to do until we've
    // locked the screen or whatever
    watch_for_lid().expect("error watching for lid");
    println!("Hello, world!");
}
