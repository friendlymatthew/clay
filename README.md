# clay

This crate provides a interactive design tool as a functional Yew component. Currently, clay can freehand, draw rectangles, and render text on an infinite canvas.

In addition to the functional component, this crate has a library for editing and tooling logic. Every geometric operation makes use of wasm32 SIMD intrinsics.

clay's current state is quite limited. There isn't a clear roadmap ahead but the most exciting topics include multicollaboration, saving app state, and **circles**. 


## Performance
The effects of SIMD are not measured yet. But the original implementation [written by Alikiki](https://github.com/alikiki/baby-tldraw) uses a scalar implementation for geometric operations. An ad-hoc benchmark between the two is planned and would be very welcomed.

## Literature 
[Baby Tldraw](https://www.hajeon.xyz/posts/post-content/20240413_tldraw.html)

[Creating a Zoom UI](https://www.steveruiz.me/posts/zoom-ui)

> A canvas is a fixed 2-dimensional plane of infinite dimensions. All shapes and objects rendered on the canvas operate in 2-dimensional space. 
> 
> The camera is suspended above and points at the canvas. The camera operates in 3-dimensional space.
>
> The implementations for converting from screen to canvas and finding the viewport can be found in Steve Ruiz's blog post above.


## SIMD
clay operates on 128-bit SIMD vectors or 4 32-bit floats. Canvas objects are represented as `<f32, f32, _, _> : f32x4` vectors. The camera lives in 3-dimensional space with `<f32, f32, f32, _>: f32x4` coordinates.

The geometric operations involved are trivial to implement in SIMD.
