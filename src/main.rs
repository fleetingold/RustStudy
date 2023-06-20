use questdb::{
    Result,
    ingress::{
        Buffer,
        SenderBuilder}};

fn main() -> Result<()> {
   let mut sender = SenderBuilder::new("192.168.1.148", 9009).connect()?;
   let mut buffer = Buffer::new();
   buffer
       .table("sensors")?
       .symbol("id", "toronto1")?
       .column_f64("temperature", 20.0)?
       .column_i64("humidity", 50)?
       .at_now()?;
   sender.flush(&mut buffer)?;
   Ok(())
}