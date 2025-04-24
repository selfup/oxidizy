![github-pipeline](https://github.com/selfup/oxidizy/actions/workflows/rust.yml/badge.svg)
    ![gitlab-pipeline](https://gitlab.com/selfup/oxidizy/badges/main/pipeline.svg)

# Oxidizy

An ambitious project to mimic the smaller things in life.

This is how I learned rust.

The idea was to build a cli tool that can generate enough life, and _sublife_ (neutrons, protons, electrons), that when you give the tool atomic weights, it can "generate" appropriate elements.

Now it uses Bevy and the amazing ECS system to constantly iterate and update atoms on the fly with new particles.

Another ambitious goal, would be to set up bonding logic to create water or other compounds.

The most difficult part will be setting up an electro-magnetic field. That is why we are starting with the simplest form of an electro magnetic field.

Core universe generator `unigen` lives here now: https://github.com/selfup/unigen-rs

---

`cargo run --release <n>`

Example Use (1.728 million atoms rendered on screen):

```
$ ./scripts/rel.run.sh 120
+ [[ 120 != '' ]]
+ cargo run -q --release 120
2025-01-03T03:02:58.363556Z  INFO bevy_diagnostic::system_information_diagnostics_plugin::internal: SystemInfo { os: "Windows 10 Pro", kernel: "19045", cpu: "AMD Ryzen 7 3800X 8-Core Processor", core_count: "8", memory: "31.9 GiB" }
2025-01-03T03:02:58.670692Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce RTX 3060 Ti", vendor: 4318, device: 9353, device_type: DiscreteGpu, driver: "NVIDIA", driver_info: "566.36", backend: Vulkan }
2025-01-03T03:02:58.860730Z  INFO bevy_winit::system: Creating new window "App" (0v1#4294967296)
--------------------------------
Threads: 16
Building.. 
--------------------------------
Universe built
--------------------------------
Calculating charge of field..
--------------------------------
Field is Anionic
--------------------------------
Atoms: 1728000
Baryons: 407808000
Quarks: 1223424000
--------------------------------
Total objects in memory: 1632960000
--------------------------------
2025-01-03T03:03:39.747135Z  INFO bevy diagnostic: fps        :    1.887225   (avg 6.317538)
2025-01-03T03:03:39.747410Z  INFO bevy diagnostic: frame_time :  529.878600ms (avg 259.353702ms)
2025-01-03T03:03:39.747693Z  INFO bevy diagnostic: frame_count:  170.000000   (avg 110.500000)
2025-01-03T03:03:40.813600Z  INFO bevy diagnostic: fps        :    1.867156   (avg 6.206554)
2025-01-03T03:03:40.813866Z  INFO bevy diagnostic: frame_time :  535.573900ms (avg 266.200975ms)
2025-01-03T03:03:40.814134Z  INFO bevy diagnostic: frame_count:  172.000000   (avg 112.500000)
2025-01-03T03:03:41.873834Z  INFO bevy diagnostic: fps        :    1.875142   (avg 6.096975)
2025-01-03T03:03:41.874101Z  INFO bevy diagnostic: frame_time :  533.292900ms (avg 273.076387ms)
2025-01-03T03:03:41.874380Z  INFO bevy diagnostic: frame_count:  174.000000   (avg 114.500000)
```

![image](https://github.com/user-attachments/assets/dd767f05-5a4d-4304-9904-991d68ebb261)

### Repos

Development Repo is on GitHub: https://github.com/selfup/oxidizy

Backup/Archive Repo is on GitLab: https://gitlab.com/selfup/oxidizy
