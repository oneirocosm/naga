fn load_wgsl(name: &str) -> naga::Module {
    let path = format!("{}/test-data/{}.wgsl", env!("CARGO_MANIFEST_DIR"), name);
    let input = std::fs::read_to_string(path).unwrap();
    naga::front::wgsl::parse_str(&input).unwrap()
}

#[test]
fn convert_quad() {
    let module = load_wgsl("quad");
    {
        use naga::back::msl;
        let mut binding_map = msl::BindingMap::default();
        binding_map.insert(
            msl::BindSource { set: 0, binding: 0 },
            msl::BindTarget { buffer: None, texture: Some(1), sampler: None },
        );
        binding_map.insert(
            msl::BindSource { set: 0, binding: 1 },
            msl::BindTarget { buffer: None, texture: None, sampler: Some(1) },
        );
        let options = msl::Options {
            binding_map: &binding_map,
        };
        msl::write_string(&module, options).unwrap();
    }
}

#[test]
fn convert_boids() {
    let _module = load_wgsl("boids");
}