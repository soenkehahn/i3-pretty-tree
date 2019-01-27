use i3ipc::I3Connection;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let mut connection = I3Connection::connect()?;
    print!("{}", connection.get_tree()?.pretty());
    Ok(())
}
