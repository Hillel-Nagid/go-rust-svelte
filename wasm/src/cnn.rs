use tch::{nn, nn::Conv2D, nn::Linear, nn::MaxPool2D, nn::Module, Tensor};

pub struct CNNModel {
    conv1: Conv2D,
    pool1: MaxPool2D,
    conv2: Conv2D,
    pool2: MaxPool2D,
    fc1: Linear,
    fc2: Linear,
}
