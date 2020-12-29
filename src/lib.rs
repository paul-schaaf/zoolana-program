pub mod instruction;
pub mod processor;

solana_program::declare_id!("zoo5fFJszDVcWWhbZYc6CKgM4fxrs5o3rWT9d1tBcNk");

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
