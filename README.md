# Merkle Tree Service

A Rust-based web service that provides a Merkle tree implementation with REST API endpoints for adding leaves, retrieving proofs, and managing the tree structure.

## Installation

1. Clone the repository:
```bash
git clone https://github.com/AsierTenaZuazolacigorraga/merkle-tree-service.git
cd merkle-tree-service
```

2. Install dependencies:
```bash
cargo build
```

## Usage

### Starting the Service

Run the service with:
```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`


The service has the following endpoints:
* /add-leaf: Adds a single leaf to the Merkle Tree.
* /add-leaf: Adds multiple leaves to the Merkle Tree.
* /get-num-leaves Returns the number of leaves in the Merkle Tree.
* /get-root: Returns the Merkle root of the tree.
* /get-proof: Returns the Merkle proof for the leaf at the given index.

### Example Usage

#### Using curl

```bash
# Add a single leaf
curl -X POST http://127.0.0.1:3000/add-leaf \
  -H "Content-Type: application/json" \
  -d '"leaf"'

# Get the number of leaves
curl -X GET http://127.0.0.1:3000/get-num-leaves

# Get the root hash
curl -X GET http://127.0.0.1:3000/get-root

# Get a proof for leaf at index 2
curl -X GET http://127.0.0.1:3000/get-proof \
  -H "Content-Type: application/json" \
  -d 2
```

## Configuration

The service currently uses hardcoded configuration:

- **Host**: 127.0.0.1
- **Port**: 3000
- **Tree Depth**: 32

## Limitations

Proper error handling is TBD