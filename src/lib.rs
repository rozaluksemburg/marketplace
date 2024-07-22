extern crate leptos;
use leptos::*;
use std::fs::File;
use std::io::Write;

fn main() {
    render!(
        render_component(HelloWord),
    );
}
