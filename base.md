# About Yew-Base

## Steps Used to Create Application

The git repository and project was created first, so the Rust application was initialized with ```cargo init```.

The code was copied and modified from the yew-primer mentioned elsewhere. As I experiment more with Yew, I will update this repository to reflect that.

## Getting Started

Out of the box, this is not a Rust application but a starting point. To get started, clone this repository
```
git clone https://github.com/Justin-Garey/yew-base.git
```

Rename the directory to your project name.
```
mv yew-base new-project-name
```

Rename the project in [Cargo.toml](./Cargo.toml) 

In [src/main.rs](./src/main.rs), the top line **use yew_base::app::app::App;** needs modified to reflect the new cargo project.

The nginx.conf should also be modified to use the correct application name.

