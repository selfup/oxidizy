![github-pipeline](https://github.com/selfup/oxidizy/actions/workflows/rust.yml/badge.svg)
![gitlab-pipeline](https://gitlab.com/selfup/oxidizy/badges/main/pipeline.svg)

# Oxidizy

An ambitious project to mimic the smaller things in life.

This is how I learned rust.

The idea was to build a cli tool that can generate enough life, and _sublife_ (neutrons, protons, electrons), that when you give the tool atomic weights, it can "generate" appropriate elements.

Now it uses Bevy and the amazing ECS system to constantly iterate and update atoms on the fly with new particles.

Another ambitious goal, would be to set up bonding logic to create water or other compounds.

The most difficult part will be setting up an electro-magnetic field. That is why we are starting with the simplest form of an electro magnetic field.

More to come :rocket:

---

`cargo run --release <n>`

Example Use:

```
$ ./scripts/rel.run.sh 30
+ cargo run -q --release 30
2022-03-04T01:33:07.274214Z  INFO bevy_render::renderer: AdapterInfo { name: "NVIDIA GeForce GTX 1660 Ti", vendor: 4318, device: 8578, device_type: DiscreteGpu, backend: Vulkan }
--------------------------------
Threads: 16
Building..
--------------------------------
Universe built
--------------------------------
Calculating charge of field..
--------------------------------
Field is Cationic
--------------------------------
Atoms: 27000
Baryons: 6372000
Quarks: 19116000
--------------------------------
Total objects in memory: 25515000
--------------------------------

...

2022-03-04T01:33:24.395275Z  INFO bevy diagnostic: frame_time  :    0.011610s (avg 0.012227s)
2022-03-04T01:33:24.395534Z  INFO bevy diagnostic: fps         :   82.042901  (avg 82.155862)
2022-03-04T01:33:24.395819Z  INFO bevy diagnostic: frame_count :  801.000000  (avg 801.0000)      
2022-03-04T01:33:25.388228Z  INFO bevy diagnostic: frame_time  :    0.012523s (avg 0.012366s)
2022-03-04T01:33:25.388604Z  INFO bevy diagnostic: fps         :   81.464006  (avg 81.372364)
2022-03-04T01:33:25.389062Z  INFO bevy diagnostic: frame_count :  882.000000  (avg 882.0000)      
2022-03-04T01:33:26.393287Z  INFO bevy diagnostic: frame_time  :    0.012117s (avg 0.012703s)
2022-03-04T01:33:26.393578Z  INFO bevy diagnostic: fps         :   81.429777  (avg 81.317298)
2022-03-04T01:33:26.393890Z  INFO bevy diagnostic: frame_count :  963.000000  (avg 963.000000)  
```

![image](https://user-images.githubusercontent.com/9837366/99208853-dce09380-277e-11eb-88be-e07d2044b10c.png)

### Sans UI

Bevy/ECS code is not tested. Still in beta and too many refactorings will take place over the next year or so.

However if you want to fill up a bunch of RAM and see how performant the `builder::generate_universe` is, you can run the generate script:

_warning this used up 80% of my RAM and I have 32GB of RAM!_

**`460` is cubed and then 118 default protons and neutrons are made per atom!**

```
$ ./scripts/generate.sh 460
--------------------------------
Threads: 16
Building..
--------------------------------
Universe built
--------------------------------
Calculating charge of field..
--------------------------------
Field is Cationic
--------------------------------
Atoms: 97336000
Baryons: 22971296000
Quarks: 68913888000
--------------------------------
Total objects in memory: 91982520000
--------------------------------

real    0m6.331s
user    0m0.000s
sys     0m0.015s
```

### Repos

Development Repo is on GitHub: https://github.com/selfup/oxidizy

Backup/Archive Repo is on GitLab: https://gitlab.com/selfup/oxidizy
