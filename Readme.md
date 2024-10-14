# Pedersen's hash function

This project provides a Rust implementation of Pedersen's Hash Function, a cryptographic hash function that maps fixed-size tuples of elements from modular arithmetic into elements of finite cyclic groups. Pedersen's hash function is widely used in zero-knowledge proofs, cryptographic protocols, and blockchain applications due to its properties of hiding inputs while maintaining commitments.

The implementation focuses on efficiency and security, allowing users to easily compute hash values for various inputs while ensuring the integrity and non-repudiation of the hashed data.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Installation

To use this implementation, add the following dependency to your Cargo.toml

[dependencies]
pedersen-hash = "0.1.0" # Replace with the latest version

## Usage

TTo include this Pedersen hash function in your Rust project, add the following to your `Cargo.toml`:

````toml
[dependencies]
pedersen-hash = "0.1.0"  # Replace with the latest version

        ```
        use Pedersen_hash_function::pedersen_hash_function;
        let x = 1;
        let y = 2;
        let cyclic_group = 5;
        let pedersen_hash = pedersen_hash_function(x ,y, 0, cyclic_group);

        assert_eq!(pedersen_hash,3);


        ```

## Contributing

1. Fork the repository.

2. Navigate to the project directory:
   ```bash
   cd moon-math
````

3. Checkout to this branch:

   ```bash
   git checkout -b pedersens-hash-function
   ```

4. Run the following command:

cargo run

## License

This project is licensed under the MIT License.
