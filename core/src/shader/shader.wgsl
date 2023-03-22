@vertex fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    switch i32(i) {
        case 0:  { return vec4<f32>(-1.,  1., 0., 1.); }
        case 1:  { return vec4<f32>(-1., -1., 0., 1.); }
        case 2:  { return vec4<f32>( 1., -1., 0., 1.); }
        case 3:  { return vec4<f32>( 1.,  1., 0., 1.); }
        case 4:  { return vec4<f32>(-1.,  1., 0., 1.); }
        default: { return vec4<f32>( 1., -1., 0., 1.); }
    }
}

@group(0) @binding(0)
var<storage, read> data: array<f32>;

var<private> i: i32;
var<private> color: vec4<f32>;
var<private> px: f32;
var<private> py: f32;

fn read_f32() -> f32 {
    var res = data[i];
    i += 1;
    return res;
}

fn draw_rect() {
    if(px < data[i] || px > data[i + 1] || py < data[i + 2] || py > data[i + 3]) {
        i += 8;
        return;
    }
    color = vec4<f32>(data[i + 4], data[i + 5], data[i + 6], data[i + 7]);
    i += 8;
}

@fragment fn fs_main(@builtin(position) pixel_pos: vec4<f32>) -> @location(0) vec4<f32> {
    px = pixel_pos.x;
    py = pixel_pos.y;
    i = 0;
    color = vec4<f32>(0.);
    let data_length = i32(arrayLength(&data));
    while(i < data_length) {
        switch i32(read_f32()) {
            case 1: { draw_rect(); }
            default {}
        }
    }
    return color;
}