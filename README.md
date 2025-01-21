# aim2go

# Screen Capture Pipeline

```mermaid
graph TD;
    A[Frame Capture] -->|Capture full screen using `windows-capture`| B
    B -->|Resize & Normalize using `image` crate| C
    C -->|Convert t::o Tensor using `yolo-rs`| D
    D -->|Run Inference with `yolo-rs`| E
    E -->|Parse Detections| F
    F -->|Identify Targets| G
    G -->|Move Mouse using `enigo`| H
    H -->|Optimize Loop| A

    subgraph Optimization Strategies
        I[Asynchronous Execution with `tokio`]
        J[Hardware Acceleration (GPU)]
        K[Adaptive Resolution Strategy]
        L[Frame Skipping]
        M[Caching Intermediate Results]
    end

    A -.-> I
    B -.-> J
    C -.-> K
    D -.-> L
    E -.-> M
