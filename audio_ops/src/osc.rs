//! # Oscillator
//!
//! Sources to connect: frequency.
use crate::function::Fn1;
use crate::phasor::{Phasor, Phasor0};
use audio_vm::{Op, Sample, Stack};

pub struct Osc {
    phasor: Phasor,
    osc: Fn1,
}

impl Osc {
    pub fn new(sample_rate: u32, f: fn(Sample) -> Sample) -> Self {
        let phasor = Phasor::new(sample_rate);
        let osc = Fn1::new(f);
        Osc { phasor, osc }
    }
}

impl Op for Osc {
    fn perform(&mut self, stack: &mut Stack) {
        self.phasor.perform(stack);
        self.osc.perform(stack);
    }

    fn migrate(&mut self, other: &Box<dyn Op>) {
        if let Some(other) = other.downcast_ref::<Self>() {
            self.phasor.migrate_same(&other.phasor);
        }
    }
}

pub struct OscPhase {
    phasor: Phasor0,
    osc: Fn1,
}

impl OscPhase {
    pub fn new(sample_rate: u32, f: fn(Sample) -> Sample) -> Self {
        let phasor = Phasor0::new(sample_rate);
        let osc = Fn1::new(f);
        OscPhase { phasor, osc }
    }
}

impl Op for OscPhase {
    fn perform(&mut self, stack: &mut Stack) {
        self.phasor.perform(stack);
        self.osc.perform(stack);
    }

    fn migrate(&mut self, other: &Box<dyn Op>) {
        if let Some(other) = other.downcast_ref::<Self>() {
            self.phasor.migrate_same(&other.phasor);
        }
    }
}
