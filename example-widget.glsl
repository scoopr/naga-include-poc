
struct Widget {
    float size;
    vec3 pos;
};


Widget make_widget(float sz) {
    Widget w;
    w.size = sz;
    w.pos = vec3(0.0, 1.0, 0.0);
    return w;
}
