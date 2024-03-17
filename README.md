# Adoption Center Backend

The Adoption Center Backend is a Rust-based backend service designed to manage adopters and animals within an adoption center system. It provides functionalities to register new adopters, add new animals, perform animal adoptions, update adopter and animal details, delete adopters and animals, list all adopters and animals, mark an animal as not adopted, and list all adopters who adopted a specific animal.

## Features

- **Register Adopter**: Allows registering a new adopter with details such as name, contact details, and desired animal type.
- **Add Animal**: Enables adding a new animal with details such as name, species, and age.
- **Perform Animal Adoption**: Facilitates the process of adopting an animal by updating its adoption status.
- **Update Adopter Details**: Allows updating details of a registered adopter, including name, contact details, and desired animal type.
- **Update Animal Details**: Enables updating details of a registered animal, including name, species, and age.
- **Delete Adopter**: Allows deleting a registered adopter from the system.
- **Delete Animal**: Enables deleting a registered animal from the system.
- **List Adopters**: Retrieves a list of all registered adopters.
- **List Animals**: Retrieves a list of all registered animals.
- **Mark Animal as Not Adopted**: Marks an animal as not adopted in the system.
- **List Adopters of Animal**: Retrieves a list of all adopters who have adopted a specific animal.

## Usage

The Adoption Center Backend provides both query and update functions that can be accessed through its Candid interface. These functions can be invoked to interact with the backend service and perform various operations related to adopters and animals.

## Installation

To integrate the Adoption Center Backend into your system, include the provided Rust code in your project and ensure that the necessary dependencies are installed. Compile the code and deploy the resulting backend service to your desired environment.

## Dependencies

- `serde`: A Rust library for serializing and deserializing data structures.
- `candid`: A Rust library for generating Candid interfaces for Motoko.
- `ic_stable_structures`: Provides stable data structures for use in the Internet Computer (IC) environment.

1. **Install Rust and Dependencies**
   - Ensure you have Rust installed, version 1.64 or higher. You can install it using the following commands:
     ```bash
     $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
     $ source "$HOME/.cargo/env"
     ```
   - Install the `wasm32-unknown-unknown` target:
     ```bash
     $ rustup target add wasm32-unknown-unknown
     ```
   - Install `candid-extractor`:
     ```bash
     $ cargo install candid-extractor
     ```

2. **Install DFINITY SDK (`dfx`)**
   - Install `dfx` using the following commands:
     ```bash
     $ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
     $ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
     $ source ~/.bashrc
     $ dfx start --background
     ```

3. **Update Dependencies**
   - Update the `dependencies` block in `/src/{canister_name}/Cargo.toml` with the required dependencies.

4. **Autogenerate DID**
   - Add the provided script to the root directory of the project.
   - Update line 16 with the name of your canister.
   - Run the script each time you modify/add/remove exported functions of the canister.

5. **Running the Project Locally**
   - Start the replica, running in the background:
     ```bash
     $ dfx start --background
     ```
   - Deploy your canisters to the replica and generate your Candid interface:
     ```bash
     $ npm run gen-deploy
     ```