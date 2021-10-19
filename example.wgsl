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
