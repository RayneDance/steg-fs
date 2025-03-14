# **steg-fs: A Steganographic Filesystem for Windows**

## **Overview**
`steg-fs` is a novel filesystem that stores data by hiding it inside images using steganographic techniques. It presents itself as a virtual drive, allowing seamless file read/write operations while storing data in modified image files. The system includes encryption, integrity verification, and optional original file preservation.

## **Key Features**
⏳ **Virtual Filesystem** – Mounts as a drive, allowing standard file operations.  
🚧 **Steganographic Storage** – Uses image files (PNG, BMP, etc.) as the backend storage medium.  
⏳ **Configurable Data Encoding** – Supports modifying either the least significant bit (LSB) or two least significant bits (2-LSB) of RGB(A) values.  
⏳ **Strong Encryption** – AES-GCM or ChaCha20-Poly1305 encryption to protect hidden data.  
⏳ **Error Detection & Correction** – Ensures data integrity using checksums, parity bits, or Reed-Solomon error correction.  
⏳ **Filesystem Recovery Tool** – Scans images and reconstructs the filesystem in case of corruption.  
⏳ **Secure File Wiping** – Implements safe deletion to prevent data recovery.  
⏳ **Optional Original File Restoration** – Ability to store original image data separately for lossless restoration.  
⏳ **Windows-Only** – Designed specifically for the Windows operating system.

## **Project Architecture**
`steg-fs` is structured into multiple Rust crates, each handling a core aspect of the filesystem:

### 1️⃣ **Image Encoding Crate (`steg-img-encode`)**
- Encodes base64 data into RGB(A) image pixels using LSB techniques.
- Supports configurable bit-depth modification (1-bit or 2-bit per color channel).
- Encrypts data before embedding.

### 2️⃣ **Filesystem Driver Crate (`steg-fs-driver`)**
- Intercepts file read/write operations.
- Interfaces with the storage management crate to allocate space and retrieve data.
- Uses WinFSP to expose the filesystem as a virtual drive in Windows.

### 3️⃣ **Image Decoding Crate (`steg-img-decode`)**
- Extracts hidden data from images.
- Verifies integrity using error detection codes.
- Decrypts extracted data.

### 4️⃣ **Image Writing Crate (`steg-img-write`)**
- Writes data into images while ensuring storage availability.
- Supports optional original file preservation mode.

### 5️⃣ **Filesystem Management Crate (`steg-fs-mgmt`)**
- Manages virtual filesystem metadata (file locations, sizes, available space).
- Handles secure file wiping.
- Implements recovery tools to scan for and rebuild lost files.

### 6️⃣ **Compression Crate (`steg-compress`)**
- Handles gzip compression/decompression.

### 7️⃣ **User Interface Crate (`steg-fs-ui`)**
- GUI/CLI for mounting, configuring, and managing `steg-fs`.
- Displays storage statistics and file mappings.

## **Installation & Usage**
(Instructions for installing dependencies, running the driver, and interacting with the filesystem will be added as development progresses.)

## **License**
MIT License

Copyright (c) 2025 Alexander Jordan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

---
