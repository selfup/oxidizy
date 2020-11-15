# Oxidizy

An ambitious project to mimic the smaller things in life.

This is how I learned rust.

The idea was to build a cli tool that can generate enough life, and _sublife_ (neutrons, protons, electrons), that when you give the tool atomic weights, it can "generate" appropriate elements.

Now it uses Bevy and it's amazing ECS system to constantly iterate and update atoms on the fly with new particles.

Another ambitious goal, would be to set up bonding logic to create water or other compounds.

The most difficult part will be setting up an electro-magnetic field. That is why we are starting with the simplest form of an electro magnetic field.

More to come :rocket:

---

`cargo run --release <n>`

Example Use:

```bash
$ ./scripts/rel.run.sh 30
+ cargo build -q --release
+ cargo run -q --release 30
Building Universe..
Threads: 16
Universe built!
Checking the charge..
Field is Anionic
Amount of Atoms in Universe: 27000
```

![image](https://user-images.githubusercontent.com/9837366/99133467-5dfd2680-25d7-11eb-9377-de7feb36336d.png)

### Testing

Bevy/ECS code is not tested. Still in beta and too many refactorings will take place over the next year or so.

However if you want to fill up a bunch of RAM and see how performant the `builder::generate_universe` is, you can run the test script which runs in release mode:

_warning this used up 80% of my RAM and I have 32GB of RAM!_

**`SHRED_SIZE=190` is cubed and then 118 default protons and neutrons are made per atom!**

```
$ SHRED_SIZE=190 ./scripts/test.sh
+ cargo test --release -- --nocapture
   Compiling oxidizy v0.1.0 (C:\Users\boudi\Documents\Rust\oxidizy)
    Finished release [optimized] target(s) in 1.65s
     Running target\release\deps\oxidizy-0d20c3fcb5f5d1ba.exe

running 4 tests
Building Universe..
Threads: 16
Threads: 16
Threads: 16
test builder::it_can_sense_the_field ... ok
test builder::it_can_begin ... ok
test builder::it_can_infer_the_charge_of_an_atom ... ok
Threads: 16
Universe built!
Checking the charge..
Field is Cationic
Amount of Atoms in Universe: 6859000
Amount of protons and neutrons: 1618724000
Time to generate/mutate/pass in ms: 5793
test builder::it_can_shred ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
