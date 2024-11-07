Requirements
Custom Pallet for Embedding Storage
Storage Items: Define storage items to hold the embeddings, metadata, and other necessary data.
Embeddings Storage: Use a map storage type with an identifier key for each embedding, storing the vector representation.
Metadata Storage: Store additional metadata (like timestamps, tags, or relations) that can support alternative lookup methods (e.g., graph traversal).
Weight Management: Define custom weights for storage and retrieval calls, especially since embedding vectors can be large. This will help manage on-chain storage costs and performance.
Transaction Functions for Managing Embeddings
Add Embedding: A function for nodes or users to submit new embeddings, along with associated metadata.
Retrieve Embedding: Functions to retrieve embeddings, either by identifier or based on metadata (e.g., tags or categories). Initially, you could use identifiers directly; more advanced search options can follow.
Remove Embedding: A function to remove embeddings if they’re no longer needed, especially useful for maintaining storage efficiency.
Custom Extrinsics for Search and Retrieval
k-NN or Similarity Search: Develop an extrinsic that performs k-nearest neighbors (k-NN) or cosine similarity searches. This can either run directly on-chain or invoke off-chain workers for intensive computation.
Graph-Based Search (Blue sky): If you’re using metadata for a graph structure, include an extrinsic for graph-based lookups or traversals. Off-Chain Workers (Optional but Useful)
Embedding Generation and Processing: If embeddings need to be created or processed before storing, off-chain workers can handle this. They can also manage similarity calculations if you prefer not to do this directly on-chain.
Efficient Search Operations: Off-chain workers can handle intensive k-NN searches or metadata graph traversals, allowing you to keep the blockchain lean while supporting advanced queries.
Multi-modal Inference: Using the Roost, a Rust iteration of a previous generation of inference hub, we can provide the agents with inference tooling for working on our chain.
RPC Endpoints for Accessing Embeddings
Custom RPC Methods: Define custom RPC calls to interact with your pallet. This will allow external clients to add, retrieve, or query embeddings without using direct extrinsics. For example:
get_embedding_by_id: Retrieve embeddings by their ID.
get_embeddings_by_metadata: Query embeddings based on metadata tags. These RPCs will serve as the main interface for agents or external apps that need to interact with the memory system.
Access Control and Permissions
Permissions for Embedding Creation and Retrieval: Implement basic access control to regulate who can add, modify, or retrieve embeddings. This could be role-based or based on staked tokens for access rights, ensuring the system remains secure.
Stake Requirements (Optional): Since you’re storing potentially large vectors, you may want to include a staking or fee mechanism to cover storage and retrieval costs, rewarding nodes maintaining this data.
Benchmarking and Weight Configuration
Benchmarks for Transactions: Benchmark your functions to calculate the weights, especially for adding and querying embeddings, as these may be resource-intensive.
Weight Adjustments: Set appropriate weights to ensure the chain’s performance doesn’t suffer from embedding operations. This may include custom weights for heavy storage calls like adding a full embedding vector.
Testing and Optimization
Unit Tests: Create tests for each function, focusing on the storage and retrieval of embeddings. Test the behavior under different scenarios (e.g., large embeddings, many embeddings).
Performance Optimization: Initially, keep vector size and metadata relatively simple to test the proof of concept; later, you can optimize for higher volumes and complex queries.
Frontend or CLI Interface (Optional)
To interact with the chain, a simple frontend or CLI tool can be helpful. This can facilitate adding embeddings, running queries, and visualizing the stored data, making it easier to test and demonstrate the MVP.