use onnxruntime::{environment::Environment, GraphOptimizationLevel};
use ndarray::{Array4, IxDyn, IxDynImpl};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let environment = Environment::builder()
        .with_name("test")
        .build()?;
    let mut session = environment
        .new_session_builder()?
        .with_optimization_level(GraphOptimizationLevel::Basic)?
        .with_number_threads(1)?
        .with_model_from_file("models/yolov5n.onnx")?;
        
    let input_array = Array4::<f32>::zeros((1, 3, 640, 640));
    let input_arrays = vec![input_array.into_dyn()];

    let outputs = session.run::<f32, f32, IxDyn>(input_arrays)?;

    println!("Model Output: {:?}", outputs);

    Ok(())
}

