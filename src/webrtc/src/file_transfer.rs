use crate::error::{IPChatWebRTCError, Result};
use crate::types::FileMetadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

const CHUNK_SIZE: usize = 16384; // 16KB chunks

/// File transfer progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTransferProgress {
    pub transfer_id: String,
    pub bytes_transferred: u64,
    pub total_bytes: u64,
    pub percentage: f64,
}

impl FileTransferProgress {
    pub fn transfer_id(&self) -> String {
        self.transfer_id.clone()
    }

    pub fn bytes_transferred(&self) -> u64 {
        self.bytes_transferred
    }

    pub fn total_bytes(&self) -> u64 {
        self.total_bytes
    }

    pub fn percentage(&self) -> f64 {
        self.percentage
    }
}

/// Manages file transfers
pub struct FileTransfer {
    transfer_id: String,
    files: Vec<FileMetadata>,
    current_file_index: usize,
    current_chunk_index: usize,
    bytes_transferred: u64,
    total_bytes: u64,
    file_data: Vec<Vec<u8>>,
}

impl FileTransfer {
    /// Create a new file transfer
    pub fn new(files: JsValue) -> Result<FileTransfer> {
        let files: Vec<FileMetadata> = serde_wasm_bindgen::from_value(files)
            .map_err(|e| IPChatWebRTCError::SerializationError(e.to_string()))?;

        let total_bytes: u64 = files.iter().map(|f| f.size).sum();

        Ok(FileTransfer {
            transfer_id: Uuid::new_v4().to_string(),
            files,
            current_file_index: 0,
            current_chunk_index: 0,
            bytes_transferred: 0,
            total_bytes,
            file_data: Vec::new(),
        })
    }

    /// Get transfer ID
    pub fn id(&self) -> String {
        self.transfer_id.clone()
    }

    /// Get total number of files
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Get progress information
    pub fn progress(&self) -> FileTransferProgress {
        let percentage = if self.total_bytes > 0 {
            (self.bytes_transferred as f64 / self.total_bytes as f64) * 100.0
        } else {
            0.0
        };

        FileTransferProgress {
            transfer_id: self.transfer_id.clone(),
            bytes_transferred: self.bytes_transferred,
            total_bytes: self.total_bytes,
            percentage,
        }
    }

    /// Check if transfer is complete
    pub fn is_complete(&self) -> bool {
        self.bytes_transferred >= self.total_bytes
    }

    /// Get files metadata as JSON
    pub fn files_json(&self) -> Result<String> {
        serde_json::to_string(&self.files)
            .map_err(|e| IPChatWebRTCError::SerializationError(e.to_string()))
    }
}

impl FileTransfer {
    /// Prepare file data for sending
    pub fn prepare_file_data(&mut self, data: Vec<Vec<u8>>) -> Result<()> {
        if data.len() != self.files.len() {
            return Err(IPChatWebRTCError::FileTransferError(
                "File data count doesn't match metadata count".to_string(),
            ));
        }
        self.file_data = data;
        Ok(())
    }

    /// Get next chunk to send
    pub fn next_chunk(&mut self) -> Option<FileChunk> {
        if self.current_file_index >= self.file_data.len() {
            return None;
        }

        let file_data = &self.file_data[self.current_file_index];
        let start = self.current_chunk_index * CHUNK_SIZE;

        if start >= file_data.len() {
            // Move to next file
            self.current_file_index += 1;
            self.current_chunk_index = 0;
            return self.next_chunk();
        }

        let end = (start + CHUNK_SIZE).min(file_data.len());
        let chunk_data = file_data[start..end].to_vec();

        let chunk = FileChunk {
            transfer_id: self.transfer_id.clone(),
            file_index: self.current_file_index,
            chunk_index: self.current_chunk_index,
            data: chunk_data,
        };

        self.bytes_transferred += (end - start) as u64;
        self.current_chunk_index += 1;

        Some(chunk)
    }
}

/// Represents a chunk of file data
#[derive(Debug, Clone)]
pub struct FileChunk {
    pub transfer_id: String,
    pub file_index: usize,
    pub chunk_index: usize,
    pub data: Vec<u8>,
}

/// Manages receiving file transfers
pub struct FileReceiver {
    transfers: HashMap<String, TransferState>,
}

struct TransferState {
    files: Vec<FileMetadata>,
    chunks: HashMap<usize, HashMap<usize, Vec<u8>>>,
    bytes_received: u64,
    total_bytes: u64,
}

impl FileReceiver {
    pub fn new() -> Self {
        Self {
            transfers: HashMap::new(),
        }
    }

    /// Start receiving a transfer
    pub fn start_transfer(&mut self, transfer_id: String, files: Vec<FileMetadata>) {
        let total_bytes: u64 = files.iter().map(|f| f.size).sum();

        let state = TransferState {
            files,
            chunks: HashMap::new(),
            bytes_received: 0,
            total_bytes,
        };

        self.transfers.insert(transfer_id, state);
    }

    /// Process a received chunk
    pub fn receive_chunk(&mut self, chunk: FileChunk) -> Result<Option<FileTransferProgress>> {
        let state = self.transfers.get_mut(&chunk.transfer_id).ok_or_else(|| {
            IPChatWebRTCError::FileTransferError("Transfer not found".to_string())
        })?;

        // Store chunk
        state
            .chunks
            .entry(chunk.file_index)
            .or_insert_with(HashMap::new)
            .insert(chunk.chunk_index, chunk.data.clone());

        state.bytes_received += chunk.data.len() as u64;

        // Calculate progress
        let percentage = if state.total_bytes > 0 {
            (state.bytes_received as f64 / state.total_bytes as f64) * 100.0
        } else {
            0.0
        };

        let progress = FileTransferProgress {
            transfer_id: chunk.transfer_id.clone(),
            bytes_transferred: state.bytes_received,
            total_bytes: state.total_bytes,
            percentage,
        };

        Ok(Some(progress))
    }

    /// Assemble received files
    pub fn assemble_files(&mut self, transfer_id: &str) -> Result<Vec<Vec<u8>>> {
        let state = self.transfers.remove(transfer_id).ok_or_else(|| {
            IPChatWebRTCError::FileTransferError("Transfer not found".to_string())
        })?;

        let mut files = Vec::new();

        for file_index in 0..state.files.len() {
            let file_chunks = state.chunks.get(&file_index).ok_or_else(|| {
                IPChatWebRTCError::FileTransferError(format!(
                    "Missing chunks for file {}",
                    file_index
                ))
            })?;

            // Sort chunks by index and concatenate
            let mut chunk_indices: Vec<_> = file_chunks.keys().copied().collect();
            chunk_indices.sort_unstable();

            let mut file_data = Vec::new();
            for chunk_index in chunk_indices {
                if let Some(chunk_data) = file_chunks.get(&chunk_index) {
                    file_data.extend_from_slice(chunk_data);
                }
            }

            files.push(file_data);
        }

        Ok(files)
    }

    /// Get progress for a transfer
    pub fn get_progress(&self, transfer_id: &str) -> Option<FileTransferProgress> {
        let state = self.transfers.get(transfer_id)?;

        let percentage = if state.total_bytes > 0 {
            (state.bytes_received as f64 / state.total_bytes as f64) * 100.0
        } else {
            0.0
        };

        Some(FileTransferProgress {
            transfer_id: transfer_id.to_string(),
            bytes_transferred: state.bytes_received,
            total_bytes: state.total_bytes,
            percentage,
        })
    }
}

impl Default for FileReceiver {
    fn default() -> Self {
        Self::new()
    }
}
