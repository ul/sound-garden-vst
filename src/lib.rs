use crate::context::Context;
use audio_graph::prelude::*;
use parking_lot::Mutex;
use std::sync::Arc;
use vst::plugin::{Info, Plugin};

mod context;
#[macro_use]
mod macros;
mod ui;

const CHANNELS: usize = 2;
const PARAMETERS: usize = 16;

struct SoundGarden {
    context: Arc<Mutex<Context>>,
    editor: ui::Editor,
    graph: Arc<Mutex<AudioGraph>>,
    input: Vec<Sample>,
    parameters: Vec<f64>,
}

impl Default for SoundGarden {
    fn default() -> Self {
        let context = Arc::new(Mutex::new(Context {
            channels: CHANNELS,
            sample_rate: 48_000,
            parameters: PARAMETERS,
        }));
        let graph = Arc::new(Mutex::new(AudioGraph::new(CHANNELS, PARAMETERS)));
        let editor = ui::Editor::new(context.clone(), graph.clone());
        SoundGarden {
            context,
            editor,
            graph,
            input: vec![0.0; CHANNELS + PARAMETERS],
            parameters: vec![0.0; PARAMETERS],
        }
    }
}

impl Plugin for SoundGarden {
    fn get_info(&self) -> Info {
        Info {
            name: "Sound Garden".to_string(),
            vendor: "Ruslan Prokopchuk".to_string(),
            unique_id: 1_804_198_801,
            inputs: CHANNELS as i32,
            outputs: CHANNELS as i32,
            f64_precision: true,
            parameters: PARAMETERS as i32, // param:<N>
            version: 1,
            category: vst::plugin::Category::Synth,
            ..Default::default()
        }
    }

    fn get_editor(&mut self) -> Option<&mut vst::editor::Editor> {
        Some(&mut self.editor)
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.context.lock().sample_rate = rate as usize;
    }

    fn can_be_automated(&self, _index: i32) -> bool {
        true
    }

    fn get_parameter(&self, index: i32) -> f32 {
        if index < PARAMETERS as i32 {
            self.parameters[index as usize] as f32
        } else {
            0.0
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        if index < PARAMETERS as i32 {
            self.parameters[index as usize] = Sample::from(value);
        }
    }

    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        let (inputs, mut outputs) = buffer.split();

        // Iterate over inputs as (&f32, &f32)
        let (left, right) = inputs.split_at(1);
        let stereo_in = left[0].iter().zip(right[0].iter());

        // Iterate over outputs as (&mut f32, &mut f32)
        let (mut left, mut right) = outputs.split_at_mut(1);
        let stereo_out = left[0].iter_mut().zip(right[0].iter_mut());

        // Prepare parameters and graph
        self.input[CHANNELS..].clone_from_slice(&self.parameters);
        let mut g = self.graph.lock();

        // Zip and process
        for ((left_in, right_in), (left_out, right_out)) in stereo_in.zip(stereo_out) {
            self.input[0] = Sample::from(*left_in);
            self.input[1] = Sample::from(*right_in);
            let output = g.sample(&self.input);
            *left_out = output[0] as f32;
            *right_out = output[1] as f32;
        }
    }

    fn process_f64(&mut self, buffer: &mut vst::buffer::AudioBuffer<f64>) {
        let (inputs, mut outputs) = buffer.split();

        // Iterate over inputs as (&f32, &f32)
        let (left, right) = inputs.split_at(1);
        let stereo_in = left[0].iter().zip(right[0].iter());

        // Iterate over outputs as (&mut f32, &mut f32)
        let (mut left, mut right) = outputs.split_at_mut(1);
        let stereo_out = left[0].iter_mut().zip(right[0].iter_mut());

        // Prepare parameters and graph
        self.input[CHANNELS..].clone_from_slice(&self.parameters);
        let mut g = self.graph.lock();

        // Zip and process
        for ((left_in, right_in), (left_out, right_out)) in stereo_in.zip(stereo_out) {
            self.input[0] = *left_in;
            self.input[1] = *right_in;
            let output = g.sample(&self.input);
            *left_out = output[0];
            *right_out = output[1];
        }
    }
}

vst::plugin_main!(SoundGarden);
