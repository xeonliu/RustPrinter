mod client;

use windows::core::*;
use windows::Win32::Foundation;
use windows::Win32::Graphics::Printing::*;
use crate::client::Client;
// fn main() -> Result<()>{
//     let printer_name = w!("联创打印管理系统");
//     let mut printer_handle = Foundation::HANDLE(core::ptr::null_mut());
//     unsafe {
//         OpenPrinterW(printer_name, &mut printer_handle, None)?;
//     }
//     println!("Printer Handle: {:?}", printer_handle);
//     println!("Hello, world!");
//     Ok(())
// }

#[tokio::main]
async fn main() {
    let client = Client::new();
    client.login().await.unwrap();
}