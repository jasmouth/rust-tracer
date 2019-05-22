# Rust-Tracer

While this began as a simple Rust port/reimplementation of Peter Shirley's wonderful books [Ray Tracing in One Weekend](https://smile.amazon.com/Ray-Tracing-Weekend-Minibooks-Book-ebook/dp/B01B5AODD8) and [Ray Tracing: The Next Week](https://smile.amazon.com/Ray-Tracing-Next-Week-Minibooks-ebook/dp/B01CO7PQ8C), it has since grown to include features not demonstrated therein.

**Notable examples of extra features include:**
* Nonhomogeneous Participating Media (distance sampling performed via Woodcock tracking)
* Support for rendering polygonal primitives and polygon meshes
* Support for multi-threaded rendering
* Anti-Aliasing via [Correlated Multi-Jittered Sampling](http://graphics.pixar.com/library/MultiJitteredSampling/paper.pdf)

[Gallery of Example Renders](./renders/README.md)

## Latest Render

<img src="./output.png" title="Snapshot of the current progress" width="500">
