#pragma once

#include "maliput/api/road_network.h"
#include "maliput/plugin/create_road_network.h"

#include <map>
#include <string>

#include <rust/cxx.h>

namespace maliput {
namespace plugin {

/// Creates a wrapper around maliput::plugin::CreateRoadNetwork.
/// cxx crate can't handle std::map, so we use std::vector instead.
/// The properties vector is a list of key-value pairs, where the key and value are strings.
/// The key and value are separated by a colon, e.g. "key:value".
std::unique_ptr<maliput::api::RoadNetwork> CreateRoadNetworkWrapper(const rust::String& road_network_loader_id,
                                                             const rust::Vec<rust::String>& properties) {
  std::map<std::string, std::string> map_properties;
    for (const auto& property : properties) {
        const std::string prop = std::string(property);
        const auto colon_index = prop.find(':');
        if (colon_index == std::string::npos) {
            throw std::runtime_error("maliput-sys: invalid property: " + prop);
        }
        const auto key = prop.substr(0, colon_index);
        const auto value = prop.substr(colon_index + 1);
        map_properties[key] = value;
    }
   std::unique_ptr<maliput::api::RoadNetwork> rn = maliput::plugin::CreateRoadNetwork(std::string(road_network_loader_id), map_properties);
  return rn;
}

} // namespace plugin
}  // namespace maliput
