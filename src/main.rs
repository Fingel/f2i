use fitsio::FitsFile;
use ndarray::{stack, Array, Array2};
use ndarray_linalg::LeastSquaresSvd;

// fn linear_scale(image_data: &[f32], zmin: f32, zmax: f32) {
//     let max: f32;
//     let min: f32;
//     if zmax == zmin {
//         max = zmax + 1.0;
//         min = zmin - 1.0;
//     } else {
//         max = zmax;
//         min = zmin;
//     }
//     let scale = 255.0 / (max - min);
//     println!("{:?}", scale);
//     let adjust = scale * min;
//     let mut scaled_data: Vec<u8> = image_data
//         .iter()
//         .map(|x| (x * scale - adjust).max(min).min(max) as u8)
//         .collect();
//     println!("{:?}", scaled_data);
// }

#[derive(Debug)]
struct LeastSquareResult {
    slope: f32,
    intercept: f32,
    num_iterations: usize,
    num_samples: usize,
    rms: f32,
}

fn least_squares_line_fit(image_data: &[f32]) -> LeastSquareResult {
    let num_samples = image_data.len();
    let x: Vec<f32> = (0..num_samples).map(|i| i as f32).collect();

    let a: Array2<f32> = stack![ndarray::Axis(1), x, vec![1.0; num_samples]];
    let y = Array::from(image_data.to_vec());
    let result = a.least_squares(&y).unwrap();
    let mean_residual = result
        .residual_sum_of_squares
        .unwrap()
        .first()
        .unwrap_or(&0.0)
        / num_samples as f32;
    let rms = mean_residual.sqrt();

    LeastSquareResult {
        slope: result.solution[0],
        intercept: result.solution[1],
        num_iterations: 1,
        num_samples,
        rms,
    }
}

fn extract_samples(image_data: &[f32]) -> Vec<f32> {
    // Return 2000 samples from the image data, sorted

    let num_samples = 2000;
    let mut samples: Vec<f32> = image_data
        .iter()
        .step_by(image_data.len() / num_samples)
        .skip(1)
        .cloned()
        .collect();
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    samples
}

fn main() {
    let mut fptr = FitsFile::open("ogg2m001-ep03-20241216-0739-e00.fits").unwrap();
    fptr.pretty_print().unwrap();
    let hdu = fptr.hdu(0).unwrap();
    let image_data: Vec<f32> = hdu.read_image(&mut fptr).unwrap();
    let sampled_data = extract_samples(&image_data);
    let fit = least_squares_line_fit(&sampled_data);
    println!("{:?}", fit);
}
