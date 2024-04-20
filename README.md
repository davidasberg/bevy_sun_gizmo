[![Crates.io](https://img.shields.io/crates/v/bevy_sun_gizmo)](https://crates.io/crates/bevy_sun_gizmo)
[![docs.rs](https://docs.rs/bevy_sun_gizmo/badge.svg)](https://docs.rs/bevy_sun_gizmo)

<div style="text-align: center">
  <h1>Bevy Sun Gizmo</h1>
</div>

![A screen recording showing the gizmo in action](https://github.com/davidasberg/bevy_sun_gizmo/assets/47108520/f609f647-d06f-44fa-8ef6-8b7e1d4530ee)

## Summary

Bevy Sun Gizmo is a plugin for the [Bevy](https://bevyengine.org) game engine that provides a simple gizmo for visualizing and controlling the direction of the sun (main directional light) in a 3D scene. It is heavily inspired by gizmo and controller found in Unreal Engine. 

## Features:

- Gizmo for visualizing the direction of the sun
- Mouse controls for changing the direction of the sun
- Configurable gizmo size and position
- Configurable key bindings for invoking the controller and gizmo

## Controls

Hold RightCtrl + L to invoke the controller and gizmo, then use the mouse to change the direction of the sun.

## How to use

Add the plugin:

```rust ignore
.add_plugins(SunGizmoPlugin)
```

Check out the examples for more details.

## Version Compatibility

| bevy | bevy_sun_gizmo |
|------|----------------------|
| 0.13 | 0.1.0                |

## License

All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there
are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.
