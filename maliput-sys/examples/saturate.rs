use maliput_sys::maliput::math::saturate;

fn main() {
  println!("\nExecuting call_saturate.rs example: ");
  let saturated = saturate(1.0, 0.0, 2.0);
  println!("\tSaturate: {}", saturated);
}
