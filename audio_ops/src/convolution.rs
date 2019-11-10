//! # Convolution
//!
//! Convolve two signals by making dot-product of a N-sample sliding window on both.
//!
//! Sources to connect: input and kernel, but roles are vague in this case.
use audio_vm::{Frame, Op, Stack, CHANNELS};
use itertools::izip;
use std::collections::VecDeque;

pub struct Convolution {
    window: VecDeque<Frame>,
}

impl Convolution {
    pub fn new(window_size: usize) -> Self {
        let mut window = VecDeque::with_capacity(window_size);
        for _ in 0..window_size {
            window.push_back([0.0; CHANNELS]);
        }
        Convolution { window }
    }
}

impl Op for Convolution {
    fn perform(&mut self, stack: &mut Stack) {
        let mut frame = [0.0; CHANNELS];
        let kernel = stack.pop();
        let input = stack.pop();
        for (sample, &x, &y) in izip!(&mut frame, &input, &kernel) {
            *sample = x * y;
        }
        self.window.pop_front();
        self.window.push_back(frame);

        let mut frame = [0.0; CHANNELS];
        for xs in self.window.iter() {
            for (sample, x) in izip!(&mut frame, xs) {
                *sample += x;
            }
        }
        stack.push(&frame);
    }
}

pub struct ConvolutionM {
    window: VecDeque<Frame>,
    kernel: Vec<Frame>,
}

impl ConvolutionM {
    pub fn new(window_size: usize) -> Self {
        let zero = [0.0; CHANNELS];
        let mut window = VecDeque::with_capacity(window_size);
        for _ in 0..window_size {
            window.push_front(zero);
        }
        ConvolutionM {
            window,
            kernel: vec![zero; window_size],
        }
    }
}

impl Op for ConvolutionM {
    fn perform(&mut self, stack: &mut Stack) {
        for kernel in self.kernel.iter_mut().rev() {
            *kernel = stack.pop();
        }
        self.window.pop_front();
        self.window.push_back(stack.pop());

        let mut frame = [0.0; CHANNELS];
        for (input, kernel) in izip!(&self.window, &self.kernel) {
            for (sample, &x, &y) in izip!(&mut frame, input, kernel) {
                *sample += x * y
            }
        }
        stack.push(&frame);
    }
}