use maliput_sys::plugin::ffi::CreateRoadNetworkWrapper;

fn main() {
  println!("\nExecuting create_rn.rs example: ");

  let road_network_loader_id = "maliput_malidrive".to_string();
  let properties = vec![
    "road_geometry_id:my_rg_from_rust".to_string(),
    "opendrive_file:/opt/ros/foxy/share/maliput_malidrive/resources/odr/TShapeRoad.xodr".to_string(),
  ];

  let road_network = CreateRoadNetworkWrapper(&road_network_loader_id, &properties);
  let road_geometry = road_network.road_geometry();

  // Excercise the RoadGeometry bindings.
  unsafe {
    // This is unsafe because we are dereferencing a raw pointer.
    println!("linear_tolerance: {}", (*road_geometry).linear_tolerance());
    println!("angular_tolerance: {}", (*road_geometry).angular_tolerance());
    println!("num_junctions: {}", (*road_geometry).num_junctions());
    println!("num_branch_points: {}", (*road_geometry).num_branch_points());
  }
}
