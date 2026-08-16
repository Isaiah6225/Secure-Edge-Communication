# Secure-Edge-Communication

Lightweight enrollment and secure session protocol for resource-constrained 
embedded devices (ESP32-S3), built as the communication foundation for a 
distributed embedded ML runtime.

## Status: In progress (~80% complete)
Device-side enrollment and session handshake implemented and tested. 
Server-side (Fedora) enrollment verification nearing completion. 
Standard message encryption/decryption layer in progress.

## Design
- Device authentication: ECDSA (P-256)
- Key exchange: ECDH (ephemeral)
- Session protection: AES-GCM
- Replay protection: nonce 

## Why
Built as the first subsystem of a larger project: a memory-safe (Rust) 
RTOS for distributed ML inference on constrained embedded devices. 
Full writeup forthcoming.

## Stack
Rust, no_std where applicable, ESP32-S3 (esp-rs), Fedora 44 server (tokio for client management)
