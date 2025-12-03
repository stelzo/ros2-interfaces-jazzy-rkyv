# ROS2 interfaces with rkyv

This repository contains Rust structs for all interfaces (i.e., messages and services) that are listed as releases
on the [ROS Index for Jazzy](https://index.ros.org/packages/#jazzy).

This crate is the rkyv version of [this crate](https://crates.io/crates/ros2-interfaces-jazzy), that was originally written for `ros2-client`. The types are mainly used for non-ROS systems that can use rkyv for (de)serialization.

Every package is a separate feature for this crate, so you can cherry-pick the interfaces you need.
Example are `std_msgs`, `geometry_msgs` and `nav2_msgs`. 

## Known Issues

### Disabled Interfaces
The following packages (and thereby: features) do not compile due to incompatible or missing interface definitions:

* `depthai_ros_msgs`
* `mrpt_msgs`
* `mrpt_nav_interfaces`
* `rosbag2_test_msgdefs`
* `sick_scan_xd`

Therefore, the features have been commented out in the `Cargo.toml` file.
