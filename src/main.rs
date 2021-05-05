use clap::Clap;

#[derive(Clap, Debug)]
struct Args {
    input: String,
    output: Option<String>,
}
struct InputImage {
    pub width: usize,
    pub height: usize,
    pub data: Vec<[f32; 3]>,
}

impl InputImage {
    fn new(width: usize, height: usize, data: Vec<[f32; 3]>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }
}

fn main() {
    let args = Args::parse();
    let input_file_name = args.input;
    let output_file_name = args.output.unwrap_or(format!("{}.png", &input_file_name));
    let input_image = exr::prelude::read_first_rgba_layer_from_file(
        &input_file_name,
        |width_height, _channels| {
            let px = [0f32, 0f32, 0f32];
            let data = vec![px; width_height.width() * width_height.height()];
            let img = InputImage::new(width_height.width(), width_height.height(), data);
            img
        },
        |img, xy, (r, g, b, _a): (f32, f32, f32, f32)| {
            img.data[xy.x() + img.width * xy.y()] = [r, g, b];
        },
    )
    .expect("error decoding input .exr file")
    .layer_data
    .channel_data
    .pixels;
    let mut output_image = vec![0u8; input_image.width * input_image.height * 3];
    do_the_convertion(
        input_image.width,
        input_image.height,
        &input_image.data,
        &mut output_image,
    );
    image::save_buffer(
        &output_file_name,
        &output_image,
        input_image.width as u32,
        input_image.height as u32,
        image::ColorType::Rgb8,
    )
    .expect("error saving output file");
}

fn do_the_convertion(width: usize, height: usize, input: &[[f32; 3]], output: &mut [u8]) -> () {}
