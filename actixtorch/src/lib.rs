/* Library to load a pretrained model and classify an image.
read from on disk model and image only
*/

use anyhow::{bail, Result};
use tch::nn::ModuleT;
use tch::vision::{imagenet, resnet};

//create a const for the model and image
const MODEL: &str = "resnet18.ot";
const IMAGE: &str = "image.jpg";

pub fn predict() -> Result<()> {
    let weights = std::path::Path::new(MODEL);
    let image = IMAGE.to_owned();
    let image = imagenet::load_image_and_resize224(image)?;

    // Create the model and load the weights from the file.
    let mut vs = tch::nn::VarStore::new(tch::Device::Cpu);
    let net: Box<dyn ModuleT> = match weights.file_name().unwrap().to_str().unwrap() {
        "resnet18.ot" => Box::new(resnet::resnet18(&vs.root(), imagenet::CLASS_COUNT)),

        _ => bail!("unknown model, use a weight file named e.g. resnet18.ot"),
    };
    vs.load(weights)?;

    // Apply the forward pass of the model to get the logits.
    let output = net
        .forward_t(&image.unsqueeze(0), /* train= */ false)
        .softmax(-1, tch::Kind::Float); // Convert to probability.

    // Print the top 5 categories for this image.
    for (probability, class) in imagenet::top(&output, 5).iter() {
        println!("{:50} {:5.2}%", class, 100.0 * probability)
    }
    Ok(())
}
