use maliput_sys::maliput_math_saturate;

fn main() {
  println!("\nExecuting call_saturate.rs example: ");
  unsafe {
    let saturated = maliput_math_saturate(1.0, 2.0, 3.0);
    println!("\tSaturate: {}", saturated);
  }
}
