fn main() {

// This is a softmax regression for MNIST digits classification with Adam.
// This achieves 0.918 test accuracy after 3 epochs (0.11 sec/epoch on 2.7GHz Intel Core i5).
use autograd::{self as ag, Graph, optimizers::adam, ndarray_ext as arr, tensor::Variable};

let rng = ag::ndarray_ext::ArrayRng::<f32>::default();
let w_arr = arr::into_shared(rng.glorot_uniform(&[28 * 28, 10]));
let b_arr = arr::into_shared(arr::zeros(&[1, 10]));
let adam_state = adam::AdamState::new(&[&w_arr, &b_arr]);

let max_epoch = 3;

for epoch in 0..max_epoch {
   ag::with(|g| {
       let w = g.variable(w_arr.clone());
       let b = g.variable(b_arr.clone());
       let x = g.placeholder(&[-1, 28*28]);
       let y = g.placeholder(&[-1]);
       let z = g.matmul(x, w) + b;
       let mean_loss = g.reduce_mean(g.sparse_softmax_cross_entropy(z, &y), &[0], false);
       let grads = &g.grad(&[&mean_loss], &[w, b]);
       let update_ops: &[ag::Tensor<f32>] =
           &adam::Adam::default().compute_updates(&[w, b], grads, &adam_state, g);

       // let batch_size = 200isize;
       // let num_samples = x_train.shape()[0];
       // let num_batches = num_samples / batch_size as usize;
       // for i in get_permutation(num_batches) {
       //     let i = i as isize * batch_size;
       //     let x_batch = x_train.slice(s![i..i + batch_size, ..]).into_dyn();
       //     let y_batch = y_train.slice(s![i..i + batch_size, ..]).into_dyn();
       //     g.eval(update_ops, &[x.given(x_batch), y.given(y_batch)]);
       // }
   });
}

}
