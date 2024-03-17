use murray_rs::{Murray, GetNodeDetailsParams};

fn main() {
  let murray = Murray::default();

  let a = murray.lightning.get_node_details(GetNodeDetailsParams {
    public_key: "03864ef025fde8fb587d989186ce6a4a186895ee44a926bfc370e2c366597a3f8f".to_string(),
  });
  println!("{:?}", a.unwrap());

  let b = murray.lightning.get_statistics();
  println!("{:?}", b.unwrap());
  
  let c = murray.lightning.get_top_nodes();
  println!("{:?}", c.unwrap());

  let d = murray.lightning.get_health();
  println!("{:?}", d.unwrap());
}