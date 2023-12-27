#[cxx::bridge(namespace = "maliput::api")]
pub mod ffi {
    unsafe extern "C++" {
        include!("create_road_network_wrapper.h");
        include!("maliput/api/road_network.h");
        include!("maliput/api/road_geometry.h");

        type RoadNetwork;
        type RoadGeometry;
        // RoadNetwork bindings definitions.
        fn road_geometry(self: &RoadNetwork) -> *const RoadGeometry;
        // RoadGeometry bindings definitions.
        fn num_junctions(self: &RoadGeometry) -> i32;
        fn linear_tolerance(self: &RoadGeometry) -> f64;
        fn angular_tolerance(self: &RoadGeometry) -> f64;
        fn num_branch_points(self: &RoadGeometry) -> i32;

        #[namespace = "maliput::plugin"]
        fn CreateRoadNetworkWrapper(road_network_loader_id: &String, properties: &Vec<String>) -> UniquePtr<RoadNetwork>;
    }
}
