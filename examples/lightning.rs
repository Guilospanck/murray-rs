use murray_rs::{GetNodeDetailsParams, LightningError, Murray};

fn lightning_example() -> std::result::Result<(), LightningError> {
  let murray = Murray::default();

  let a = murray.lightning.get_node_details(GetNodeDetailsParams {
    public_key: "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f".to_string(),
  })?;
  println!("{:?}", a);

  let b = murray.lightning.get_statistics()?;
  println!("{:?}", b);
  
  let c = murray.lightning.get_top_nodes()?;
  println!("{:?}", c);

  let d = murray.lightning.get_health()?;
  println!("{:?}", d);

  Ok(())
}

fn main() {
  let _ = lightning_example();
}