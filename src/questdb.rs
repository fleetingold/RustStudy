use questdb::{
    Result,
    ingress::{
        Sender,
        Buffer,
        SenderBuilder}};

fn main() -> Result<()> {
   let mut sender = SenderBuilder::new("localhost", 9009).connect()?;
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