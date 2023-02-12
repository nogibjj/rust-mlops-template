/*Ported from here: https://raw.githubusercontent.com/nbigaouette */
#![forbid(unsafe_code)]

use onnxruntime::{
    environment::Environment, ndarray::Array, tensor::OrtOwnedTensor, GraphOptimizationLevel,
    LoggingLevel,
};

type Error = Box<dyn std::error::Error>;

pub fn runit() -> Result<Vec<f32>, Error> {
    let environment = Environment::builder()
        .with_name("test")
        // The ONNX Runtime's log level can be different than the one of the wrapper crate or the application.
        .with_log_level(LoggingLevel::Info)
        .build()?;

    let mut session = environment
        .new_session_builder()?
        .with_optimization_level(GraphOptimizationLevel::Basic)?
        .with_number_threads(1)?
        // NOTE: The example uses SqueezeNet 1.0 (ONNX version: 1.3, Opset version: 8),
        //       _not_ SqueezeNet 1.1 as downloaded by '.with_model_downloaded(ImageClassification::SqueezeNet)'
        //       Obtain it with:
        //          curl -LO "https://github.com/onnx/models/raw/master/vision/classification/squeezenet/model/squeezenet1.0-8.onnx"
        .with_model_from_file("/mnt/efs/squeezenet1.0-12.onnx")?;

    let input0_shape: Vec<usize> = session.inputs[0].dimensions().map(|d| d.unwrap()).collect();
    let output0_shape: Vec<usize> = session.outputs[0]
        .dimensions()
        .map(|d| d.unwrap())
        .collect();

    assert_eq!(input0_shape, [1, 3, 224, 224]);
    assert_eq!(output0_shape, [1, 1000, 1, 1]);

    // initialize input data with values in [0.0, 1.0]
    let n: u32 = session.inputs[0]
        .dimensions
        .iter()
        .map(|d| d.unwrap())
        .product();
    let array = Array::linspace(0.0_f32, 1.0, n as usize)
        .into_shape(input0_shape)
        .unwrap();
    let input_tensor_values = vec![array];

    let outputs: Vec<OrtOwnedTensor<f32, _>> = session.run(input_tensor_values)?;

    assert_eq!(outputs[0].shape(), output0_shape.as_slice());
    //capture the top 5 scores
    let mut scores = Vec::new();
    for i in 0..5 {
        scores.push(outputs[0][[0, i, 0, 0]]);
        println!("Score for class [{}] =  {}", i, outputs[0][[0, i, 0, 0]]);
    }
    //return the top 5 scores
    Ok(scores)
}
