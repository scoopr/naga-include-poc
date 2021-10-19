# The polyglot shader concatenator

This proof of concept implements "shader file includes"  with `naga` shader language frontends (glsl and wgsl) by means of combining the parsed outputs, instead of textual processing, and outputs a combined shader as a `wgsl` shader.

Because I could. To let the shader language live in harmony.

## Example

Lets look at `example.wgsl`

```wgsl
//include:example-util.glsl
//include:example-types.glsl
//include:example-types.wgsl
//include:example-widget.glsl

struct WidgetThing {
    widget: Widget;
    color: vec4<f32>;
};


[[block]]
struct Input1 {
    quux: Quux;
    xyzzy: Xyzzy;
};

[[group(0), binding(0)]]
var<uniform> input1: Input1;


struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
};


[[stage(vertex)]]
fn example_vertex_entrypoint() -> VertexOutput  {

    let sz = sqr(2.0);
    let widget_thing = WidgetThing(make_widget(sz), vec4<f32>(0.0, 0.0, 0.0, 1.0));

    return widget_thing.widget.pos;    
}

```

First there is some include directives as specially formed comments. Then rest of the shader looks fairly normal, but it refers to some types and functions that are not visible anywhere, like `Widget`,  `Quux`, `Xyzzy`, `sqr` and `make_widget`.

Lets see where they come from,

`example-util.glsl`:
```glsl
//include:example-util.wgsl
float sqr(float a) {
    return mul(a,a);
}
```
Ok, that declares the `sqr` in a `glsl` shader. It in itself calls `mul` that is declared further in `example-util.wgsl`.

The types `Quux` and `Xyzzy` are declared in the `example-types` files, the other as `glsl` and the other as `wgsl`.

```glsl
struct Quux {
    vec4 a;
};

```
```wgsl
struct Xyzzy {
    view: mat4x4<f32>;
};
```

## Result

Combining them and outputting a `wgsl` seems to work (and validate!)

```wgsl
struct Quux {
    a: vec4<f32>;
};

struct Xyzzy {
    view: mat4x4<f32>;
};

struct Widget {
    size: f32;
    pos: vec3<f32>;
};

struct WidgetThing {
    widget: Widget;
    color: vec4<f32>;
};

[[block]]
struct Input1 {
    quux: Quux;
    xyzzy: Xyzzy;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
};

[[group(0), binding(0)]]
var<uniform> input1: Input1;

fn mul(a: f32, b: f32) -> f32 {
    return (a * b);
}

fn sqr(a1: f32) -> f32 {
    var a2: f32;

    a2 = a1;
    let e4: f32 = a2;
    let e5: f32 = a2;
    let e6: f32 = mul(e4, e5);
    return e6;
}

fn make_widget(sz: f32) -> Widget {
    var sz1: f32;
    var w: Widget;

    sz1 = sz;
    let e4: f32 = sz1;
    w.size = e4;
    w.pos = vec3<f32>(0.0, 1.0, 0.0);
    let e10: Widget = w;
    return e10;
}

[[stage(vertex)]]
fn example_vertex_entrypoint() -> VertexOutput {
    let e2: f32 = sqr(2.0);
    let e3: Widget = make_widget(e2);
    let widget_thing: WidgetThing = WidgetThing(e3, vec4<f32>(0.0, 0.0, 0.0, 1.0));
    return widget_thing.widget.pos;
}


```


## How does it work

I made some small changes to `naga` frontends to not start with a empty `naga::Module`, but continue on what is already built. This required populating some lookup tables. Also for `glsl` I made the shaderstage optional (and to not require entrypoint)

## Conclusion

It kinda seems to work, not that I excercised it with any real shaders. Edge-cases may be plentiful. Its not really _that_ ugly, I think.

The requirement for frontends to cope with already populated module may be somewhat an anti-feature for naga developers (extra complexity), but the changes to get this far were fairly minimal!

This doesn't tackle conditional at all, which is another big part of shader pre-processors.

In the end, perhaps much riskier than standard text processed includes, which makes it as it is perhaps not quite as useful, but it could perhaps be used as springboard to reimagine shader workflows.


