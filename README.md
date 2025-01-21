# aim2go

# Screen Capture Pipeline

'''mermaid
graph TD;
    A[Frame Capture] -->|Capture full screen using windows-capture| B
    B -->|Resize & Normalize using image crate| C
    C -->|Convert to Tensor using yolo-rs| D
    D -->|Run Inference with yolo-rs| E
    E -->|Parse Detections| F
    F -->|Identify Targets| G
    G -->|Move Mouse using enigo| H
    H -->|Optimize Loop| A

    subgraph Optimization_Strategies
        I[Asynchronous Execution with tokio]
        J[Hardware Acceleration with GPU]
        K[Adaptive Resolution Strategy]
        L[Frame Skipping]
        M[Caching Intermediate Results]
    end

    A -.-> I
    B -.-> J
    C -.-> K
    D -.-> L
    E -.-> M
'''
    
## Execution Flow (Async Parallelization)

1. **Frame Capture:** Using `windows-capture` to grab the screen.
2. **Preprocessing:** Resize and normalize image using `image` crate.
3. **Tensor Conversion:** Convert image to tensor using `yolo-rs`.
4. **Inference:** Run YOLO model inference using `yolo-rs`.
5. **Detection Processing:** Parse results and filter objects.
6. **Mouse Movement:** Smooth and human-like movement using `enigo`.
7. **Loop Optimization:** Overlap tasks for performance improvement.

## Optimization Strategies
1. **Asynchronous Execution:** Each stage runs in parallel using Rust's `tokio`.
2. **Hardware Acceleration:** Utilize GPU for screen capture and inference.
3. **Adaptive Resolution:** Lower peripheral resolution while maintaining high center resolution.
4. **Frame Skipping:** Skip frames when no movement is detected.
5. **Caching:** Store intermediate results to avoid redundant computations.
