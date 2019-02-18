use crate::context::Context;
use audio_graph::prelude::*;
use parking_lot::Mutex;
use sciter::{self, make_args, Element};
use std::os::raw::c_void;
use std::sync::Arc;
use vst;

const HTML: &[u8] = include_bytes!("./main.htm");

pub struct Editor {
    context: Arc<Mutex<Context>>,
    graph: Arc<Mutex<AudioGraph>>,
    frame: Option<sciter::window::Window>,
    is_open: Arc<Mutex<bool>>,
    text: Arc<Mutex<String>>,
}

impl Editor {
    pub fn new(context: Arc<Mutex<Context>>, graph: Arc<Mutex<AudioGraph>>) -> Self {
        Editor {
            context,
            graph,
            frame: None,
            is_open: Arc::new(Mutex::new(false)),
            text: Arc::new(Mutex::new("".to_string())),
        }
    }
}

struct EventHandler {
    context: Arc<Mutex<Context>>,
    graph: Arc<Mutex<AudioGraph>>,
    text: Arc<Mutex<String>>,
}

struct HostHandler {
    is_open: Arc<Mutex<bool>>,
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

impl EventHandler {
    fn graph_text_change(&mut self, root: &Element, text: String) {
        let context = self.context.lock();
        let channels = context.channels;
        let sample_rate = context.sample_rate;
        let mut nodes = Vec::new();
        let mut tokens = Vec::new();
        let mut g = AudioGraph::new(channels, context.parameters);
        for token in text.split_whitespace() {
            let node: Option<Node> = match token {
                "s" => Some(Box::new(Osc::new(channels, sample_rate, sine))),
                "sine" => Some(Box::new(OscPhase::new(channels, sample_rate, sine))),
                "t" => Some(Box::new(Osc::new(channels, sample_rate, triangle))),
                "tri" => Some(Box::new(OscPhase::new(channels, sample_rate, triangle))),
                "w" => Some(Box::new(Phasor::new(channels, sample_rate))),
                "saw" => Some(Box::new(Phasor0::new(channels, sample_rate))),
                "p" | "pulse" => Some(Box::new(Pulse::new(channels, sample_rate))),
                "+" => Some(Box::new(Fn2::new(channels, add))),
                "-" => Some(Box::new(Fn2::new(channels, sub))),
                "*" => Some(Box::new(Fn2::new(channels, mul))),
                "/" => Some(Box::new(Fn2::new(channels, div))),
                "\\" => Some(Box::new(Fn1::new(channels, recip))),
                "^" | "pow" => Some(Box::new(Fn2::new(channels, pow))),
                "unit" => Some(Box::new(Fn1::new(channels, unit))),
                "r" | "range" => Some(Box::new(Fn3::new(channels, range))),
                "n" | "noise" => Some(Box::new(Noise::new(channels))),
                "delay" => Some(Box::new(Delay::new(channels, sample_rate, 60.0))),
                "fb" | "feedback" => Some(Box::new(Feedback::new(channels, sample_rate, 60.0))),
                "lpf" => Some(Box::new(LPF::new(channels, sample_rate))),
                "hpf" => Some(Box::new(HPF::new(channels, sample_rate))),
                "l" | "bqlpf" => Some(Box::new(BiQuad::new(
                    channels,
                    sample_rate,
                    make_lpf_coefficients,
                ))),
                "h" | "bqhpf" => Some(Box::new(BiQuad::new(
                    channels,
                    sample_rate,
                    make_hpf_coefficients,
                ))),
                "m2f" | "midi2freq" => Some(Box::new(Fn1::new(channels, midi2freq))),
                "round" => Some(Box::new(Fn1::new(channels, round))),
                "quantize" => Some(Box::new(Fn2::new(channels, quantize))),
                "sin" => Some(Box::new(Fn1::new(channels, sin))),
                "cos" => Some(Box::new(Fn1::new(channels, cos))),
                "pan" => Some(Box::new(Pan3::new(channels))),
                "pan1" => Some(Box::new(Pan1::new(channels))),
                "pan2" => Some(Box::new(Pan2::new(channels))),
                "in" | "input" => Some(Box::new(Input::new(channels))),
                "cheb2" => Some(Box::new(Fn1::new(channels, cheb2))),
                "cheb3" => Some(Box::new(Fn1::new(channels, cheb3))),
                "cheb4" => Some(Box::new(Fn1::new(channels, cheb4))),
                "cheb5" => Some(Box::new(Fn1::new(channels, cheb5))),
                "cheb6" => Some(Box::new(Fn1::new(channels, cheb6))),
                "sh" | "sample&hold" => Some(Box::new(SampleAndHold::new(channels))),
                "m" | "metro" => Some(Box::new(Metro::new(channels, sample_rate))),
                "dm" | "dmetro" => Some(Box::new(DMetro::new(channels, sample_rate))),
                "mh" | "metroHold" => Some(Box::new(MetroHold::new(channels, sample_rate))),
                "dmh" | "dmetroHold" => Some(Box::new(DMetroHold::new(channels, sample_rate))),
                "yin" | "pitch" => Some(Box::new(Yin::new(channels, sample_rate, 1024, 512, 0.2))),
                "zip" => Some(Box::new(Zip::new(channels))),
                _ => match token.parse::<Sample>() {
                    Ok(x) => Some(Box::new(Constant::new(channels, x))),
                    Err(_) => {
                        let subcmd = token.split(':').collect::<Vec<_>>();
                        match subcmd[0] {
                            "param" => match subcmd.get(1) {
                                Some(x) => match x.parse::<usize>() {
                                    Ok(index) => Some(Box::new(Parameter::new(channels, index))),
                                    Err(_) => None,
                                },
                                None => None,
                            },
                            _ => None,
                        }
                    }
                },
            };
            nodes.push(node.and_then(|node| Some(g.add_node(node))));
            tokens.push(token);
        }
        let mut stack = Vec::new();
        for (i, (idx, token)) in nodes.into_iter().zip(&tokens).enumerate() {
            match idx {
                Some(idx) => {
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
                    for _ in 0..inputs {
                        sources.push(stack.pop().unwrap());
                    }
                    g.set_sources_rev(idx, &sources);
                    stack.push(idx);
                }
                None => match *token {
                    "pop" => {
                        if stack.is_empty() {
                            report_error(root, &format!("Nothing to pop at #{}!", i + 1));
                            return;
                        }
                        stack.pop();
                    }
                    "swap" => {
                        let len = stack.len();
                        if len < 2 {
                            report_error(root, &format!("Nothing to swap at #{}!", i + 1));
                            return;
                        }
                        stack.swap(len - 2, len - 1);
                    }
                    "dup" => match stack.last() {
                        Some(idx) => {
                            stack.push(*idx);
                        }
                        None => {
                            report_error(root, &format!("Nothing to dup at #{}!", i + 1));
                            return;
                        }
                    },
                    "rot" => {
                        let len = stack.len();
                        if len < 3 {
                            report_error(root, &format!("Nothing to rot at #{}!", i + 1));
                            return;
                        }
                        stack.swap(len - 2, len - 1);
                        stack.swap(len - 3, len - 1);
                    }
                    _ => {
                        report_error(
                            root,
                            &format!("Node #{} `{}` is unknown module.", i + 1, token),
                        );
                        return;
                    }
                },
            }
        }
        report_error(root, "");
        *self.text.lock() = text;
        *self.graph.lock() = g
    }
}

impl sciter::EventHandler for EventHandler {
    dispatch_script_call! {
        fn graph_text_change(String);
    }
}

impl sciter::HostHandler for HostHandler {
    fn on_engine_destroyed(&mut self) {
        *self.is_open.lock() = false;
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
        let mut frame = sciter::Window::create(
            (0, 0, 400, 800),
            sciter::types::SCITER_CREATE_WINDOW_FLAGS::SW_TITLEBAR
                | sciter::types::SCITER_CREATE_WINDOW_FLAGS::SW_RESIZEABLE,
            None,
        );
        let event_handler = EventHandler {
            context: self.context.clone(),
            graph: self.graph.clone(),
            text: self.text.clone(),
        };
        frame.event_handler(event_handler);
        let host_handler = HostHandler {
            is_open: self.is_open.clone(),
        };
        frame.sciter_handler(host_handler);
        frame.load_html(HTML, None);
        frame.expand(false);
        if let Ok(root) = Element::from_window(frame.get_hwnd()) {
            set_editor_text(&root, &self.text.lock());
        }
        self.frame = Some(frame);
        *self.is_open.lock() = true;
    }

    fn is_open(&mut self) -> bool {
        *self.is_open.lock()
    }

    fn close(&mut self) {
        if !self.is_open() {
            return;
        }
        let frame = self.frame.take();
        frame.unwrap().dismiss();
        *self.is_open.lock() = false;
    }
}
