use crate::context::Context;
use audio_graph::prelude::*;
use parking_lot::Mutex;
use sciter::{make_args, Element, EventHandler};
use std::os::raw::c_void;
use std::sync::Arc;
use vst;

const HTML: &[u8] = include_bytes!("./main.htm");

pub struct Editor {
    context: Arc<Mutex<Context>>,
    graph: Arc<Mutex<AudioGraph>>,
    frame: Option<sciter::window::Window>,
    text: Arc<Mutex<String>>,
}

impl Editor {
    pub fn new(context: Arc<Mutex<Context>>, graph: Arc<Mutex<AudioGraph>>) -> Self {
        Editor {
            context,
            graph,
            frame: None,
            text: Arc::new(Mutex::new("".to_string())),
        }
    }
}

struct Handler {
    context: Arc<Mutex<Context>>,
    graph: Arc<Mutex<AudioGraph>>,
    text: Arc<Mutex<String>>,
}

fn report_error(root: &Element, msg: &str) {
    if root
        .call_function("Error.report", &make_args!(msg))
        .is_err()
    {
        println!("Failed to call Error.report");
    };
}

fn set_editor_text(root: &Element, text: &str) {
    if root
        .call_function("Editor.set_text", &make_args!(text))
        .is_err()
    {
        println!("Failed to call Error.report");
    };
}

impl Handler {
    fn graph_text_change(&mut self, root: &Element, text: String) {
        let context = self.context.lock();
        let channels = context.channels;
        let sample_rate = context.sample_rate;
        let mut nodes = Vec::new();
        let mut tokens = Vec::new();
        let mut g = AudioGraph::new(channels);
        for (i, token) in text.split_whitespace().enumerate() {
            let node: Node = match token {
                "s" => Box::new(Osc::new(channels, sample_rate, sine)),
                "sine" => Box::new(OscPhase::new(channels, sample_rate, sine)),
                "t" => Box::new(Osc::new(channels, sample_rate, triangle)),
                "tri" => Box::new(OscPhase::new(channels, sample_rate, triangle)),
                "w" => Box::new(Phasor::new(channels, sample_rate)),
                "saw" => Box::new(Phasor0::new(channels, sample_rate)),
                "p" | "pulse" => Box::new(Pulse::new(channels, sample_rate)),
                "+" => Box::new(Fn2::new(channels, add)),
                "-" => Box::new(Fn2::new(channels, sub)),
                "*" => Box::new(Fn2::new(channels, mul)),
                "/" => Box::new(Fn2::new(channels, div)),
                "unit" => Box::new(Fn1::new(channels, unit)),
                "r" | "range" => Box::new(Fn3::new(channels, range)),
                "n" | "noise" => Box::new(Noise::new(channels)),
                "delay" => Box::new(Delay::new(channels, sample_rate, 60.0)),
                "fb" | "feedback" => Box::new(Feedback::new(channels, sample_rate, 60.0)),
                "lpf" => Box::new(LPF::new(channels, sample_rate)),
                "hpf" => Box::new(HPF::new(channels, sample_rate)),
                "l" | "bqlpf" => {
                    Box::new(BiQuad::new(channels, sample_rate, make_lpf_coefficients))
                }
                "h" | "bqhpf" => {
                    Box::new(BiQuad::new(channels, sample_rate, make_hpf_coefficients))
                }
                "m2f" | "midi2freq" => Box::new(Fn1::new(channels, midi2freq)),
                "round" => Box::new(Fn1::new(channels, round)),
                "sin" => Box::new(Fn1::new(channels, sin)),
                "pan" => Box::new(Pan3::new(channels)),
                "pan1" => Box::new(Pan1::new(channels)),
                "pan2" => Box::new(Pan2::new(channels)),
                "in" | "input" => Box::new(Input::new(channels)),
                _ => match token.parse::<Sample>() {
                    Ok(x) => Box::new(Constant::new(channels, x)),
                    Err(_) => {
                        report_error(
                            root,
                            &format!("Node #{} `{}` is unknown module.", i + 1, token),
                        );
                        return;
                    }
                },
            };
            nodes.push(g.add_node(node));
            tokens.push(token);
        }
        let mut new_text = Vec::new();
        let mut stack = Vec::new();
        let mut widths = Vec::new();
        let space = " ";
        for (i, (idx, token)) in nodes.into_iter().zip(&tokens).enumerate() {
            let inputs = g.node(idx).inputs();
            if stack.len() < (inputs as usize) {
                report_error(
                    root,
                    &format!(
                        "Node #{} `{}` has not enough inputs on the stack.",
                        i + 1,
                        token
                    ),
                );
                return;
            }
            let mut sources = Vec::new();
            let mut width = 0;
            for _ in 0..inputs {
                sources.push(stack.pop().unwrap());
                width = width.max(widths.pop().unwrap());
            }
            g.set_sources_rev(idx, &sources);
            stack.push(idx);
            widths.push(width + token.len());
            new_text.push(format!("{}{}", space.repeat(width), token));
        }
        report_error(root, "");
        // TODO to autoformat or not to autoformat?
        let text = new_text.join("\n");
        // set_editor_text(root, &text);
        *self.text.lock() = text;
        *self.graph.lock() = g
    }
}

impl EventHandler for Handler {
    dispatch_script_call! {
        fn graph_text_change(String);
    }
}

impl vst::editor::Editor for Editor {
    fn size(&self) -> (i32, i32) {
        (0, 0)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, _window: *mut c_void) {
        if self.is_open() {
            self.frame.as_ref().unwrap().expand(false);
            return;
        }
        // let hwnd = window as sciter::types::HWINDOW;
        let mut frame = sciter::Window::create(
            (0, 0, 400, 800),
            sciter::types::SCITER_CREATE_WINDOW_FLAGS::SW_TITLEBAR
                | sciter::types::SCITER_CREATE_WINDOW_FLAGS::SW_RESIZEABLE,
            // Some(hwnd),
            None,
        );
        let handler = Handler {
            context: self.context.clone(),
            graph: self.graph.clone(),
            text: self.text.clone(),
        };
        frame.event_handler(handler);
        frame.load_html(HTML, None);
        frame.expand(false);
        if let Ok(root) = Element::from_window(frame.get_hwnd()) {
            set_editor_text(&root, &self.text.lock());
        }
        self.frame = Some(frame);
    }

    fn is_open(&mut self) -> bool {
        self.frame.is_some()
    }

    fn close(&mut self) {
        if !self.is_open() {
            return;
        }
        let frame = self.frame.take();
        frame.unwrap().dismiss();
    }
}
