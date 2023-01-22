/*library for loading pre-trained PyTorch */
use anyhow::{bail, Result};
use tch::nn::ModuleT;
use tch::vision::{imagenet, resnet};

//create a constant for the model
pub const MODEL: &str = "../model/resnet18.ot";

//load model and predict image passed in as argument and return output
pub fn predict_image(image_path: &str) -> Result<tch::Tensor> {
    let image_file = std::path::Path::new(image_path);
    if !image_file.exists() {
        bail!("Image file does not exist");
    }
    // Load the image and resize it to the input size expected by the model.
    let image = imagenet::load_image_and_resize224(image_file)?;
    let mut vs = tch::nn::VarStore::new(tch::Device::Cpu);
    //print current working directory
    println!(
        "Current working directory: {}",
        std::env::current_dir().unwrap().display()
    );
    //Verify that the model exists in the cost path by printing the model path and size
    let model_path = std::path::Path::new(MODEL);
    println!("Model path: {}", model_path.display());
    println!("Model size: {}", model_path.metadata().unwrap().len());

    //load model
    let resnet18 = resnet::resnet18(&vs.root(), imagenet::CLASS_COUNT);
    vs.load(model_path)?;
    let output = resnet18
        .forward_t(&image.unsqueeze(0), /*train=*/ false)
        .softmax(-1, tch::Kind::Float);
    Ok(output)
}
