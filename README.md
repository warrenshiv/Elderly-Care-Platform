# Decentralized Pet Care and Adoption Management System

This project is a decentralized platform built on the Internet Computer for managing pet care, adoptions and pet events. It allows users to register as pet owners, shelters, or adopters and manage pets, adoption requests, pet care events, feedback, and donations. The platform ensures robust access control and user management.

## Key Features

1. **User Management**
   - **Add User:** Allows users to create profiles as Owners, Shelters, or Adopters.
   - **Get All Users:** Retrieve a list of all user profiles.

2. **Pet Management**
   - **Add Pet:** Allows users to add pets to the system.
   - **Get All Pets:** Retrieve a list of all pets available in the system.

3. **Adoption Request Management**
   - **Create Adoption Request:** Allows adopters to request the adoption of a pet.
   - **Get All Adoption Requests:** Retrieve a list of all adoption requests.

4. **Pet Care Event Management**
   - **Create Pet Care Event:** Allows organizers to create pet care events.
   - **Get All Pet Care Events:** Retrieve a list of all pet care events.

5. **Feedback Management**
   - **Create Feedback:** Allows users to provide feedback on pets or events.
   - **Get All Feedback:** Retrieve a list of all feedback entries.

6. **Donation Management**
   - **Create Donation:** Allows users to make donations to the system.
   - **Get All Donations:** Retrieve a list of all donations.

## Error Handling
   - **Not Found:** Returns an error if a requested item is not found.
   - **Unauthorized Access:** Returns an error if a user tries to perform an action without necessary permissions.


## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```