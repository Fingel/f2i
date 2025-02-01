use fitsio::FitsFile;
use image::imageops::resize;
use image::ImageBuffer;
use ndarray::{stack, Array, Array1, Array2, Axis};
use ndarray_linalg::LeastSquaresSvd;

fn gamma_adjust_table() -> Vec<u8> {
    let size = 255 + 1; // Max size minus min size plus 1
    let mut table = vec![0; size];
    (0..size).for_each(|i| {
        table[i] = (size as f32 * (i as f32 / 255.0).powf(1.0 / 2.5)) as u8;
    });
    table
}

fn linear_scale(image_data: &[f32], zmin: f32, zmax: f32) -> Array1<u8> {
    let mut max = zmax;
    let mut min = zmin;
    if zmax == zmin {
        max = zmax + 1.0;
        min = zmin - 1.0;
    }
    let scale = 255.0 / (max - min);
    let adjust = scale * min;
    let mut scaled_data = Array::from(image_data.to_vec());
    scaled_data = scaled_data.clamp(min, max);
    scaled_data *= scale;
    scaled_data -= adjust;
    scaled_data = scaled_data.round();
    let gamma_lookup = gamma_adjust_table();
    // todo figure out how to use select here
    scaled_data
        .iter()
        .map(|&x| gamma_lookup[x as usize])
        .collect::<Array1<u8>>()
}

#[derive(Debug)]
struct LeastSquareResult {
    slope: f32,
    intercept: f32,
    num_iterations: usize,
    num_samples: usize,
    rms: f32,
}

fn least_squares_line_fit(sample_data: &[f32]) -> LeastSquareResult {
    let num_samples = sample_data.len();
    let x: Vec<f32> = (0..num_samples).map(|i| i as f32).collect();

    let a: Array2<f32> = stack![ndarray::Axis(1), x, vec![1.0; num_samples]];
    let y = Array::from(sample_data.to_vec());
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

#[derive(Debug)]
struct ZscaleBounds {
    min: f32,
    max: f32,
}

fn calc_zscale(sample_data: &[f32]) -> ZscaleBounds {
    let contrast = 0.1; // Hardcoded for now

    let nsamples = sample_data.len();
    let lsq_fit = least_squares_line_fit(sample_data);
    let zmin = sample_data[0];
    let zmax = sample_data[nsamples - 1];
    let mut slope = lsq_fit.slope;

    if contrast > 0.0 {
        slope /= contrast;
    }

    let fitted_dy = slope * nsamples as f32 / 2.0;

    ZscaleBounds {
        min: zmin.max(lsq_fit.intercept - fitted_dy),
        max: zmax.min(lsq_fit.intercept + fitted_dy),
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
    let median = sampled_data[sampled_data.len() / 2];
    let min_max = calc_zscale(&sampled_data);
    let mut scaled = linear_scale(&image_data, median, min_max.max);
    scaled.invert_axis(Axis(0));
    let image: ImageBuffer<image::Luma<u8>, _> =
        ImageBuffer::from_vec(2080, 2048, scaled.to_vec()).unwrap();
    let resized = resize(&image, 200, 197, image::imageops::FilterType::Gaussian);
    resized.save("scaled.jpg").unwrap();
}
