
#[cxx::bridge(namespace = "maliput::plugin")]
pub mod ffi {
    unsafe extern "C++" {
        include!("create_road_network_wrapper.h");

        #[namespace = "maliput::api"]
        type RoadNetwork = crate::api::ffi::RoadNetwork;

        fn CreateRoadNetworkWrapper(road_network_loader_id: &String, properties: &Vec<String>) -> UniquePtr<RoadNetwork>;
    }
}
