![github-pipeline](https://github.com/selfup/oxidizy/actions/workflows/rust.yml/badge.svg)
![gitlab-pipeline](https://gitlab.com/selfup/oxidizy/badges/master/pipeline.svg)

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
--------------------------------
Threads: 16
Building..
--------------------------------
Universe built!
--------------------------------
Field is Anionic
--------------------------------
Atoms: 27000
Baryons: 6372000
Quarks: 19116000
--------------------------------
```

![image](https://user-images.githubusercontent.com/9837366/99208853-dce09380-277e-11eb-88be-e07d2044b10c.png)

### Sans UI

Bevy/ECS code is not tested. Still in beta and too many refactorings will take place over the next year or so.

However if you want to fill up a bunch of RAM and see how performant the `builder::generate_universe` is, you can run the generate script:

_warning this used up 80% of my RAM and I have 32GB of RAM!_

**`480` is cubed and then 118 default protons and neutrons are made per atom!**

```
$ ./scripts/generate.sh 480
--------------------------------
Threads: 16
Building..
--------------------------------
Universe built!
--------------------------------
Field is Cationic
--------------------------------
Atoms: 110592000
Baryons: 26099712000
Quarks: 78299136000
--------------------------------

real    0m12.779s
user    0m0.000s
sys     0m0.000s
```

### Repos

Development Repo is on GitHub: https://github.com/selfup/oxidizy
Backup/Archive Repo is on GitLab: https://gitlab.com/selfup/oxidizy
