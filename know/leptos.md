Introduction
This book is intended as an introduction to the Leptos Web framework. It will walk through the fundamental concepts you need to build applications, beginning with a simple application rendered in the browser, and building toward a full-stack application with server-side rendering and hydration.

The guide doesn’t assume you know anything about fine-grained reactivity or the details of modern Web frameworks. It does assume you are familiar with the Rust programming language, HTML, CSS, and the DOM and basic Web APIs.

Leptos is most similar to frameworks like Solid (JavaScript) and Sycamore (Rust). There are some similarities to other frameworks like React (JavaScript), Svelte (JavaScript), Yew (Rust), and Dioxus (Rust), so knowledge of one of those frameworks may also make it easier to understand Leptos.

You can find more detailed docs for each part of the API at Docs.rs.

The source code for the book is available here. PRs for typos or clarification are always welcome.


Getting Started
There are two basic paths to getting started with Leptos:

Client-side rendering (CSR) with Trunk - a great option if you just want to make a snappy website with Leptos, or work with a pre-existing server or API. In CSR mode, Trunk compiles your Leptos app to WebAssembly (WASM) and runs it in the browser like a typical Javascript single-page app (SPA). The advantages of Leptos CSR include faster build times and a quicker iterative development cycle, as well as a simpler mental model and more options for deploying your app. CSR apps do come with some disadvantages: initial load times for your end users are slower compared to a server-side rendering approach, and the usual SEO challenges that come along with using a JS single-page app model apply to Leptos CSR apps as well. Also note that, under the hood, an auto-generated snippet of JS is used to load the Leptos WASM bundle, so JS must be enabled on the client device for your CSR app to display properly. As with all software engineering, there are trade-offs here you'll need to consider.

Full-stack, server-side rendering (SSR) with cargo-leptos - SSR is a great option for building CRUD-style websites and custom web apps if you want Rust powering both your frontend and backend. With the Leptos SSR option, your app is rendered to HTML on the server and sent down to the browser; then, WebAssembly is used to instrument the HTML so your app becomes interactive - this process is called 'hydration'. On the server side, Leptos SSR apps integrate closely with your choice of either Actix-web or Axum server libraries, so you can leverage those communities' crates to help build out your Leptos server. The advantages of taking the SSR route with Leptos include helping you get the best initial load times and optimal SEO scores for your web app. SSR apps can also dramatically simplify working across the server/client boundary via a Leptos feature called "server functions", which lets you transparently call functions on the server from your client code (more on this feature later). Full-stack SSR isn't all rainbows and butterflies, though - disadvantages include a slower developer iteration loop (because you need to recompile both the server and client when making Rust code changes), as well as some added complexity that comes along with hydration.

By the end of the book, you should have a good idea of which trade-offs to make and which route to take - CSR or SSR - depending on your project's requirements.

In Part 1 of this book, we'll start with client-side rendering Leptos sites and building reactive UIs using Trunk to serve our JS and WASM bundle to the browser.

We’ll introduce cargo-leptos in Part 2 of this book, which is all about working with the full power of Leptos in its full-stack, SSR mode.

Note

If you're coming from the Javascript world and terms like client-side rendering (CSR) and server-side rendering (SSR) are unfamiliar to you, the easiest way to understand the difference is by analogy:

Leptos' CSR mode is similar to working with React (or a 'signals'-based framework like SolidJS), and focuses on producing a client-side UI which you can use with any tech stack on the server.

Using Leptos' SSR mode is similar to working with a full-stack framework like Next.js in the React world (or Solid's "SolidStart" framework) - SSR helps you build sites and apps that are rendered on the server then sent down to the client. SSR can help to improve your site's loading performance and accessibility as well as make it easier for one person to work on both client- and server-side without needing to context-switch between different languages for frontend and backend.

The Leptos framework can be used either in CSR mode to just make a UI (like React), or you can use Leptos in full-stack SSR mode (like Next.js) so that you can build both your UI and your server with one language: Rust.

Hello World! Getting Set up for Leptos CSR Development
First up, make sure Rust is installed and up-to-date (see here if you need instructions).

If you don’t have it installed already, you can install the "Trunk" tool for running Leptos CSR sites by running the following on the command-line:

cargo install trunk
And then create a basic Rust project

cargo init leptos-tutorial
cd into your new leptos-tutorial project and add leptos as a dependency

cargo add leptos --features=csr,nightly
Or you can leave off nightly if you're using stable Rust

cargo add leptos --features=csr
Using nightly Rust, and the nightly feature in Leptos enables the function-call syntax for signal getters and setters that is used in most of this book.

To use nightly Rust, you can either opt into nightly for all your Rust projects by running

rustup toolchain install nightly
rustup default nightly
or only for this project

rustup toolchain install nightly
cd <into your project>
rustup override set nightly
See here for more details.

If you’d rather use stable Rust with Leptos, you can do that too. In the guide and examples, you’ll just use the ReadSignal::get() and WriteSignal::set() methods instead of calling signal getters and setters as functions.

Make sure you've added the wasm32-unknown-unknown target so that Rust can compile your code to WebAssembly to run in the browser.

rustup target add wasm32-unknown-unknown
Create a simple index.html in the root of the leptos-tutorial directory

<!DOCTYPE html>
<html>
  <head></head>
  <body></body>
</html>
And add a simple “Hello, world!” to your main.rs

use leptos::*;

fn main() {
mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
Your directory structure should now look something like this


leptos_tutorial
├── src
│   └── main.rs
├── Cargo.toml
├── index.html
Now run trunk serve --open from the root of the leptos-tutorial directory. Trunk should automatically compile your app and open it in your default browser. If you make edits to main.rs, Trunk will recompile your source code and live-reload the page.

Welcome to the world of UI development with Rust and WebAssembly (WASM), powered by Leptos and Trunk!

Note

If you are using Windows, note that trunk serve --open may not work. If you have issues with --open, simply use trunk serve and open a browser tab manually.

Now before we get started building your first real UI's with Leptos, there are a couple of things you might want to know to help make your experience with Leptos just a little bit easier.


Leptos Developer Experience Improvements
There are a couple of things you can do to improve your experience of developing websites and apps with Leptos. You may want to take a few minutes and set up your environment to optimize your development experience, especially if you want to code along with the examples in this book.

1) Set up console_error_panic_hook
   By default, panics that happen while running your WASM code in the browser just throw an error in the browser with an unhelpful message like Unreachable executed and a stack trace that points into your WASM binary.

With console_error_panic_hook, you get an actual Rust stack trace that includes a line in your Rust source code.

It's very easy to set up:

Run cargo add console_error_panic_hook in your project
In your main function, add console_error_panic_hook::set_once();
If this is unclear, click here for an example.

Now you should have much better panic messages in the browser console!

2) Editor Autocompletion inside #[component] and #[server]
   Because of the nature of macros (they can expand from anything to anything, but only if the input is exactly correct at that instant) it can be hard for rust-analyzer to do proper autocompletion and other support.

If you run into issues using these macros in your editor, you can explicitly tell rust-analyzer to ignore certain proc macros. For the #[server] macro especially, which annotates function bodies but doesn't actually transform anything inside the body of your function, this can be really helpful.

Starting in Leptos version 0.5.3, rust-analyzer support was added for the #[component] macro, but if you run into issues, you may want to add #[component] to the macro ignore list as well (see below). Note that this means that rust-analyzer doesn't know about your component props, which may generate its own set of errors or warnings in the IDE.

VSCode settings.json:

"rust-analyzer.procMacro.ignored": {
"leptos_macro": [
// optional:
// "component",
"server"
],
}
VSCode with cargo-leptos settings.json:

"rust-analyzer.procMacro.ignored": {
"leptos_macro": [
// optional:
// "component",
"server"
],
},
// if code that is cfg-gated for the `ssr` feature is shown as inactive,
// you may want to tell rust-analyzer to enable the `ssr` feature by default
//
// you can also use `rust-analyzer.cargo.allFeatures` to enable all features
"rust-analyzer.cargo.features": ["ssr"]
neovim with lspconfig:

require('lspconfig').rust_analyzer.setup {
-- Other Configs ...
settings = {
["rust-analyzer"] = {
-- Other Settings ...
procMacro = {
ignored = {
leptos_macro = {
-- optional: --
-- "component",
"server",
},
},
},
},
}
}
Helix, in .helix/languages.toml:

[[language]]
name = "rust"

[language-server.rust-analyzer]
config = { procMacro = { ignored = { leptos_macro = [
# Optional:
# "component",
"server"
] } } }
Zed, in settings.json:

{
-- Other Settings ...
"lsp": {
"rust-analyzer": {
"procMacro": {
"ignored": [
// optional:
// "component",
"server"
]
}
}
}
}
SublimeText 3, under LSP-rust-analyzer.sublime-settings in Goto Anything... menu:

// Settings in here override those in "LSP-rust-analyzer/LSP-rust-analyzer.sublime-settings"
{
"rust-analyzer.procMacro.ignored": {
"leptos_macro": [
// optional:
// "component",
"server"
],
},
}
3) Set up leptosfmt With Rust Analyzer (optional)
   leptosfmt is a formatter for the Leptos view! macro (inside of which you'll typically write your UI code). Because the view! macro enables an 'RSX' (like JSX) style of writing your UI's, cargo-fmt has a harder time auto-formatting your code that's inside the view! macro. leptosfmt is a crate that solves your formatting issues and keeps your RSX-style UI code looking nice and tidy!

leptosfmt can be installed and used via the command line or from within your code editor:

First, install the tool with cargo install leptosfmt.

If you just want to use the default options from the command line, just run leptosfmt ./**/*.rs from the root of your project to format all the rust files using leptosfmt.

If you wish to set up your editor to work with leptosfmt, or if you wish to customize your leptosfmt experience, please see the instructions available on the leptosfmt github repo's README.md page.

Just note that it's recommended to set up your editor with leptosfmt on a per-workspace basis for best results.


The Leptos Community and leptos-* Crates
The Community
One final note before we get to building with Leptos: if you haven't already, feel free to join the growing community on the Leptos Discord and on Github. Our Discord channel in particular is very active and friendly - we'd love to have you there!

Note

If you find a chapter or an explanation that isn't clear while you're working your way through the Leptos book, just mention it in the "docs-and-education" channel or ask a question in "help" so we can clear things up and update the book for others.

As you get further along in your Leptos journey and find that you have questions about "how to do 'x' with Leptos", then search the Discord "help" channel to see if a similar question has been asked before, or feel free to post your own question - the community is quite helpful and very responsive.

The "Discussions" on Github are also a great place for asking questions and keeping up with Leptos announcements.

And of course, if you run into any bugs while developing with Leptos or would like to make a feature request (or contribute a bug fix / new feature), open up an issue on the Github issue tracker.

Leptos-* Crates
The community has built a growing number of Leptos-related crates that will help you get productive with Leptos projects more quickly - check out the list of crates built on top of Leptos and contributed by the community on the Awesome Leptos repo on Github.

If you want to find the newest, up-and-coming Leptos-related crates, check out the "Tools and Libraries" section of the Leptos Discord. In that section, there are channels for the Leptos view! macro formatter (in the "leptosfmt" channel); there's a channel for the utility library "leptos-use"; another channel for the UI component libary "leptonic"; and a "libraries" channel where new leptos-* crates are discussed before making their way into the growing list of crates and resources available on Awesome Leptos.


Part 1: Building User Interfaces
In the first part of the book, we're going to look at building user interfaces on the client-side using Leptos. Under the hood, Leptos and Trunk are bundling up a snippet of Javascript which will load up the Leptos UI, which has been compiled to WebAssembly to drive the interactivity in your CSR (client-side rendered) website.

Part 1 will introduce you to the basic tools you need to build a reactive user interface powered by Leptos and Rust. By the end of Part 1, you should be able to build a snappy synchronous website that's rendered in the browser and which you can deploy on any static-site hosting service, like Github Pages or Vercel.

Info

To get the most out of this book, we encourage you to code along with the examples provided. In the Getting Started and Leptos DX chapters, we showed you how to set up a basic project with Leptos and Trunk, including WASM error handling in the browser. That basic setup is enough to get you started developing with Leptos.

If you'd prefer to get started using a more full-featured template which demonstrates how to set up a few of the basics you'd see in a real Leptos project, such as routing, (covered later in the book), injecting <Title> and <Meta> tags into the page head, and a few other niceties, then feel free to utilize the leptos-rs start-trunk template repo to get up and running.

The start-trunk template requires that you have Trunk and cargo-generate installed, which you can get by running cargo install trunk and cargo install cargo-generate.

To use the template to set up your project, just run

cargo generate --git https://github.com/leptos-community/start-csr

then run

trunk serve --port 3000 --open

in the newly created app's directory to start developing your app. The Trunk server will reload your app on file changes, making development relatively seamless.


A Basic Component
That “Hello, world!” was a very simple example. Let’s move on to something a little more like an ordinary app.

First, let’s edit the main function so that, instead of rendering the whole app, it just renders an <App/> component. Components are the basic unit of composition and design in most web frameworks, and Leptos is no exception. Conceptually, they are similar to HTML elements: they represent a section of the DOM, with self-contained, defined behavior. Unlike HTML elements, they are in PascalCase, so most Leptos applications will start with something like an <App/> component.

fn main() {
leptos::mount_to_body(|| view! { <App/> })
}
Now let’s define our <App/> component itself. Because it’s relatively simple, I’ll give you the whole thing up front, then walk through it line by line.

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count(3);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || count()}
        </button>
    }
}
The Component Signature
#[component]
Like all component definitions, this begins with the #[component] macro. #[component] annotates a function so it can be used as a component in your Leptos application. We’ll see some of the other features of this macro in a couple chapters.

fn App() -> impl IntoView
Every component is a function with the following characteristics

It takes zero or more arguments of any type.
It returns impl IntoView, which is an opaque type that includes anything you could return from a Leptos view.
Component function arguments are gathered together into a single props struct which is built by the view macro as needed.

The Component Body
The body of the component function is a set-up function that runs once, not a render function that reruns multiple times. You’ll typically use it to create a few reactive variables, define any side effects that run in response to those values changing, and describe the user interface.

let (count, set_count) = create_signal(0);
create_signal creates a signal, the basic unit of reactive change and state management in Leptos. This returns a (getter, setter) tuple. To access the current value, you’ll use count.get() (or, on nightly Rust, the shorthand count()). To set the current value, you’ll call set_count.set(...) (or set_count(...)).

.get() clones the value and .set() overwrites it. In many cases, it’s more efficient to use .with() or .update(); check out the docs for ReadSignal and WriteSignal if you’d like to learn more about those trade-offs at this point.

The View
Leptos defines user interfaces using a JSX-like format via the view macro.

view! {
<button
// define an event listener with on:
on:click=move |_| {
set_count(3);
}
>
// text nodes are wrapped in quotation marks
"Click me: "
// blocks can include Rust code
{move || count()}
</button>
}
This should mostly be easy to understand: it looks like HTML, with a special on:click to define a click event listener, a text node that’s formatted like a Rust string, and then...

{move || count()}
whatever that is.

People sometimes joke that they use more closures in their first Leptos application than they’ve ever used in their lives. And fair enough. Basically, passing a function into the view tells the framework: “Hey, this is something that might change.”

When we click the button and call set_count, the count signal is updated. This move || count() closure, whose value depends on the value of count, reruns, and the framework makes a targeted update to that one specific text node, touching nothing else in your application. This is what allows for extremely efficient updates to the DOM.

Now, if you have Clippy on—or if you have a particularly sharp eye—you might notice that this closure is redundant, at least if you’re in nightly Rust. If you’re using Leptos with nightly Rust, signals are already functions, so the closure is unnecessary. As a result, you can write a simpler view:

view! {
<button /* ... */>
"Click me: "
// identical to {move || count()}
{count}
</button>
}
Remember—and this is very important—only functions are reactive. This means that {count} and {count()} do very different things in your view. {count} passes in a function, telling the framework to update the view every time count changes. {count()} accesses the value of count once, and passes an i32 into the view, rendering it once, unreactively. You can see the difference in the CodeSandbox below!

Let’s make one final change. set_count(3) is a pretty useless thing for a click handler to do. Let’s replace “set this value to 3” with “increment this value by 1”:

move |_| {
set_count.update(|n| *n += 1);
}
You can see here that while set_count just sets the value, set_count.update() gives us a mutable reference and mutates the value in place. Either one will trigger a reactive update in our UI.

Throughout this tutorial, we’ll use CodeSandbox to show interactive examples. To show the browser in the sandbox, you may need to click Add DevTools > Other Previews > 8080. Hover over any of the variables to show Rust-Analyzer details and docs for what’s going on. Feel free to fork the examples to play with them yourself!


view: Dynamic Classes, Styles and Attributes
So far we’ve seen how to use the view macro to create event listeners and to create dynamic text by passing a function (such as a signal) into the view.

But of course there are other things you might want to update in your user interface. In this section, we’ll look at how to update classes, styles and attributes dynamically, and we’ll introduce the concept of a derived signal.

Let’s start with a simple component that should be familiar: click a button to increment a counter.

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count()}
        </button>
    }
}
So far, this is just the example from the last chapter.

Dynamic Classes
Now let’s say I’d like to update the list of CSS classes on this element dynamically. For example, let’s say I want to add the class red when the count is odd. I can do this using the class: syntax.

class:red=move || count() % 2 == 1
class: attributes take

the class name, following the colon (red)
a value, which can be a bool or a function that returns a bool
When the value is true, the class is added. When the value is false, the class is removed. And if the value is a function that accesses a signal, the class will reactively update when the signal changes.

Now every time I click the button, the text should toggle between red and black as the number switches between even and odd.

<button
on:click=move |_| {
set_count.update(|n| *n += 1);
}
// the class: syntax reactively updates a single class
// here, we'll set the `red` class when `count` is odd
class:red=move || count() % 2 == 1
>
    "Click me"
</button>
If you’re following along, make sure you go into your index.html and add something like this:

<style>
  .red {
    color: red;
  }
</style>
Some CSS class names can’t be directly parsed by the view macro, especially if they include a mix of dashes and numbers or other characters. In that case, you can use a tuple syntax: class=("name", value) still directly updates a single class.

class=("button-20", move || count() % 2 == 1)
Dynamic Styles
Individual CSS properties can be directly updated with a similar style: syntax.

    let (x, set_x) = create_signal(0);
        view! {
            <button
                on:click={move |_| {
                    set_x.update(|n| *n += 10);
                }}
                // set the `style` attribute
                style="position: absolute"
                // and toggle individual CSS properties with `style:`
                style:left=move || format!("{}px", x() + 100)
                style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
                style:max-width="400px"
                // Set a CSS variable for stylesheet use
                style=("--columns", x)
            >
                "Click to Move"
            </button>
    }
Dynamic Attributes
The same applies to plain attributes. Passing a plain string or primitive value to an attribute gives it a static value. Passing a function (including a signal) to an attribute causes it to update its value reactively. Let’s add another element to our view:

<progress
max="50"
// signals are functions, so `value=count` and `value=move || count.get()`
// are interchangeable.
value=count
/>
Now every time we set the count, not only will the class of the <button> be toggled, but the value of the <progress> bar will increase, which means that our progress bar will move forward.

Derived Signals
Let’s go one layer deeper, just for fun.

You already know that we create reactive interfaces just by passing functions into the view. This means that we can easily change our progress bar. For example, suppose we want it to move twice as fast:

<progress
max="50"
value=move || count() * 2
/>
But imagine we want to reuse that calculation in more than one place. You can do this using a derived signal: a closure that accesses a signal.

let double_count = move || count() * 2;

/* insert the rest of the view */
<progress
max="50"
// we use it once here
value=double_count
/>
<p>
    "Double Count: "
    // and again here
    {double_count}
</p>
Derived signals let you create reactive computed values that can be used in multiple places in your application with minimal overhead.

Note: Using a derived signal like this means that the calculation runs once per signal change (when count() changes) and once per place we access double_count; in other words, twice. This is a very cheap calculation, so that’s fine. We’ll look at memos in a later chapter, which were designed to solve this problem for expensive calculations.

Advanced Topic: Injecting Raw HTML
The view macro provides support for an additional attribute, inner_html, which can be used to directly set the HTML contents of any element, wiping out any other children you’ve given it. Note that this does not escape the HTML you provide. You should make sure that it only contains trusted input or that any HTML entities are escaped, to prevent cross-site scripting (XSS) attacks.

let html = "<p>This HTML will be injected.</p>";
view! {
  <div inner_html=html/>
}

full view macros docs

Macro leptos::viewCopy item path
source · [−]
view!() { /* proc-macro */ }
The view macro uses RSX (like JSX, but Rust!) It follows most of the same rules as HTML, with the following differences:

Text content should be provided as a Rust string, i.e., double-quoted:
view! { <p>"Here’s some text"</p> };
Self-closing tags need an explicit / as in XML/XHTML
ⓘ
// ❌ not like this
view! { <input type="text" name="name"> }
// ✅ add that slash
view! { <input type="text" name="name" /> }
Components (functions annotated with #[component]) can be inserted as camel-cased tags. (Generics on components are specified as <Component<T>/>, not the turbofish <Component::<T>/>.)
view! { <div><Counter initial_value=3 /></div> }
Dynamic content can be wrapped in curly braces ({ }) to insert text nodes, elements, or set attributes. If you insert a signal here, Leptos will create an effect to update the DOM whenever the value changes. (“Signal” here means Fn() -> T where T is the appropriate type for that node: a String in case of text nodes, a bool for class: attributes, etc.)

Attributes can take a wide variety of primitive types that can be converted to strings. They can also take an Option, in which case Some sets the attribute and None removes the attribute.

ⓘ
let (count, set_count) = create_signal(0);

view! {
// ❌ not like this: `count.get()` returns an `i32`, not a function
  <p>{count.get()}</p>
  // ✅ this is good: Leptos sees the function and knows it's a dynamic value
  <p>{move || count.get()}</p>
  // 🔥 with the `nightly` feature, `count` is a function, so `count` itself can be passed directly into the view
  <p>{count}</p>
}
Event handlers can be added with on: attributes. In most cases, the events are given the correct type based on the event name.
view! {
  <button on:click=|ev| {
    log::debug!("click event: {ev:#?}");
  }>
    "Click me"
  </button>
}
DOM properties can be set with prop: attributes, which take any primitive type or JsValue (or a signal that returns a primitive or JsValue). They can also take an Option, in which case Some sets the property and None deletes the property.
let (name, set_name) = create_signal("Alice".to_string());

view! {
<input
type="text"
name="user_name"
value={move || name.get()} // this only sets the default value!
prop:value={move || name.get()} // here's how you update values. Sorry, I didn’t invent the DOM.
on:click=move |ev| set_name.set(event_target_value(&ev)) // `event_target_value` is a useful little Leptos helper
/>
}
Classes can be toggled with class: attributes, which take a bool (or a signal that returns a bool).
let (count, set_count) = create_signal(2);
view! { <div class:hidden-div={move || count.get() < 3}>"Now you see me, now you don’t."</div> }
Class names can include dashes, and since v0.5.0 can include a dash-separated segment of only numbers.

let (count, set_count) = create_signal(2);
view! { <div class:hidden-div-25={move || count.get() < 3}>"Now you see me, now you don’t."</div> }
Class names cannot include special symbols.

ⓘ
let (count, set_count) = create_signal(2);
// class:hidden-[div]-25 is invalid attribute name
view! { <div class:hidden-[div]-25={move || count.get() < 3}>"Now you see me, now you don’t."</div> }
However, you can pass arbitrary class names using the syntax class=("name", value).

let (count, set_count) = create_signal(2);
// this allows you to use CSS frameworks that include complex class names
view! {
  <div
    class=("is-[this_-_really]-necessary-42", move || count.get() < 3)
  >
    "Now you see me, now you don’t."
  </div>
}
Individual styles can also be set with style: or style=("property-name", value) syntax.
let (x, set_x) = create_signal(0);
let (y, set_y) = create_signal(0);
view! {
  <div
    style="position: absolute"
    style:left=move || format!("{}px", x.get())
    style:top=move || format!("{}px", y.get())
    style=("background-color", move || format!("rgb({}, {}, 100)", x.get(), y.get()))
  >
    "Moves when coordinates change"
  </div>
}
You can use the node_ref or _ref attribute to store a reference to its DOM element in a NodeRef to use later.
use leptos::html::Input;

let (value, set_value) = create_signal(0);
let my_input = create_node_ref::<Input>();
view! { <input type="text" _ref=my_input/> }
// `my_input` now contains an `Element` that we can use anywhere
You can add the same class to every element in the view by passing in a special class = {/* ... */}, argument after ``. This is useful for injecting a class provided by a scoped styling library.
let class = "mycustomclass";
view! { class = class,
  <div> // will have class="mycustomclass"
    <p>"Some text"</p> // will also have class "mycustomclass"
  </div>
}
You can set any HTML element’s innerHTML with the inner_html attribute on an element. Be careful: this HTML will not be escaped, so you should ensure that it only contains trusted input.
let html = "<p>This HTML will be injected.</p>";
view! {
  <div inner_html=html/>
}
Here’s a simple example that shows off several of these features, put together


pub fn SimpleCounter() -> impl IntoView {
// create a reactive signal with the initial value
let (value, set_value) = create_signal(0);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_ev| set_value.set(0);
    let decrement = move |_ev| set_value.update(|value| *value -= 1);
    let increment = move |_ev| set_value.update(|value| *value += 1);

    view! {
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {move || value.get().to_string()} "!"</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}

+ one more code

use leptos::*;

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);

    // a "derived signal" is a function that accesses other signals
    // we can use this to create reactive values that depend on the
    // values of one or more other signals
    let double_count = move || count() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            // the class: syntax reactively updates a single class
            // here, we'll set the `red` class when `count` is odd
            class:red=move || count() % 2 == 1
        >
            "Click me"
        </button>
        // NOTE: self-closing tags like <br> need an explicit /
        <br/>

        // We'll update this progress bar every time `count` changes
        <progress
            // static attributes work as in HTML
            max="50"

            // passing a function to an attribute
            // reactively sets that attribute
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            value=count
        >
        </progress>
        <br/>

        // This progress bar will use `double_count`
        // so it should move twice as fast!
        <progress
            max="50"
            // derived signals are functions, so they can also
            // reactively update the DOM
            value=double_count
        >
        </progress>
        <p>"Count: " {count}</p>
        <p>"Double Count: " {double_count}</p>
    }
}

fn main() {
leptos::mount_to_body(App)
}




Components and Props
So far, we’ve been building our whole application in a single component. This is fine for really tiny examples, but in any real application you’ll need to break the user interface out into multiple components, so you can break your interface down into smaller, reusable, composable chunks.

Let’s take our progress bar example. Imagine that you want two progress bars instead of one: one that advances one tick per click, one that advances two ticks per click.

You could do this by just creating two <progress> elements:

let (count, set_count) = create_signal(0);
let double_count = move || count() * 2;

view! {
<progress
max="50"
value=count
/>
<progress
max="50"
value=double_count
/>
}
But of course, this doesn’t scale very well. If you want to add a third progress bar, you need to add this code another time. And if you want to edit anything about it, you need to edit it in triplicate.

Instead, let’s create a <ProgressBar/> component.

#[component]
fn ProgressBar() -> impl IntoView {
view! {
<progress
max="50"
// hmm... where will we get this from?
value=progress
/>
}
}
There’s just one problem: progress is not defined. Where should it come from? When we were defining everything manually, we just used the local variable names. Now we need some way to pass an argument into the component.

Component Props
We do this using component properties, or “props.” If you’ve used another frontend framework, this is probably a familiar idea. Basically, properties are to components as attributes are to HTML elements: they let you pass additional information into the component.

In Leptos, you define props by giving additional arguments to the component function.

#[component]
fn ProgressBar(
progress: ReadSignal<i32>
) -> impl IntoView {
view! {
<progress
max="50"
// now this works
value=progress
/>
}
}
Now we can use our component in the main <App/> component’s view.

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);
view! {
<button on:click=move |_| { set_count.update(|n| *n += 1); }>
"Click me"
</button>
// now we use our component!
<ProgressBar progress=count/>
}
}
Using a component in the view looks a lot like using an HTML element. You’ll notice that you can easily tell the difference between an element and a component because components always have PascalCase names. You pass the progress prop in as if it were an HTML element attribute. Simple.

Reactive and Static Props
You’ll notice that throughout this example, progress takes a reactive ReadSignal<i32>, and not a plain i32. This is very important.

Component props have no special meaning attached to them. A component is simply a function that runs once to set up the user interface. The only way to tell the interface to respond to changes is to pass it a signal type. So if you have a component property that will change over time, like our progress, it should be a signal.

optional Props
Right now the max setting is hard-coded. Let’s take that as a prop too. But let’s add a catch: let’s make this prop optional by annotating the particular argument to the component function with #[prop(optional)].

#[component]
fn ProgressBar(
// mark this prop optional
// you can specify it or not when you use <ProgressBar/>
#[prop(optional)]
max: u16,
progress: ReadSignal<i32>
) -> impl IntoView {
view! {
<progress
max=max
value=progress
/>
}
}
Now, we can use <ProgressBar max=50 progress=count/>, or we can omit max to use the default value (i.e., <ProgressBar progress=count/>). The default value on an optional is its Default::default() value, which for a u16 is going to be 0. In the case of a progress bar, a max value of 0 is not very useful.

So let’s give it a particular default value instead.

default props
You can specify a default value other than Default::default() pretty simply with #[prop(default = ...).

#[component]
fn ProgressBar(
#[prop(default = 100)]
max: u16,
progress: ReadSignal<i32>
) -> impl IntoView {
view! {
<progress
max=max
value=progress
/>
}
}
Generic Props
This is great. But we began with two counters, one driven by count, and one by the derived signal double_count. Let’s recreate that by using double_count as the progress prop on another <ProgressBar/>.

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);
let double_count = move || count() * 2;

    view! {
        <button on:click=move |_| { set_count.update(|n| *n += 1); }>
            "Click me"
        </button>
        <ProgressBar progress=count/>
        // add a second progress bar
        <ProgressBar progress=double_count/>
    }
}
Hm... this won’t compile. It should be pretty easy to understand why: we’ve declared that the progress prop takes ReadSignal<i32>, and double_count is not ReadSignal<i32>. As rust-analyzer will tell you, its type is || -> i32, i.e., it’s a closure that returns an i32.

There are a couple ways to handle this. One would be to say: “Well, I know that a ReadSignal is a function, and I know that a closure is a function; maybe I could just take any function?” If you’re savvy, you may know that both these implement the trait Fn() -> i32. So you could use a generic component:

#[component]
fn ProgressBar<F>(
#[prop(default = 100)]
max: u16,
progress: F
) -> impl IntoView
where
F: Fn() -> i32 + 'static,
{
view! {
<progress
max=max
value=progress
/>
}
}
This is a perfectly reasonable way to write this component: progress now takes any value that implements this Fn() trait.

This generic can also be specified inline:

#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
#[prop(default = 100)] max: u16,
progress: F,
) -> impl IntoView {
view! {
<progress
max=max
value=progress
/>
}
}
Note that generic component props can’t be specified with an impl yet (progress: impl Fn() -> i32 + 'static,), in part because they’re actually used to generate a struct ProgressBarProps, and struct fields cannot be impl types. The #[component] macro may be further improved in the future to allow inline impl generic props.

Generics need to be used somewhere in the component props. This is because props are built into a struct, so all generic types must be used somewhere in the struct. This is often easily accomplished using an optional PhantomData prop. You can then specify a generic in the view using the syntax for expressing types: <Component<T>/> (not with the turbofish-style <Component::<T>/>).

#[component]
fn SizeOf<T: Sized>(#[prop(optional)] _ty: PhantomData<T>) -> impl IntoView {
std::mem::size_of::<T>()
}

#[component]
pub fn App() -> impl IntoView {
view! {
<SizeOf<usize>/>
<SizeOf<String>/>
}
}
Note that there are some limitations. For example, our view macro parser can’t handle nested generics like <SizeOf<Vec<T>>/>.

into Props
There’s one more way we could implement this, and it would be to use #[prop(into)]. This attribute automatically calls .into() on the values you pass as props, which allows you to easily pass props with different values.

In this case, it’s helpful to know about the Signal type. Signal is an enumerated type that represents any kind of readable reactive signal. It can be useful when defining APIs for components you’ll want to reuse while passing different sorts of signals. The MaybeSignal type is useful when you want to be able to take either a static or reactive value.

#[component]
fn ProgressBar(
#[prop(default = 100)]
max: u16,
#[prop(into)]
progress: Signal<i32>
) -> impl IntoView
{
view! {
<progress
max=max
value=progress
/>
}
}

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);
let double_count = move || count() * 2;

    view! {
        <button on:click=move |_| { set_count.update(|n| *n += 1); }>
            "Click me"
        </button>
        // .into() converts `ReadSignal` to `Signal`
        <ProgressBar progress=count/>
        // use `Signal::derive()` to wrap a derived signal
        <ProgressBar progress=Signal::derive(double_count)/>
    }
}
Optional Generic Props
Note that you can’t specify optional generic props for a component. Let’s see what would happen if you try:

#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
#[prop(optional)] progress: Option<F>,
) -> impl IntoView {
progress.map(|progress| {
view! {
<progress
max=100
value=progress
/>
}
})
}

#[component]
pub fn App() -> impl IntoView {
view! {
<ProgressBar/>
}
}
Rust helpfully gives the error

xx |         <ProgressBar/>
|          ^^^^^^^^^^^ cannot infer type of the type parameter `F` declared on the function `ProgressBar`
|
help: consider specifying the generic argument
|
xx |         <ProgressBar::<F>/>
|                     +++++
You can specify generics on components with a <ProgressBar<F>/> syntax (no turbofish in the view macro). Specifying the correct type here is not possible; closures and functions in general are unnameable types. The compiler can display them with a shorthand, but you can’t specify them.

However, you can get around this by providing a concrete type using Box<dyn _> or &dyn _:

#[component]
fn ProgressBar(
#[prop(optional)] progress: Option<Box<dyn Fn() -> i32>>,
) -> impl IntoView {
progress.map(|progress| {
view! {
<progress
max=100
value=progress
/>
}
})
}

#[component]
pub fn App() -> impl IntoView {
view! {
<ProgressBar/>
}
}
Because the Rust compiler now knows the concrete type of the prop, and therefore its size in memory even in the None case, this compiles fine.

In this particular case, &dyn Fn() -> i32 will cause lifetime issues, but in other cases, it may be a possibility.

Documenting Components
This is one of the least essential but most important sections of this book. It’s not strictly necessary to document your components and their props. It may be very important, depending on the size of your team and your app. But it’s very easy, and bears immediate fruit.

To document a component and its props, you can simply add doc comments on the component function, and each one of the props:

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
/// The maximum value of the progress bar.
#[prop(default = 100)]
max: u16,
/// How much progress should be displayed.
#[prop(into)]
progress: Signal<i32>,
) -> impl IntoView {
/* ... */
}
That’s all you need to do. These behave like ordinary Rust doc comments, except that you can document individual component props, which can’t be done with Rust function arguments.

This will automatically generate documentation for your component, its Props type, and each of the fields used to add props. It can be a little hard to understand how powerful this is until you hover over the component name or props and see the power of the #[component] macro combined with rust-analyzer here.

Advanced Topic: #[component(transparent)]
All Leptos components return -> impl IntoView. Some, though, need to return some data directly without any additional wrapping. These can be marked with #[component(transparent)], in which case they return exactly the value they return, without the rendering system transforming them in any way.

This is mostly used in two situations:

Creating wrappers around <Suspense/> or <Transition/>, which return a transparent suspense structure to integrate with SSR and hydration properly.
Refactoring <Route/> definitions for leptos_router out into separate components, because <Route/> is a transparent component that returns a RouteDefinition struct rather than a view.
In general, you should not need to use transparent components unless you are creating custom wrapping components that fall into one of these two categories.


use leptos::*;

// Composing different components together is how we build
// user interfaces. Here, we'll define a reusable <ProgressBar/>.
// You'll see how doc comments can be used to document components
// and their properties.

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
// Marks this as an optional prop. It will default to the default
// value of its type, i.e., 0.
#[prop(default = 100)]
/// The maximum value of the progress bar.
max: u16,
// Will run `.into()` on the value passed into the prop.
#[prop(into)]
// `Signal<T>` is a wrapper for several reactive types.
// It can be helpful in component APIs like this, where we
// might want to take any kind of reactive value
/// How much progress should be displayed.
progress: Signal<i32>,
) -> impl IntoView {
view! {
<progress
max={max}
value=progress
/>
<br/>
}
}

#[component]
fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);

    let double_count = move || count() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me"
        </button>
        <br/>
        // If you have this open in CodeSandbox or an editor with
        // rust-analyzer support, try hovering over `ProgressBar`,
        // `max`, or `progress` to see the docs we defined above
        <ProgressBar max=50 progress=count/>
        // Let's use the default max value on this one
        // the default is 100, so it should move half as fast
        <ProgressBar progress=count/>
        // Signal::derive creates a Signal wrapper from our derived signal
        // using double_count means it should move twice as fast
        <ProgressBar max=50 progress=Signal::derive(double_count)/>
    }
}

fn main() {
leptos::mount_to_body(App)
}


Iteration
Whether you’re listing todos, displaying a table, or showing product images, iterating over a list of items is a common task in web applications. Reconciling the differences between changing sets of items can also be one of the trickiest tasks for a framework to handle well.

Leptos supports two different patterns for iterating over items:

For static views: Vec<_>
For dynamic lists: <For/>
Static Views with Vec<_>
Sometimes you need to show an item repeatedly, but the list you’re drawing from does not often change. In this case, it’s important to know that you can insert any Vec<IV> where IV: IntoView into your view. In other words, if you can render T, you can render Vec<T>.

let values = vec![0, 1, 2];
view! {
// this will just render "012"
<p>{values.clone()}</p>
// or we can wrap them in <li>
<ul>
{values.into_iter()
.map(|n| view! { <li>{n}</li>})
.collect::<Vec<_>>()}
</ul>
}
Leptos also provides a .collect_view() helper function that allows you to collect any iterator of T: IntoView into Vec<View>.

let values = vec![0, 1, 2];
view! {
// this will just render "012"
<p>{values.clone()}</p>
// or we can wrap them in <li>
<ul>
{values.into_iter()
.map(|n| view! { <li>{n}</li>})
.collect_view()}
</ul>
}
The fact that the list is static doesn’t mean the interface needs to be static. You can render dynamic items as part of a static list.

// create a list of 5 signals
let length = 5;
let counters = (1..=length).map(|idx| create_signal(idx));

// each item manages a reactive view
// but the list itself will never change
let counter_buttons = counters
.map(|(count, set_count)| {
view! {
<li>
<button
on:click=move |_| set_count.update(|n| *n += 1)
>
{count}
</button>
</li>
}
})
.collect_view();

view! {
<ul>{counter_buttons}</ul>
}
You can render a Fn() -> Vec<_> reactively as well. But note that every time it changes, this will rerender every item in the list. This is quite inefficient! Fortunately, there’s a better way.

Dynamic Rendering with the <For/> Component
The <For/> component is a keyed dynamic list. It takes three props:

each: a function (such as a signal) that returns the items T to be iterated over
key: a key function that takes &T and returns a stable, unique key or ID
children: renders each T into a view
key is, well, the key. You can add, remove, and move items within the list. As long as each item’s key is stable over time, the framework does not need to rerender any of the items, unless they are new additions, and it can very efficiently add, remove, and move items as they change. This allows for extremely efficient updates to the list as it changes, with minimal additional work.

Creating a good key can be a little tricky. You generally do not want to use an index for this purpose, as it is not stable—if you remove or move items, their indices change.

But it’s a great idea to do something like generating a unique ID for each row as it is generated, and using that as an ID for the key function.

Check out the <DynamicList/> component below for an example.



// Iteration is a very common task in most applications.
// So how do you take a list of data and render it in the DOM?
// This example will show you the two ways:
// 1) for mostly-static lists, using Rust iterators
// 2) for lists that grow, shrink, or move items, using <For/>

#[component]
fn App() -> impl IntoView {
view! {
<h1>"Iteration"</h1>
<h2>"Static List"</h2>
<p>"Use this pattern if the list itself is static."</p>
<StaticList length=5/>
<h2>"Dynamic List"</h2>
<p>"Use this pattern if the rows in your list will change."</p>
<DynamicList initial_length=5/>
}
}

/// A list of counters, without the ability
/// to add or remove any.
#[component]
fn StaticList(
/// How many counters to include in this list.
length: usize,
) -> impl IntoView {
// create counter signals that start at incrementing numbers
let counters = (1..=length).map(|idx| create_signal(idx));

    // when you have a list that doesn't change, you can
    // manipulate it using ordinary Rust iterators
    // and collect it into a Vec<_> to insert it into the DOM
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect::<Vec<_>>();

    // Note that if `counter_buttons` were a reactive list
    // and its value changed, this would be very inefficient:
    // it would rerender every row every time the list changed.
    view! {
        <ul>{counter_buttons}</ul>
    }
}

/// A list of counters that allows you to add or
/// remove counters.
#[component]
fn DynamicList(
/// The number of counters to begin with.
initial_length: usize,
) -> impl IntoView {
// This dynamic list will use the <For/> component.
// <For/> is a keyed list. This means that each row
// has a defined key. If the key does not change, the row
// will not be re-rendered. When the list changes, only
// the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        let sig = create_signal(next_counter_id + 1);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=counters
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button
                                    on:click=move |_| set_count.update(|n| *n += 1)
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters.update(|counters| {
                                            counters.retain(|(counter_id, (signal, _))| {
                                                // NOTE: in this example, we are creating the signals
                                                // in the scope of the parent. This means the memory used to
                                                // store them will not be reclaimed until the parent component
                                                // is unmounted. Here, we're removing the signal early (i.e, before
                                                // the DynamicList is unmounted), so we manually dispose of the signal
                                                // to avoid leaking memory.
                                                //
                                                // This is only necessary in an example with nested signals like this one.
                                                if counter_id == &id {
                                                    signal.dispose();
                                                }
                                                counter_id != &id
                                            })
                                        });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

fn main() {
leptos::mount_to_body(App)
}


Iterating over More Complex Data with <For/>
This chapter goes into iteration over nested data structures in a bit more depth. It belongs here with the other chapter on iteration, but feel free to skip it and come back if you’d like to stick with simpler subjects for now.

The Problem
I just said that the framework does not rerender any of the items in one of the rows, unless the key has changed. This probably makes sense at first, but it can easily trip you up.

Let’s consider an example in which each of the items in our row is some data structure. Imagine, for example, that the items come from some JSON array of keys and values:

#[derive(Debug, Clone)]
struct DatabaseEntry {
key: String,
value: i32,
}
Let’s define a simple component that will iterate over the rows and display each one:

#[component]
pub fn App() -> impl IntoView {
// start with a set of three rows
let (data, set_data) = create_signal(vec![
DatabaseEntry {
key: "foo".to_string(),
value: 10,
},
DatabaseEntry {
key: "bar".to_string(),
value: 20,
},
DatabaseEntry {
key: "baz".to_string(),
value: 15,
},
]);
view! {
// when we click, update each row,
// doubling its value
<button on:click=move |_| {
set_data.update(|data| {
for row in data {
row.value *= 2;
}
});
// log the new value of the signal
logging::log!("{:?}", data.get());
}>
"Update Values"
</button>
// iterate over the rows and display each value
<For
each=data
key=|state| state.key.clone()
let:child
>
<p>{child.value}</p>
</For>
}
}
Note the let:child syntax here. In the previous chapter we introduced <For/> with a children prop. We can actually create this value directly in the children of the <For/> component, without breaking out of the view macro: the let:child combined with <p>{child.value}</p> above is the equivalent of

children=|child| view! { <p>{child.value}</p> }
When you click the Update Values button... nothing happens. Or rather: the signal is updated, the new value is logged, but the {child.value} for each row doesn’t update.

Let’s see: is that because we forgot to add a closure to make it reactive? Let’s try {move || child.value}.

...Nope. Still nothing.

Here’s the problem: as I said, each row is only rerendered when the key changes. We’ve updated the value for each row, but not the key for any of the rows, so nothing has rerendered. And if you look at the type of child.value, it’s a plain i32, not a reactive ReadSignal<i32> or something. This means that even if we wrap a closure around it, the value in this row will never update.

We have three possible solutions:

change the key so that it always updates when the data structure changes
change the value so that it’s reactive
take a reactive slice of the data structure instead of using each row directly
Option 1: Change the Key
Each row is only rerendered when the key changes. Our rows above didn’t rerender, because the key didn’t change. So: why not just force the key to change?

<For
each=data
key=|state| (state.key.clone(), state.value)
let:child
>
    <p>{child.value}</p>
</For>
Now we include both the key and the value in the key. This means that whenever the value of a row changes, <For/> will treat it as if it’s an entirely new row, and replace the previous one.

Pros
This is very easy. We can make it even easier by deriving PartialEq, Eq, and Hash on DatabaseEntry, in which case we could just key=|state| state.clone().

Cons
This is the least efficient of the three options. Every time the value of a row changes, it throws out the previous <p> element and replaces it with an entirely new one. Rather than making a fine-grained update to the text node, in other words, it really does rerender the entire row on every change, and this is expensive in proportion to how complex the UI of the row is.

You’ll notice we also end up cloning the whole data structure so that <For/> can hold onto a copy of the key. For more complex structures, this can become a bad idea fast!

Option 2: Nested Signals
If we do want that fine-grained reactivity for the value, one option is to wrap the value of each row in a signal.

#[derive(Debug, Clone)]
struct DatabaseEntry {
key: String,
value: RwSignal<i32>,
}
RwSignal<_> is a “read-write signal,” which combines the getter and setter in one object. I’m using it here because it’s a little easier to store in a struct than separate getters and setters.

#[component]
pub fn App() -> impl IntoView {
// start with a set of three rows
let (data, set_data) = create_signal(vec![
DatabaseEntry {
key: "foo".to_string(),
value: create_rw_signal(10),
},
DatabaseEntry {
key: "bar".to_string(),
value: create_rw_signal(20),
},
DatabaseEntry {
key: "baz".to_string(),
value: create_rw_signal(15),
},
]);
view! {
// when we click, update each row,
// doubling its value
<button on:click=move |_| {
data.with(|data| {
for row in data {
row.value.update(|value| *value *= 2);
}
});
// log the new value of the signal
logging::log!("{:?}", data.get());
}>
"Update Values"
</button>
// iterate over the rows and display each value
<For
each=data
key=|state| state.key.clone()
let:child
>
<p>{child.value}</p>
</For>
}
}
This version works! And if you look in the DOM inspector in your browser, you’ll see that unlike in the previous version, in this version only the individual text nodes are updated. Passing the signal directly into {child.value} works, as signals do keep their reactivity if you pass them into the view.

Note that I changed the set_data.update() to a data.with(). .with() is the non-cloning way of accessing a signal’s value. In this case, we are only updating the internal values, not updating the list of values: because signals maintain their own state, we don’t actually need to update the data signal at all, so the immutable .with() is fine here.

In fact, this version doesn’t update data, so the <For/> is essentially a static list as in the last chapter, and this could just be a plain iterator. But the <For/> is useful if we want to add or remove rows in the future.

Pros
This is the most efficient option, and fits directly with the rest of the mental model of the framework: values that change over time are wrapped in signals so the interface can respond to them.

Cons
Nested reactivity can be cumbersome if you’re receiving data from an API or another data source you don’t control, and you don’t want to create a different struct wrapping each field in a signal.

Option 3: Memoized Slices
Leptos provides a primitive called create_memo, which creates a derived computation that only triggers a reactive update when its value has changed.

This allows you to create reactive values for subfields of a larger data structure, without needing to wrap the fields of that structure in signals.

Most of the application can remain the same as the initial (broken) version, but the <For/> will be updated to this:

<For
each=move || data().into_iter().enumerate()
key=|(_, state)| state.key.clone()
children=move |(index, _)| {
let value = create_memo(move |_| {
data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
});
view! {
<p>{value}</p>
}
}
/>
You’ll notice a few differences here:

we convert the data signal into an enumerated iterator
we use the children prop explicitly, to make it easier to run some non-view code
we define a value memo and use that in the view. This value field doesn’t actually use the child being passed into each row. Instead, it uses the index and reaches back into the original data to get the value.
Every time data changes, now, each memo will be recalculated. If its value has changed, it will update its text node, without rerendering the whole row.

Pros
We get the same fine-grained reactivity of the signal-wrapped version, without needing to wrap the data in signals.

Cons
It’s a bit more complex to set up this memo-per-row inside the <For/> loop rather than using nested signals. For example, you’ll notice that we have to guard against the possibility that the data[index] would panic by using data.get(index), because this memo may be triggered to re-run once just after the row is removed. (This is because the memo for each row and the whole <For/> both depend on the same data signal, and the order of execution for multiple reactive values that depend on the same signal isn’t guaranteed.)

Note also that while memos memoize their reactive changes, the same calculation does need to re-run to check the value every time, so nested reactive signals will still be more efficient for pinpoint updates here.


Forms and Inputs
Forms and form inputs are an important part of interactive apps. There are two basic patterns for interacting with inputs in Leptos, which you may recognize if you’re familiar with React, SolidJS, or a similar framework: using controlled or uncontrolled inputs.

Controlled Inputs
In a "controlled input," the framework controls the state of the input element. On every input event, it updates a local signal that holds the current state, which in turn updates the value prop of the input.

There are two important things to remember:

The input event fires on (almost) every change to the element, while the change event fires (more or less) when you unfocus the input. You probably want on:input, but we give you the freedom to choose.
The value attribute only sets the initial value of the input, i.e., it only updates the input up to the point that you begin typing. The value property continues updating the input after that. You usually want to set prop:value for this reason. (The same is true for checked and prop:checked on an <input type="checkbox">.)
let (name, set_name) = create_signal("Controlled".to_string());

view! {
<input type="text"
on:input=move |ev| {
// event_target_value is a Leptos helper function
// it functions the same way as event.target.value
// in JavaScript, but smooths out some of the typecasting
// necessary to make this work in Rust
set_name(event_target_value(&ev));
}

        // the `prop:` syntax lets you update a DOM property,
        // rather than an attribute.
        prop:value=name
    />
    <p>"Name is: " {name}</p>
}
Why do you need prop:value?
Web browsers are the most ubiquitous and stable platform for rendering graphical user interfaces in existence. They have also maintained an incredible backwards compatibility over their three decades of existence. Inevitably, this means there are some quirks.

One odd quirk is that there is a distinction between HTML attributes and DOM element properties, i.e., between something called an “attribute” which is parsed from HTML and can be set on a DOM element with .setAttribute(), and something called a “property” which is a field of the JavaScript class representation of that parsed HTML element.

In the case of an <input value=...>, setting the value attribute is defined as setting the initial value for the input, and setting value property sets its current value. It maybe easiest to understand this by opening about:blank and running the following JavaScript in the browser console, line by line:

// create an input and append it to the DOM
const el = document.createElement("input");
document.body.appendChild(el);

el.setAttribute("value", "test"); // updates the input
el.setAttribute("value", "another test"); // updates the input again

// now go and type into the input: delete some characters, etc.

el.setAttribute("value", "one more time?");
// nothing should have changed. setting the "initial value" does nothing now

// however...
el.value = "But this works";
Many other frontend frameworks conflate attributes and properties, or create a special case for inputs that sets the value correctly. Maybe Leptos should do this too; but for now, I prefer giving users the maximum amount of control over whether they’re setting an attribute or a property, and doing my best to educate people about the actual underlying browser behavior rather than obscuring it.

Uncontrolled Inputs
In an "uncontrolled input," the browser controls the state of the input element. Rather than continuously updating a signal to hold its value, we use a NodeRef to access the input when we want to get its value.

In this example, we only notify the framework when the <form> fires a submit event. Note the use of the leptos::html module, which provides a bunch of types for every HTML element.

let (name, set_name) = create_signal("Uncontrolled".to_string());

let input_element: NodeRef<html::Input> = create_node_ref();

view! {
<form on:submit=on_submit> // on_submit defined below
<input type="text"
value=name
node_ref=input_element
/>
<input type="submit" value="Submit"/>
</form>
<p>"Name is: " {name}</p>
}
The view should be pretty self-explanatory by now. Note two things:

Unlike in the controlled input example, we use value (not prop:value). This is because we’re just setting the initial value of the input, and letting the browser control its state. (We could use prop:value instead.)
We use node_ref=... to fill the NodeRef. (Older examples sometimes use _ref. They are the same thing, but node_ref has better rust-analyzer support.)
NodeRef is a kind of reactive smart pointer: we can use it to access the underlying DOM node. Its value will be set when the element is rendered.

let on_submit = move |ev: leptos::ev::SubmitEvent| {
// stop the page from reloading!
ev.prevent_default();

    // here, we'll extract the value from the input
    let value = input_element()
        // event handlers can only fire after the view
        // is mounted to the DOM, so the `NodeRef` will be `Some`
        .expect("<input> should be mounted")
        // `leptos::HtmlElement<html::Input>` implements `Deref`
        // to a `web_sys::HtmlInputElement`.
        // this means we can call`HtmlInputElement::value()`
        // to get the current value of the input
        .value();
    set_name(value);
};
Our on_submit handler will access the input’s value and use it to call set_name. To access the DOM node stored in the NodeRef, we can simply call it as a function (or using .get()). This will return Option<leptos::HtmlElement<html::Input>>, but we know that the element has already been mounted (how else did you fire this event!), so it's safe to unwrap here.

We can then call .value() to get the value out of the input, because NodeRef gives us access to a correctly-typed HTML element.

Take a look at web_sys and HtmlElement to learn more about using a leptos::HtmlElement. Also see the full CodeSandbox example at the end of this page.

Special Cases: <textarea> and <select>
Two form elements tend to cause some confusion, in different ways.

<textarea>
Unlike <input>, the <textarea> element does not support a value attribute. Instead, it receives its value as a plain text node in its HTML children.

In the current version of Leptos (in fact in Leptos 0.1-0.6), creating a dynamic child inserts a comment marker node. This can cause incorrect <textarea> rendering (and issues during hydration) if you try to use it to show dynamic content.

Instead, you can pass a non-reactive initial value as a child, and use prop:value to set its current value. (<textarea> doesn’t support the value attribute, but does support the value property...)

view! {
<textarea
prop:value=move || some_value.get()
on:input=/* etc */
>
/* plain-text initial value, does not change if the signal changes */
{some_value.get_untracked()}
</textarea>
}
<select>
The <select> element also does not have a value attribute, nor a value property. Instead, its value is determined by the selected attribute of its <option> fields. Some frameworks obscure this with a value field on <select>; if you try this in Leptos (or vanilla JavaScript) it won’t work.

To use the selected field:

let (value, set_value) = create_signal("B".to_string());
view! {
<select on:change=move |ev| {
let new_value = event_target_value(&ev);
set_value(new_value);
}>
<option
value="A"
selected=move || value() == "A"
>
"A"
</option>
<option
value="B"
selected=move || value() == "B"
>
"B"
</option>
</select>
}
That's somewhat repetitive, but can easily be refactored:

#[component]
pub fn App() -> impl IntoView {
let (value, set_value) = create_signal("B".to_string());
view! {
<select on:change=move |ev| {
let new_value = event_target_value(&ev);
set_value(new_value);
}>
<SelectOption value is="A"/>
<SelectOption value is="B"/>
<SelectOption value is="C"/>
</select>
}
}

#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
view! {
<option
value=is
selected=move || value() == is
>
{is}
</option>
}
}
Tip: the single value attribute in the component is equivalent to value=value. This is only the case for components: in HTML elements, a single value attribute is equivalent to value=true. This is expected to be made consistent in the next major version of Leptos; see this issue for more details.

use leptos::{ev::SubmitEvent, *};

#[component]
fn App() -> impl IntoView {
view! {
<h2>"Controlled Component"</h2>
<ControlledComponent/>
<h2>"Uncontrolled Component"</h2>
<UncontrolledComponent/>
}
}

#[component]
fn ControlledComponent() -> impl IntoView {
// create a signal to hold the value
let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input type="text"
            // fire an event whenever the input changes
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // it functions the same way as event.target.value
                // in JavaScript, but smooths out some of the typecasting
                // necessary to make this work in Rust
                set_name(event_target_value(&ev));
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            //
            // IMPORTANT: the `value` *attribute* only sets the
            // initial value, until you have made a change.
            // The `value` *property* sets the current value.
            // This is a quirk of the DOM; I didn't invent it.
            // Other frameworks gloss this over; I think it's
            // more important to give you access to the browser
            // as it really works.
            //
            // tl;dr: use prop:value for form inputs
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn UncontrolledComponent() -> impl IntoView {
// import the type for <input>
use leptos::html::Input;

    let (name, set_name) = create_signal("Uncontrolled".to_string());

    // we'll use a NodeRef to store a reference to the input element
    // this will be filled when the element is created
    let input_element: NodeRef<Input> = create_node_ref();

    // fires when the form `submit` event happens
    // this will store the value of the <input> in our signal
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text"
                // here, we use the `value` *attribute* to set only
                // the initial value, letting the browser maintain
                // the state after that
                value=name

                // store a reference to this input in `input_element`
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

// This `main` function is the entry point into the app
// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
leptos::mount_to_body(App)
}


Control Flow
In most applications, you sometimes need to make a decision: Should I render this part of the view, or not? Should I render <ButtonA/> or <WidgetB/>? This is control flow.

A Few Tips
When thinking about how to do this with Leptos, it’s important to remember a few things:

Rust is an expression-oriented language: control-flow expressions like if x() { y } else { z } and match x() { ... } return their values. This makes them very useful for declarative user interfaces.
For any T that implements IntoView—in other words, for any type that Leptos knows how to render—Option<T> and Result<T, impl Error> also implement IntoView. And just as Fn() -> T renders a reactive T, Fn() -> Option<T> and Fn() -> Result<T, impl Error> are reactive.
Rust has lots of handy helpers like Option::map, Option::and_then, Option::ok_or, Result::map, Result::ok, and bool::then that allow you to convert, in a declarative way, between a few different standard types, all of which can be rendered. Spending time in the Option and Result docs in particular is one of the best ways to level up your Rust game.
And always remember: to be reactive, values must be functions. You’ll see me constantly wrap things in a move || closure, below. This is to ensure that they actually rerun when the signal they depend on changes, keeping the UI reactive.
So What?
To connect the dots a little: this means that you can actually implement most of your control flow with native Rust code, without any control-flow components or special knowledge.

For example, let’s start with a simple signal and derived signal:

let (value, set_value) = create_signal(0);
let is_odd = move || value() & 1 == 1;
If you don’t recognize what’s going on with is_odd, don’t worry about it too much. It’s just a simple way to test whether an integer is odd by doing a bitwise AND with 1.

We can use these signals and ordinary Rust to build most control flow.

if statements
Let’s say I want to render some text if the number is odd, and some other text if it’s even. Well, how about this?

view! {
<p>
{move || if is_odd() {
"Odd"
} else {
"Even"
}}
</p>
}
An if expression returns its value, and a &str implements IntoView, so a Fn() -> &str implements IntoView, so this... just works!

Option<T>
Let’s say we want to render some text if it’s odd, and nothing if it’s even.

let message = move || {
if is_odd() {
Some("Ding ding ding!")
} else {
None
}
};

view! {
<p>{message}</p>
}
This works fine. We can make it a little shorter if we’d like, using bool::then().

let message = move || is_odd().then(|| "Ding ding ding!");
view! {
<p>{message}</p>
}
You could even inline this if you’d like, although personally I sometimes like the better cargo fmt and rust-analyzer support I get by pulling things out of the view.

match statements
We’re still just writing ordinary Rust code, right? So you have all the power of Rust’s pattern matching at your disposal.

let message = move || {
match value() {
0 => "Zero",
1 => "One",
n if is_odd() => "Odd",
_ => "Even"
}
};
view! {
<p>{message}</p>
}
And why not? YOLO, right?

Preventing Over-Rendering
Not so YOLO.

Everything we’ve just done is basically fine. But there’s one thing you should remember and try to be careful with. Each one of the control-flow functions we’ve created so far is basically a derived signal: it will rerun every time the value changes. In the examples above, where the value switches from even to odd on every change, this is fine.

But consider the following example:

let (value, set_value) = create_signal(0);

let message = move || if value() > 5 {
"Big"
} else {
"Small"
};

view! {
<p>{message}</p>
}
This works, for sure. But if you added a log, you might be surprised

let message = move || if value() > 5 {
logging::log!("{}: rendering Big", value());
"Big"
} else {
logging::log!("{}: rendering Small", value());
"Small"
};
As a user clicks a button, you’d see something like this:

1: rendering Small
2: rendering Small
3: rendering Small
4: rendering Small
5: rendering Small
6: rendering Big
7: rendering Big
8: rendering Big
... ad infinitum
Every time value changes, it reruns the if statement. This makes sense, with how reactivity works. But it has a downside. For a simple text node, rerunning the if statement and rerendering isn’t a big deal. But imagine it were like this:

let message = move || if value() > 5 {
<Big/>
} else {
<Small/>
};
This rerenders <Small/> five times, then <Big/> infinitely. If they’re loading resources, creating signals, or even just creating DOM nodes, this is unnecessary work.

<Show/>
The <Show/> component is the answer. You pass it a when condition function, a fallback to be shown if the when function returns false, and children to be rendered if when is true.

let (value, set_value) = create_signal(0);

view! {
<Show
when=move || { value() > 5 }
fallback=|| view! { <Small/> }
>
    <Big/>
  </Show>
}
<Show/> memoizes the when condition, so it only renders its <Small/> once, continuing to show the same component until value is greater than five; then it renders <Big/> once, continuing to show it indefinitely or until value goes below five and then renders <Small/> again.

This is a helpful tool to avoid rerendering when using dynamic if expressions. As always, there's some overhead: for a very simple node (like updating a single text node, or updating a class or attribute), a move || if ... will be more efficient. But if it’s at all expensive to render either branch, reach for <Show/>.

Note: Type Conversions
There‘s one final thing it’s important to say in this section.

The view macro doesn’t return the most-generic wrapping type View. Instead, it returns things with types like Fragment or HtmlElement<Input>. This can be a little annoying if you’re returning different HTML elements from different branches of a conditional:

view! {
<main>
{move || match is_odd() {
true if value() == 1 => {
// returns HtmlElement<Pre>
view! { <pre>"One"</pre> }
},
false if value() == 2 => {
// returns HtmlElement<P>
view! { <p>"Two"</p> }
}
// returns HtmlElement<Textarea>
_ => view! { <textarea>{value()}</textarea> }
}}
</main>
}
This strong typing is actually very powerful, because HtmlElement is, among other things, a smart pointer: each HtmlElement<T> type implements Deref for the appropriate underlying web_sys type. In other words, in the browser your view returns real DOM elements, and you can access native DOM methods on them.

But it can be a little annoying in conditional logic like this, because you can’t return different types from different branches of a condition in Rust. There are two ways to get yourself out of this situation:

If you have multiple HtmlElement types, convert them to HtmlElement<AnyElement> with .into_any()
If you have a variety of view types that are not all HtmlElement, convert them to Views with .into_view().
Here’s the same example, with the conversion added:

view! {
<main>
{move || match is_odd() {
true if value() == 1 => {
// returns HtmlElement<Pre>
view! { <pre>"One"</pre> }.into_any()
},
false if value() == 2 => {
// returns HtmlElement<P>
view! { <p>"Two"</p> }.into_any()
}
// returns HtmlElement<Textarea>
_ => view! { <textarea>{value()}</textarea> }.into_any()
}}
</main>
}

other code


use leptos::*;

#[component]
fn App() -> impl IntoView {
let (value, set_value) = create_signal(0);
let is_odd = move || value() & 1 == 1;
let odd_text = move || if is_odd() { Some("How odd!") } else { None };

    view! {
        <h1>"Control Flow"</h1>

        // Simple UI to update and show a value
        <button on:click=move |_| set_value.update(|n| *n += 1)>
            "+1"
        </button>
        <p>"Value is: " {value}</p>

        <hr/>

        <h2><code>"Option<T>"</code></h2>
        // For any `T` that implements `IntoView`,
        // so does `Option<T>`

        <p>{odd_text}</p>
        // This means you can use `Option` methods on it
        <p>{move || odd_text().map(|text| text.len())}</p>

        <h2>"Conditional Logic"</h2>
        // You can do dynamic conditional if-then-else
        // logic in several ways
        //
        // a. An "if" expression in a function
        //    This will simply re-render every time the value
        //    changes, which makes it good for lightweight UI
        <p>
            {move || if is_odd() {
                "Odd"
            } else {
                "Even"
            }}
        </p>

        // b. Toggling some kind of class
        //    This is smart for an element that's going to
        //    toggled often, because it doesn't destroy
        //    it in between states
        //    (you can find the `hidden` class in `index.html`)
        <p class:hidden=is_odd>"Appears if even."</p>

        // c. The <Show/> component
        //    This only renders the fallback and the child
        //    once, lazily, and toggles between them when
        //    needed. This makes it more efficient in many cases
        //    than a {move || if ...} block
        <Show when=is_odd
            fallback=|| view! { <p>"Even steven"</p> }
        >
            <p>"Oddment"</p>
        </Show>

        // d. Because `bool::then()` converts a `bool` to
        //    `Option`, you can use it to create a show/hide toggled
        {move || is_odd().then(|| view! { <p>"Oddity!"</p> })}

        <h2>"Converting between Types"</h2>
        // e. Note: if branches return different types,
        //    you can convert between them with
        //    `.into_any()` (for different HTML element types)
        //    or `.into_view()` (for all view types)
        {move || match is_odd() {
            true if value() == 1 => {
                // <pre> returns HtmlElement<Pre>
                view! { <pre>"One"</pre> }.into_any()
            },
            false if value() == 2 => {
                // <p> returns HtmlElement<P>
                // so we convert into a more generic type
                view! { <p>"Two"</p> }.into_any()
            }
            _ => view! { <textarea>{value()}</textarea> }.into_any()
        }}
    }
}

fn main() {
leptos::mount_to_body(App)
}


Error Handling
In the last chapter, we saw that you can render Option<T>: in the None case, it will render nothing, and in the Some(T) case, it will render T (that is, if T implements IntoView). You can actually do something very similar with a Result<T, E>. In the Err(_) case, it will render nothing. In the Ok(T) case, it will render the T.

Let’s start with a simple component to capture a number input.

#[component]
fn NumericInput() -> impl IntoView {
let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <label>
            "Type a number (or not!)"
            <input type="number" on:input=on_input/>
            <p>
                "You entered "
                <strong>{value}</strong>
            </p>
        </label>
    }
}
Every time you change the input, on_input will attempt to parse its value into a 32-bit integer (i32), and store it in our value signal, which is a Result<i32, _>. If you type the number 42, the UI will display

You entered 42
But if you type the stringfoo, it will display

You entered
This is not great. It saves us using .unwrap_or_default() or something, but it would be much nicer if we could catch the error and do something with it.

You can do that, with the <ErrorBoundary/> component.

<ErrorBoundary/>
An <ErrorBoundary/> is a little like the <Show/> component we saw in the last chapter. If everything’s okay—which is to say, if everything is Ok(_)—it renders its children. But if there’s an Err(_) rendered among those children, it will trigger the <ErrorBoundary/>’s fallback.

Let’s add an <ErrorBoundary/> to this example.

#[component]
fn NumericInput() -> impl IntoView {
let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect_view()
                            }
                        </ul>
                    </div>
                }
            >
                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}
Now, if you type 42, value is Ok(42) and you’ll see

You entered 42

If you type foo, value is Err(_) and the fallback will render. We’ve chosen to render the list of errors as a String, so you’ll see something like

Not a number! Errors:
- cannot parse integer from empty string
  If you fix the error, the error message will disappear and the content you’re wrapping in an <ErrorBoundary/> will appear again.

use leptos::*;

#[component]
fn App() -> impl IntoView {
let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input=on_input/>
            // If an `Err(_) had been rendered inside the <ErrorBoundary/>,
            // the fallback will be displayed. Otherwise, the children of the
            // <ErrorBoundary/> will be displayed.
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors
                        // as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                }
            >
                <p>
                    "You entered "
                    // because `value` is `Result<i32, _>`,
                    // it will render the `i32` if it is `Ok`,
                    // and render nothing and trigger the error boundary
                    // if it is `Err`. It's a signal, so this will dynamically
                    // update when `value` changes
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>
        </label>
    }
}

fn main() {
leptos::mount_to_body(App)
}


Parent-Child Communication
You can think of your application as a nested tree of components. Each component handles its own local state and manages a section of the user interface, so components tend to be relatively self-contained.

Sometimes, though, you’ll want to communicate between a parent component and its child. For example, imagine you’ve defined a <FancyButton/> component that adds some styling, logging, or something else to a <button/>. You want to use a <FancyButton/> in your <App/> component. But how can you communicate between the two?

It’s easy to communicate state from a parent component to a child component. We covered some of this in the material on components and props. Basically if you want the parent to communicate to the child, you can pass a ReadSignal, a Signal, or even a MaybeSignal as a prop.

But what about the other direction? How can a child send notifications about events or state changes back up to the parent?

There are four basic patterns of parent-child communication in Leptos.


1. Pass a WriteSignal
   One approach is simply to pass a WriteSignal from the parent down to the child, and update it in the child. This lets you manipulate the state of the parent from the child.

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);
view! {
<p>"Toggled? " {toggled}</p>
<ButtonA setter=set_toggled/>
}
}

#[component]
pub fn ButtonA(setter: WriteSignal<bool>) -> impl IntoView {
view! {
<button
on:click=move |_| setter.update(|value| *value = !*value)
>
"Toggle"
</button>
}
}
This pattern is simple, but you should be careful with it: passing around a WriteSignal can make it hard to reason about your code. In this example, it’s pretty clear when you read <App/> that you are handing off the ability to mutate toggled, but it’s not at all clear when or how it will change. In this small, local example it’s easy to understand, but if you find yourself passing around WriteSignals like this throughout your code, you should really consider whether this is making it too easy to write spaghetti code.

2. Use a Callback
   Another approach would be to pass a callback to the child: say, on_click.

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);
view! {
<p>"Toggled? " {toggled}</p>
<ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value)/>
}
}


#[component]
pub fn ButtonB(#[prop(into)] on_click: Callback<MouseEvent>) -> impl IntoView
{
view! {
<button on:click=on_click>
"Toggle"
</button>
}
}
You’ll notice that whereas <ButtonA/> was given a WriteSignal and decided how to mutate it, <ButtonB/> simply fires an event: the mutation happens back in <App/>. This has the advantage of keeping local state local, preventing the problem of spaghetti mutation. But it also means the logic to mutate that signal needs to exist up in <App/>, not down in <ButtonB/>. These are real trade-offs, not a simple right-or-wrong choice.

Note the way we use the Callback<In, Out> type. This is basically a wrapper around a closure Fn(In) -> Out that is also Copy and makes it easy to pass around.

We also used the #[prop(into)] attribute so we can pass a normal closure into on_click. Please see the chapter "into Props" for more details.

2.1 Use Closure instead of Callback
You can use a Rust closure Fn(MouseEvent) directly instead of Callback:

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);
view! {
<p>"Toggled? " {toggled}</p>
<ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value)/>
}
}


#[component]
pub fn ButtonB<F>(on_click: F) -> impl IntoView
where
F: Fn(MouseEvent) + 'static
{
view! {
<button on:click=on_click>
"Toggle"
</button>
}
}
The code is very similar in this case. On more advanced use-cases using a closure might require some cloning compared to using a Callback.

Note the way we declare the generic type F here for the callback. If you’re confused, look back at the generic props section of the chapter on components.

3. Use an Event Listener
   You can actually write Option 2 in a slightly different way. If the callback maps directly onto a native DOM event, you can add an on: listener directly to the place you use the component in your view macro in <App/>.

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);
view! {
<p>"Toggled? " {toggled}</p>
// note the on:click instead of on_click
// this is the same syntax as an HTML element event listener
<ButtonC on:click=move |_| set_toggled.update(|value| *value = !*value)/>
}
}


#[component]
pub fn ButtonC() -> impl IntoView {
view! {
<button>"Toggle"</button>
}
}
This lets you write way less code in <ButtonC/> than you did for <ButtonB/>, and still gives a correctly-typed event to the listener. This works by adding an on: event listener to each element that <ButtonC/> returns: in this case, just the one <button>.

Of course, this only works for actual DOM events that you’re passing directly through to the elements you’re rendering in the component. For more complex logic that doesn’t map directly onto an element (say you create <ValidatedForm/> and want an on_valid_form_submit callback) you should use Option 2.

4. Providing a Context
   This version is actually a variant on Option 1. Say you have a deeply-nested component tree:

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);
view! {
<p>"Toggled? " {toggled}</p>
<Layout/>
}
}

#[component]
pub fn Layout() -> impl IntoView {
view! {
<header>
<h1>"My Page"</h1>
</header>
<main>
<Content/>
</main>
}
}

#[component]
pub fn Content() -> impl IntoView {
view! {
<div class="content">
<ButtonD/>
</div>
}
}

#[component]
pub fn ButtonD<F>() -> impl IntoView {
todo!()
}
Now <ButtonD/> is no longer a direct child of <App/>, so you can’t simply pass your WriteSignal to its props. You could do what’s sometimes called “prop drilling,” adding a prop to each layer between the two:

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);
view! {
<p>"Toggled? " {toggled}</p>
<Layout set_toggled/>
}
}

#[component]
pub fn Layout(set_toggled: WriteSignal<bool>) -> impl IntoView {
view! {
<header>
<h1>"My Page"</h1>
</header>
<main>
<Content set_toggled/>
</main>
}
}

#[component]
pub fn Content(set_toggled: WriteSignal<bool>) -> impl IntoView {
view! {
<div class="content">
<ButtonD set_toggled/>
</div>
}
}

#[component]
pub fn ButtonD<F>(set_toggled: WriteSignal<bool>) -> impl IntoView {
todo!()
}
This is a mess. <Layout/> and <Content/> don’t need set_toggled; they just pass it through to <ButtonD/>. But I need to declare the prop in triplicate. This is not only annoying but hard to maintain: imagine we add a “half-toggled” option and the type of set_toggled needs to change to an enum. We have to change it in three places!

Isn’t there some way to skip levels?

There is!

4.1 The Context API
You can provide data that skips levels by using provide_context and use_context. Contexts are identified by the type of the data you provide (in this example, WriteSignal<bool>), and they exist in a top-down tree that follows the contours of your UI tree. In this example, we can use context to skip the unnecessary prop drilling.

#[component]
pub fn App() -> impl IntoView {
let (toggled, set_toggled) = create_signal(false);

    // share `set_toggled` with all children of this component
    provide_context(set_toggled);

    view! {
        <p>"Toggled? " {toggled}</p>
        <Layout/>
    }
}

// <Layout/> and <Content/> omitted
// To work in this version, drop their references to set_toggled

#[component]
pub fn ButtonD() -> impl IntoView {
// use_context searches up the context tree, hoping to
// find a `WriteSignal<bool>`
// in this case, I .expect() because I know I provided it
let setter = use_context::<WriteSignal<bool>>()
.expect("to have found the setter provided");

    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle"
        </button>
    }
}
The same caveats apply to this as to <ButtonA/>: passing a WriteSignal around should be done with caution, as it allows you to mutate state from arbitrary parts of your code. But when done carefully, this can be one of the most effective techniques for global state management in Leptos: simply provide the state at the highest level you’ll need it, and use it wherever you need it lower down.

Note that there are no performance downsides to this approach. Because you are passing a fine-grained reactive signal, nothing happens in the intervening components (<Layout/> and <Content/>) when you update it. You are communicating directly between <ButtonD/> and <App/>. In fact—and this is the power of fine-grained reactivity—you are communicating directly between a button click in <ButtonD/> and a single text node in <App/>. It’s as if the components themselves don’t exist at all. And, well... at runtime, they don’t. It’s just signals and effects, all the way down.

use leptos::{ev::MouseEvent, *};

// This highlights four different ways that child components can communicate
// with their parent:
// 1) <ButtonA/>: passing a WriteSignal as one of the child component props,
//    for the child component to write into and the parent to read
// 2) <ButtonB/>: passing a closure as one of the child component props, for
//    the child component to call
// 3) <ButtonC/>: adding an `on:` event listener to a component
// 4) <ButtonD/>: providing a context that is used in the component (rather than prop drilling)

#[derive(Copy, Clone)]
struct SmallcapsContext(WriteSignal<bool>);

#[component]
pub fn App() -> impl IntoView {
// just some signals to toggle three classes on our <p>
let (red, set_red) = create_signal(false);
let (right, set_right) = create_signal(false);
let (italics, set_italics) = create_signal(false);
let (smallcaps, set_smallcaps) = create_signal(false);

    // the newtype pattern isn't *necessary* here but is a good practice
    // it avoids confusion with other possible future `WriteSignal<bool>` contexts
    // and makes it easier to refer to it in ButtonC
    provide_context(SmallcapsContext(set_smallcaps));

    view! {
        <main>
            <p
                // class: attributes take F: Fn() => bool, and these signals all implement Fn()
                class:red=red
                class:right=right
                class:italics=italics
                class:smallcaps=smallcaps
            >
                "Lorem ipsum sit dolor amet."
            </p>

            // Button A: pass the signal setter
            <ButtonA setter=set_red/>

            // Button B: pass a closure
            <ButtonB on_click=move |_| set_right.update(|value| *value = !*value)/>

            // Button B: use a regular event listener
            // setting an event listener on a component like this applies it
            // to each of the top-level elements the component returns
            <ButtonC on:click=move |_| set_italics.update(|value| *value = !*value)/>

            // Button D gets its setter from context rather than props
            <ButtonD/>
        </main>
    }
}

/// Button A receives a signal setter and updates the signal itself
#[component]
pub fn ButtonA(
/// Signal that will be toggled when the button is clicked.
setter: WriteSignal<bool>,
) -> impl IntoView {
view! {
<button
on:click=move |_| setter.update(|value| *value = !*value)
>
"Toggle Red"
</button>
}
}

/// Button B receives a closure
#[component]
pub fn ButtonB<F>(
/// Callback that will be invoked when the button is clicked.
on_click: F,
) -> impl IntoView
where
F: Fn(MouseEvent) + 'static,
{
view! {
<button
on:click=on_click
>
"Toggle Right"
</button>
}

    // just a note: in an ordinary function ButtonB could take on_click: impl Fn(MouseEvent) + 'static
    // and save you from typing out the generic
    // the component macro actually expands to define a
    //
    // struct ButtonBProps<F> where F: Fn(MouseEvent) + 'static {
    //   on_click: F
    // }
    //
    // this is what allows us to have named props in our component invocation,
    // instead of an ordered list of function arguments
    // if Rust ever had named function arguments we could drop this requirement
}

/// Button C is a dummy: it renders a button but doesn't handle
/// its click. Instead, the parent component adds an event listener.
#[component]
pub fn ButtonC() -> impl IntoView {
view! {
<button>
"Toggle Italics"
</button>
}
}

/// Button D is very similar to Button A, but instead of passing the setter as a prop
/// we get it from the context
#[component]
pub fn ButtonD() -> impl IntoView {
let setter = use_context::<SmallcapsContext>().unwrap().0;

    view! {
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
            "Toggle Small Caps"
        </button>
    }
}

fn main() {
leptos::mount_to_body(App)
}


Component Children
It’s pretty common to want to pass children into a component, just as you can pass children into an HTML element. For example, imagine I have a <FancyForm/> component that enhances an HTML <form>. I need some way to pass all its inputs.

view! {
<Form>
<fieldset>
<label>
"Some Input"
<input type="text" name="something"/>
</label>
</fieldset>
<button>"Submit"</button>
</Form>
}
How can you do this in Leptos? There are basically two ways to pass components to other components:

render props: properties that are functions that return a view
the children prop: a special component property that includes anything you pass as a child to the component.
In fact, you’ve already seen these both in action in the <Show/> component:

view! {
<Show
// `when` is a normal prop
when=move || value() > 5
// `fallback` is a "render prop": a function that returns a view
fallback=|| view! { <Small/> }
>
    // `<Big/>` (and anything else here)
    // will be given to the `children` prop
    <Big/>
  </Show>
}
Let’s define a component that takes some children and a render prop.

#[component]
pub fn TakesChildren<F, IV>(
/// Takes a function (type F) that returns anything that can be
/// converted into a View (type IV)
render_prop: F,
/// `children` takes the `Children` type
children: Children,
) -> impl IntoView
where
F: Fn() -> IV,
IV: IntoView,
{
view! {
<h2>"Render Prop"</h2>
{render_prop()}

        <h2>"Children"</h2>
        {children()}
    }
}
render_prop and children are both functions, so we can call them to generate the appropriate views. children, in particular, is an alias for Box<dyn FnOnce() -> Fragment>. (Aren't you glad we named it Children instead?)

If you need a Fn or FnMut here because you need to call children more than once, we also provide ChildrenFn and ChildrenMut aliases.

We can use the component like this:

view! {
<TakesChildren render_prop=|| view! { <p>"Hi, there!"</p> }>
// these get passed to `children`
"Some text"
<span>"A span"</span>
</TakesChildren>
}
Manipulating Children
The Fragment type is basically a way of wrapping a Vec<View>. You can insert it anywhere into your view.

But you can also access those inner views directly to manipulate them. For example, here’s a component that takes its children and turns them into an unordered list.

#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
// Fragment has `nodes` field that contains a Vec<View>
let children = children()
.nodes
.into_iter()
.map(|child| view! { <li>{child}</li> })
.collect_view();

    view! {
        <ul>{children}</ul>
    }
}
Calling it like this will create a list:


view! {
<WrapsChildren>
"A"
"B"
"C"
</WrapsChildren>
}

other code

use leptos::*;

// Often, you want to pass some kind of child view to another
// component. There are two basic patterns for doing this:
// - "render props": creating a component prop that takes a function
//   that creates a view
// - the `children` prop: a special property that contains content
//   passed as the children of a component in your view, not as a
//   property

#[component]
pub fn App() -> impl IntoView {
let (items, set_items) = create_signal(vec![0, 1, 2]);
let render_prop = move || {
// items.with(...) reacts to the value without cloning
// by applying a function. Here, we pass the `len` method
// on a `Vec<_>` directly
let len = move || items.with(Vec::len);
view! {
<p>"Length: " {len}</p>
}
};

    view! {
        // This component just displays the two kinds of children,
        // embedding them in some other markup
        <TakesChildren
            // for component props, you can shorthand
            // `render_prop=render_prop` => `render_prop`
            // (this doesn't work for HTML element attributes)
            render_prop
        >
            // these look just like the children of an HTML element
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </TakesChildren>
        <hr/>
        // This component actually iterates over and wraps the children
        <WrapsChildren>
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </WrapsChildren>
    }
}

/// Displays a `render_prop` and some children within markup.
#[component]
pub fn TakesChildren<F, IV>(
/// Takes a function (type F) that returns anything that can be
/// converted into a View (type IV)
render_prop: F,
/// `children` takes the `Children` type
/// this is an alias for `Box<dyn FnOnce() -> Fragment>`
/// ... aren't you glad we named it `Children` instead?
children: Children,
) -> impl IntoView
where
F: Fn() -> IV,
IV: IntoView,
{
view! {
<h1><code>"<TakesChildren/>"</code></h1>
<h2>"Render Prop"</h2>
{render_prop()}
<hr/>
<h2>"Children"</h2>
{children()}
}
}

/// Wraps each child in an `<li>` and embeds them in a `<ul>`.
#[component]
pub fn WrapsChildren(children: Children) -> impl IntoView {
// children() returns a `Fragment`, which has a
// `nodes` field that contains a Vec<View>
// this means we can iterate over the children
// to create something new!
let children = children()
.nodes
.into_iter()
.map(|child| view! { <li>{child}</li> })
.collect::<Vec<_>>();

    view! {
        <h1><code>"<WrapsChildren/>"</code></h1>
        // wrap our wrapped children in a UL
        <ul>{children}</ul>
    }
}

fn main() {
leptos::mount_to_body(App)
}


No Macros: The View Builder Syntax
If you’re perfectly happy with the view! macro syntax described so far, you’re welcome to skip this chapter. The builder syntax described in this section is always available, but never required.

For one reason or another, many developers would prefer to avoid macros. Perhaps you don’t like the limited rustfmt support. (Although, you should check out leptosfmt, which is an excellent tool!) Perhaps you worry about the effect of macros on compile time. Perhaps you prefer the aesthetics of pure Rust syntax, or you have trouble context-switching between an HTML-like syntax and your Rust code. Or perhaps you want more flexibility in how you create and manipulate HTML elements than the view macro provides.

If you fall into any of those camps, the builder syntax may be for you.

The view macro expands an HTML-like syntax to a series of Rust functions and method calls. If you’d rather not use the view macro, you can simply use that expanded syntax yourself. And it’s actually pretty nice!

First off, if you want you can even drop the #[component] macro: a component is just a setup function that creates your view, so you can define a component as a simple function call:

pub fn counter(initial_value: i32, step: u32) -> impl IntoView { }
Elements are created by calling a function with the same name as the HTML element:

p()
You can add children to the element with .child(), which takes a single child or a tuple or array of types that implement IntoView.

p().child((em().child("Big, "), strong().child("bold "), "text"))
Attributes are added with .attr(). This can take any of the same types that you could pass as an attribute into the view macro (types that implement IntoAttribute).

p().attr("id", "foo").attr("data-count", move || count().to_string())
Similarly, the class:, prop:, and style: syntaxes map directly onto .class(), .prop(), and .style() methods.

Event listeners can be added with .on(). Typed events found in leptos::ev prevent typos in event names and allow for correct type inference in the callback function.

button()
.on(ev::click, move |_| set_count.update(|count| *count = 0))
.child("Clear")
Many additional methods can be found in the HtmlElement docs, including some methods that are not directly available in the view macro.

All of this adds up to a very Rusty syntax to build full-featured views, if you prefer this style.

/// A simple counter view.
// A component is really just a function call: it runs once to create the DOM and reactive system
pub fn counter(initial_value: i32, step: u32) -> impl IntoView {
let (count, set_count) = create_signal(0);
div().child((
button()
// typed events found in leptos::ev
// 1) prevent typos in event names
// 2) allow for correct type inference in callbacks
.on(ev::click, move |_| set_count.update(|count| *count = 0))
.child("Clear"),
button()
.on(ev::click, move |_| set_count.update(|count| *count -= 1))
.child("-1"),
span().child(("Value: ", move || count.get(), "!")),
button()
.on(ev::click, move |_| set_count.update(|count| *count += 1))
.child("+1"),
))
}
This also has the benefit of being more flexible: because these are all plain Rust functions and methods, it’s easier to use them in things like iterator adapters without any additional “magic”:

// take some set of attribute names and values
let attrs: Vec<(&str, AttributeValue)> = todo!();
// you can use the builder syntax to “spread” these onto the
// element in a way that’s not possible with the view macro
let p = attrs
.into_iter()
.fold(p(), |el, (name, value)| el.attr(name, value));
Performance Note
One caveat: the view macro applies significant optimizations in server-side-rendering (SSR) mode to improve HTML rendering performance significantly (think 2-4x faster, depending on the characteristics of any given app). It does this by analyzing your view at compile time and converting the static parts into simple HTML strings, rather than expanding them into the builder syntax.

This means two things:

The builder syntax and view macro should not be mixed, or should only be mixed very carefully: at least in SSR mode, the output of the view should be treated as a “black box” that can’t have additional builder methods applied to it without causing inconsistencies.
Using the builder syntax will result in less-than-optimal SSR performance. It won’t be slow, by any means (and it’s worth running your own benchmarks in any case), just slower than the view-optimized version.


Reactivity
Leptos is built on top of a fine-grained reactive system, designed to run expensive side effects (like rendering something in a browser, or making a network request) as infrequently as possible in response to change, reactive values.

So far we’ve seen signals in action. These chapters will go into a bit more depth, and look at effects, which are the other half of the story.


Working with Signals
So far we’ve used some simple examples of create_signal, which returns a ReadSignal getter and a WriteSignal setter.

Getting and Setting
There are four basic signal operations:

.get() clones the current value of the signal and tracks any future changes to the value reactively.
.with() takes a function, which receives the current value of the signal by reference (&T), and tracks any future changes.
.set() replaces the current value of the signal and notifies any subscribers that they need to update.
.update() takes a function, which receives a mutable reference to the current value of the signal (&mut T), and notifies any subscribers that they need to update. (.update() doesn’t return the value returned by the closure, but you can use .try_update() if you need to; for example, if you’re removing an item from a Vec<_> and want the removed item.)
Calling a ReadSignal as a function is syntax sugar for .get(). Calling a WriteSignal as a function is syntax sugar for .set(). So

let (count, set_count) = create_signal(0);
set_count(1);
logging::log!(count());
is the same as

let (count, set_count) = create_signal(0);
set_count.set(1);
logging::log!(count.get());
You might notice that .get() and .set() can be implemented in terms of .with() and .update(). In other words, count.get() is identical with count.with(|n| n.clone()), and count.set(1) is implemented by doing count.update(|n| *n = 1).

But of course, .get() and .set() (or the plain function-call forms!) are much nicer syntax.

However, there are some very good use cases for .with() and .update().

For example, consider a signal that holds a Vec<String>.

let (names, set_names) = create_signal(Vec::new());
if names().is_empty() {
set_names(vec!["Alice".to_string()]);
}
In terms of logic, this is simple enough, but it’s hiding some significant inefficiencies. Remember that names().is_empty() is sugar for names.get().is_empty(), which clones the value (it’s names.with(|n| n.clone()).is_empty()). This means we clone the whole Vec<String>, run is_empty(), and then immediately throw away the clone.

Likewise, set_names replaces the value with a whole new Vec<_>. This is fine, but we might as well just mutate the original Vec<_> in place.

let (names, set_names) = create_signal(Vec::new());
if names.with(|names| names.is_empty()) {
set_names.update(|names| names.push("Alice".to_string()));
}
Now our function simply takes names by reference to run is_empty(), avoiding that clone.

And if you have Clippy on, or if you have sharp eyes, you may notice we can make this even neater:

if names.with(Vec::is_empty) {
// ...
}
After all, .with() simply takes a function that takes the value by reference. Since Vec::is_empty takes &self, we can pass it in directly and avoid the unnecessary closure.

There are some helper macros to make using .with() and .update() easier to use, especially when using multiple signals.

let (first, _) = create_signal("Bob".to_string());
let (middle, _) = create_signal("J.".to_string());
let (last, _) = create_signal("Smith".to_string());
If you wanted to concatenate these 3 signals together without unnecessary cloning, you would have to write something like:

let name = move || {
first.with(|first| {
middle.with(|middle| last.with(|last| format!("{first} {middle} {last}")))
})
};
Which is very long and annoying to write.

Instead, you can use the with! macro to get references to all the signals at the same time.

let name = move || with!(|first, middle, last| format!("{first} {middle} {last}"));
This expands to the same thing as above. Take a look at the with! docs for more info, and the corresponding macros update!, with_value! and update_value!.

Making signals depend on each other
Often people ask about situations in which some signal needs to change based on some other signal’s value. There are three good ways to do this, and one that’s less than ideal but okay under controlled circumstances.

Good Options
1) B is a function of A. Create a signal for A and a derived signal or memo for B.

let (count, set_count) = create_signal(1);
let derived_signal_double_count = move || count() * 2;
let memoized_double_count = create_memo(move |_| count() * 2);
For guidance on whether to use a derived signal or a memo, see the docs for create_memo

2) C is a function of A and some other thing B. Create signals for A and B and a derived signal or memo for C.

let (first_name, set_first_name) = create_signal("Bridget".to_string());
let (last_name, set_last_name) = create_signal("Jones".to_string());
let full_name = move || with!(|first_name, last_name| format!("{first_name} {last_name}"));
3) A and B are independent signals, but sometimes updated at the same time. When you make the call to update A, make a separate call to update B.

let (age, set_age) = create_signal(32);
let (favorite_number, set_favorite_number) = create_signal(42);
// use this to handle a click on a `Clear` button
let clear_handler = move |_| {
set_age(0);
set_favorite_number(0);
};
If you really must...
4) Create an effect to write to B whenever A changes. This is officially discouraged, for several reasons: a) It will always be less efficient, as it means every time A updates you do two full trips through the reactive process. (You set A, which causes the effect to run, as well as any other effects that depend on A. Then you set B, which causes any effects that depend on B to run.) b) It increases your chances of accidentally creating things like infinite loops or over-re-running effects. This is the kind of ping-ponging, reactive spaghetti code that was common in the early 2010s and that we try to avoid with things like read-write segregation and discouraging writing to signals from effects.

In most situations, it’s best to rewrite things such that there’s a clear, top-down data flow based on derived signals or memos. But this isn’t the end of the world.

I’m intentionally not providing an example here. Read the create_effect docs to figure out how this would work.

Function leptos::create_effectCopy item path
source · [−]
pub fn create_effect<T>(f: impl Fn(Option<T>) -> T + 'static) -> Effect<T>
where
T: 'static,
Effects run a certain chunk of code whenever the signals they depend on change. create_effect queues the given function to run once, tracks its dependence on any signal values read within it, and reruns the function whenever the value of a dependency changes.

Effects are intended to run side-effects of the system, not to synchronize state within the system. In other words: don’t write to signals within effects, unless you’re coordinating with some other non-reactive side effect. (If you need to define a signal that depends on the value of other signals, use a derived signal or create_memo).

This first run is queued for the next microtask, i.e., it runs after all other synchronous code has completed. In practical terms, this means that if you use create_effect in the body of the component, it will run after the view has been created and (presumably) mounted. (If you need an effect that runs immediately, use create_render_effect.)

The effect function is called with an argument containing whatever value it returned the last time it ran. On the initial run, this is None.

By default, effects do not run on the server. This means you can call browser-specific APIs within the effect function without causing issues. If you need an effect to run on the server, use create_isomorphic_effect.

let (a, set_a) = create_signal(0);
let (b, set_b) = create_signal(0);

// ✅ use effects to interact between reactive state and the outside world
create_effect(move |_| {
// immediately prints "Value: 0" and subscribes to `a`
log::debug!("Value: {}", a.get());
});

set_a.set(1);
// ✅ because it's subscribed to `a`, the effect reruns and prints "Value: 1"

// ❌ don't use effects to synchronize state within the reactive system
create_effect(move |_| {
// this technically works but can cause unnecessary re-renders
// and easily lead to problems like infinite loops
set_b.set(a.get() + 1);
});


Responding to Changes with create_effect
We’ve made it this far without having mentioned half of the reactive system: effects.

Reactivity works in two halves: updating individual reactive values (“signals”) notifies the pieces of code that depend on them (“effects”) that they need to run again. These two halves of the reactive system are inter-dependent. Without effects, signals can change within the reactive system but never be observed in a way that interacts with the outside world. Without signals, effects run once but never again, as there’s no observable value to subscribe to. Effects are quite literally “side effects” of the reactive system: they exist to synchronize the reactive system with the non-reactive world outside it.

Hidden behind the whole reactive DOM renderer that we’ve seen so far is a function called create_effect.

create_effect takes a function as its argument. It immediately runs the function. If you access any reactive signal inside that function, it registers the fact that the effect depends on that signal with the reactive runtime. Whenever one of the signals that the effect depends on changes, the effect runs again.

let (a, set_a) = create_signal(0);
let (b, set_b) = create_signal(0);

create_effect(move |_| {
// immediately prints "Value: 0" and subscribes to `a`
log::debug!("Value: {}", a());
});
The effect function is called with an argument containing whatever value it returned the last time it ran. On the initial run, this is None.

By default, effects do not run on the server. This means you can call browser-specific APIs within the effect function without causing issues. If you need an effect to run on the server, use create_isomorphic_effect.

Auto-tracking and Dynamic Dependencies
If you’re familiar with a framework like React, you might notice one key difference. React and similar frameworks typically require you to pass a “dependency array,” an explicit set of variables that determine when the effect should rerun.

Because Leptos comes from the tradition of synchronous reactive programming, we don’t need this explicit dependency list. Instead, we automatically track dependencies depending on which signals are accessed within the effect.

This has two effects (no pun intended). Dependencies are:

Automatic: You don’t need to maintain a dependency list, or worry about what should or shouldn’t be included. The framework simply tracks which signals might cause the effect to rerun, and handles it for you.
Dynamic: The dependency list is cleared and updated every time the effect runs. If your effect contains a conditional (for example), only signals that are used in the current branch are tracked. This means that effects rerun the absolute minimum number of times.
If this sounds like magic, and if you want a deep dive into how automatic dependency tracking works, check out this video. (Apologies for the low volume!)

Effects as Zero-Cost-ish Abstraction
While they’re not a “zero-cost abstraction” in the most technical sense—they require some additional memory use, exist at runtime, etc.—at a higher level, from the perspective of whatever expensive API calls or other work you’re doing within them, effects are a zero-cost abstraction. They rerun the absolute minimum number of times necessary, given how you’ve described them.

Imagine that I’m creating some kind of chat software, and I want people to be able to display their full name, or just their first name, and to notify the server whenever their name changes:

let (first, set_first) = create_signal(String::new());
let (last, set_last) = create_signal(String::new());
let (use_last, set_use_last) = create_signal(true);

// this will add the name to the log
// any time one of the source signals changes
create_effect(move |_| {
log(
if use_last() {
format!("{} {}", first(), last())
} else {
first()
},
)
});
If use_last is true, effect should rerun whenever first, last, or use_last changes. But if I toggle use_last to false, a change in last will never cause the full name to change. In fact, last will be removed from the dependency list until use_last toggles again. This saves us from sending multiple unnecessary requests to the API if I change last multiple times while use_last is still false.

To create_effect, or not to create_effect?
Effects are intended to synchronize the reactive system with the non-reactive world outside, not to synchronize between different reactive values. In other words: using an effect to read a value from one signal and set it in another is always sub-optimal.

If you need to define a signal that depends on the value of other signals, use a derived signal or create_memo. Writing to a signal inside an effect isn’t the end of the world, and it won’t cause your computer to light on fire, but a derived signal or memo is always better—not only because the dataflow is clear, but because the performance is better.

let (a, set_a) = create_signal(0);

// ⚠️ not great
let (b, set_b) = create_signal(0);
create_effect(move |_| {
set_b(a() * 2);
});

// ✅ woo-hoo!
let b = move || a() * 2;
If you need to synchronize some reactive value with the non-reactive world outside—like a web API, the console, the filesystem, or the DOM—writing to a signal in an effect is a fine way to do that. In many cases, though, you’ll find that you’re really writing to a signal inside an event listener or something else, not inside an effect. In these cases, you should check out leptos-use to see if it already provides a reactive wrapping primitive to do that!

If you’re curious for more information about when you should and shouldn’t use create_effect, check out this video for a more in-depth consideration!

Effects and Rendering
We’ve managed to get this far without mentioning effects because they’re built into the Leptos DOM renderer. We’ve seen that you can create a signal and pass it into the view macro, and it will update the relevant DOM node whenever the signal changes:

let (count, set_count) = create_signal(0);

view! {
<p>{count}</p>
}
This works because the framework essentially creates an effect wrapping this update. You can imagine Leptos translating this view into something like this:

let (count, set_count) = create_signal(0);

// create a DOM element
let document = leptos::document();
let p = document.create_element("p").unwrap();

// create an effect to reactively update the text
create_effect(move |prev_value| {
// first, access the signal’s value and convert it to a string
let text = count().to_string();

    // if this is different from the previous value, update the node
    if prev_value != Some(text) {
        p.set_text_content(&text);
    }

    // return this value so we can memoize the next update
    text
});
Every time count is updated, this effect will rerun. This is what allows reactive, fine-grained updates to the DOM.

Explicit, Cancelable Tracking with watch
In addition to create_effect, Leptos provides a watch function, which can be used for two main purposes:

Separating tracking and responding to changes by explicitly passing in a set of values to track.
Canceling tracking by calling a stop function.
Like create_resource, watch takes a first argument, which is reactively tracked, and a second, which is not. Whenever a reactive value in its deps argument is changed, the callback is run. watch returns a function that can be called to stop tracking the dependencies.

let (num, set_num) = create_signal(0);

let stop = watch(
move || num.get(),
move |num, prev_num, _| {
log::debug!("Number: {}; Prev: {:?}", num, prev_num);
},
false,
);

set_num.set(1); // > "Number: 1; Prev: Some(0)"

stop(); // stop watching

set_num.set(2); // (nothing happens)

other code

use leptos::html::Input;
use leptos::*;

#[derive(Copy, Clone)]
struct LogContext(RwSignal<Vec<String>>);

#[component]
fn App() -> impl IntoView {
// Just making a visible log here
// You can ignore this...
let log = create_rw_signal::<Vec<String>>(vec![]);
let logged = move || log().join("\n");

    // the newtype pattern isn't *necessary* here but is a good practice
    // it avoids confusion with other possible future `RwSignal<Vec<String>>` contexts
    // and makes it easier to refer to it
    provide_context(LogContext(log));

    view! {
        <CreateAnEffect/>
        <pre>{logged}</pre>
    }
}

#[component]
fn CreateAnEffect() -> impl IntoView {
let (first, set_first) = create_signal(String::new());
let (last, set_last) = create_signal(String::new());
let (use_last, set_use_last) = create_signal(true);

    // this will add the name to the log
    // any time one of the source signals changes
    create_effect(move |_| {
        log(if use_last() {
            with!(|first, last| format!("{first} {last}"))
        } else {
            first()
        })
    });

    view! {
        <h1>
            <code>"create_effect"</code>
            " Version"
        </h1>
        <form>
            <label>
                "First Name"
                <input
                    type="text"
                    name="first"
                    prop:value=first
                    on:change=move |ev| set_first(event_target_value(&ev))
                />
            </label>
            <label>
                "Last Name"
                <input
                    type="text"
                    name="last"
                    prop:value=last
                    on:change=move |ev| set_last(event_target_value(&ev))
                />
            </label>
            <label>
                "Show Last Name"
                <input
                    type="checkbox"
                    name="use_last"
                    prop:checked=use_last
                    on:change=move |ev| set_use_last(event_target_checked(&ev))
                />
            </label>
        </form>
    }
}

#[component]
fn ManualVersion() -> impl IntoView {
let first = create_node_ref::<Input>();
let last = create_node_ref::<Input>();
let use_last = create_node_ref::<Input>();

    let mut prev_name = String::new();
    let on_change = move |_| {
        log("      listener");
        let first = first.get().unwrap();
        let last = last.get().unwrap();
        let use_last = use_last.get().unwrap();
        let this_one = if use_last.checked() {
            format!("{} {}", first.value(), last.value())
        } else {
            first.value()
        };

        if this_one != prev_name {
            log(&this_one);
            prev_name = this_one;
        }
    };

    view! {
        <h1>"Manual Version"</h1>
        <form on:change=on_change>
            <label>"First Name" <input type="text" name="first" node_ref=first/></label>
            <label>"Last Name" <input type="text" name="last" node_ref=last/></label>
            <label>
                "Show Last Name" <input type="checkbox" name="use_last" checked node_ref=use_last/>
            </label>
        </form>
    }
}

#[component]
fn EffectVsDerivedSignal() -> impl IntoView {
let (my_value, set_my_value) = create_signal(String::new());
// Don't do this.
/*let (my_optional_value, set_optional_my_value) = create_signal(Option::<String>::None);

    create_effect(move |_| {
        if !my_value.get().is_empty() {
            set_optional_my_value(Some(my_value.get()));
        } else {
            set_optional_my_value(None);
        }
    });*/

    // Do this
    let my_optional_value =
        move || (!my_value.with(String::is_empty)).then(|| Some(my_value.get()));

    view! {
        <input prop:value=my_value on:input=move |ev| set_my_value(event_target_value(&ev))/>

        <p>
            <code>"my_optional_value"</code>
            " is "
            <code>
                <Show when=move || my_optional_value().is_some() fallback=|| view! { "None" }>
                    "Some(\""
                    {my_optional_value().unwrap()}
                    "\")"
                </Show>
            </code>
        </p>
    }
}

#[component]
pub fn Show<F, W, IV>(
/// The components Show wraps
children: Box<dyn Fn() -> Fragment>,
/// A closure that returns a bool that determines whether this thing runs
when: W,
/// A closure that returns what gets rendered if the when statement is false
fallback: F,
) -> impl IntoView
where
W: Fn() -> bool + 'static,
F: Fn() -> IV + 'static,
IV: IntoView,
{
let memoized_when = create_memo(move |_| when());

    move || match memoized_when.get() {
        true => children().into_view(),
        false => fallback().into_view(),
    }
}

fn log(msg: impl std::fmt::Display) {
let log = use_context::<LogContext>().unwrap().0;
log.update(|log| log.push(msg.to_string()));
}

fn main() {
leptos::mount_to_body(App)
}


Interlude: Reactivity and Functions
One of our core contributors said to me recently: “I never used closures this often until I started using Leptos.” And it’s true. Closures are at the heart of any Leptos application. It sometimes looks a little silly:

// a signal holds a value, and can be updated
let (count, set_count) = create_signal(0);

// a derived signal is a function that accesses other signals
let double_count = move || count() * 2;
let count_is_odd = move || count() & 1 == 1;
let text = move || if count_is_odd() {
"odd"
} else {
"even"
};

// an effect automatically tracks the signals it depends on
// and reruns when they change
create_effect(move |_| {
logging::log!("text = {}", text());
});

view! {
<p>{move || text().to_uppercase()}</p>
}
Closures, closures everywhere!

But why?

Functions and UI Frameworks
Functions are at the heart of every UI framework. And this makes perfect sense. Creating a user interface is basically divided into two phases:

initial rendering
updates
In a web framework, the framework does some kind of initial rendering. Then it hands control back over to the browser. When certain events fire (like a mouse click) or asynchronous tasks finish (like an HTTP request finishing), the browser wakes the framework back up to update something. The framework runs some kind of code to update your user interface, and goes back asleep until the browser wakes it up again.

The key phrase here is “runs some kind of code.” The natural way to “run some kind of code” at an arbitrary point in time—in Rust or in any other programming language—is to call a function. And in fact every UI framework is based on rerunning some kind of function over and over:

virtual DOM (VDOM) frameworks like React, Yew, or Dioxus rerun a component or render function over and over, to generate a virtual DOM tree that can be reconciled with the previous result to patch the DOM
compiled frameworks like Angular and Svelte divide your component templates into “create” and “update” functions, rerunning the update function when they detect a change to the component’s state
in fine-grained reactive frameworks like SolidJS, Sycamore, or Leptos, you define the functions that rerun
That’s what all our components are doing.

Take our typical <SimpleCounter/> example in its simplest form:

#[component]
pub fn SimpleCounter() -> impl IntoView {
let (value, set_value) = create_signal(0);

    let increment = move |_| set_value.update(|value| *value += 1);

    view! {
        <button on:click=increment>
            {value}
        </button>
    }
}
The SimpleCounter function itself runs once. The value signal is created once. The framework hands off the increment function to the browser as an event listener. When you click the button, the browser calls increment, which updates value via set_value. And that updates the single text node represented in our view by {value}.

Closures are key to reactivity. They provide the framework with the ability to rerun the smallest possible unit of your application in response to a change.

So remember two things:

Your component function is a setup function, not a render function: it only runs once.
For values in your view template to be reactive, they must be functions: either signals (which implement the Fn traits) or closures.


Testing Your Components
Testing user interfaces can be relatively tricky, but really important. This article will discuss a couple principles and approaches for testing a Leptos app.

1. Test business logic with ordinary Rust tests
   In many cases, it makes sense to pull the logic out of your components and test it separately. For some simple components, there’s no particular logic to test, but for many it’s worth using a testable wrapping type and implementing the logic in ordinary Rust impl blocks.

For example, instead of embedding logic in a component directly like this:

#[component]
pub fn TodoApp() -> impl IntoView {
let (todos, set_todos) = create_signal(vec![Todo { /* ... */ }]);
// ⚠️ this is hard to test because it's embedded in the component
let num_remaining = move || todos.with(|todos| {
todos.iter().filter(|todo| !todo.completed).sum()
});
}
You could pull that logic out into a separate data structure and test it:

pub struct Todos(Vec<Todo>);

impl Todos {
pub fn num_remaining(&self) -> usize {
self.0.iter().filter(|todo| !todo.completed).sum()
}
}

#[cfg(test)]
mod tests {
#[test]
fn test_remaining() {
// ...
}
}

#[component]
pub fn TodoApp() -> impl IntoView {
let (todos, set_todos) = create_signal(Todos(vec![Todo { /* ... */ }]));
// ✅ this has a test associated with it
let num_remaining = move || todos.with(Todos::num_remaining);
}
In general, the less of your logic is wrapped into your components themselves, the more idiomatic your code will feel and the easier it will be to test.

2. Test components with end-to-end (e2e) testing
   Our examples directory has several examples with extensive end-to-end testing, using different testing tools.

The easiest way to see how to use these is to take a look at the test examples themselves:

wasm-bindgen-test with counter
This is a fairly simple manual testing setup that uses the wasm-pack test command.

Sample Test
#[wasm_bindgen_test]
fn clear() {
let document = leptos::document();
let test_wrapper = document.create_element("section").unwrap();
let _ = document.body().unwrap().append_child(&test_wrapper);

    mount_to(
        test_wrapper.clone().unchecked_into(),
        || view! { <SimpleCounter initial_value=10 step=1/> },
    );

    let div = test_wrapper.query_selector("div").unwrap().unwrap();
    let clear = test_wrapper
        .query_selector("button")
        .unwrap()
        .unwrap()
        .unchecked_into::<web_sys::HtmlElement>();

    clear.click();

assert_eq!(
div.outer_html(),
// here we spawn a mini reactive system to render the test case
run_scope(create_runtime(), || {
// it's as if we're creating it with a value of 0, right?
let (value, set_value) = create_signal(0);

        // we can remove the event listeners because they're not rendered to HTML
        view! {
            <div>
                <button>"Clear"</button>
                <button>"-1"</button>
                <span>"Value: " {value} "!"</span>
                <button>"+1"</button>
            </div>
        }
        // the view returned an HtmlElement<Div>, which is a smart pointer for
        // a DOM element. So we can still just call .outer_html()
        .outer_html()
    })
);
}
wasm-bindgen-test with counters_stable
This more developed test suite uses a system of fixtures to refactor the manual DOM manipulation of the counter tests and easily test a wide range of cases.

Sample Test
use super::*;
use crate::counters_page as ui;
use pretty_assertions::assert_eq;

#[wasm_bindgen_test]
fn should_increase_the_total_count() {
// Given
ui::view_counters();
ui::add_counter();

    // When
    ui::increment_counter(1);
    ui::increment_counter(1);
    ui::increment_counter(1);

    // Then
    assert_eq!(ui::total(), 3);
}
Playwright with counters_stable
These tests use the common JavaScript testing tool Playwright to run end-to-end tests on the same example, using a library and testing approach familiar to many who have done frontend development before.

Sample Test
import { test, expect } from "@playwright/test";
import { CountersPage } from "./fixtures/counters_page";

test.describe("Increment Count", () => {
test("should increase the total count", async ({ page }) => {
const ui = new CountersPage(page);
await ui.goto();
await ui.addCounter();

    await ui.incrementCount();
    await ui.incrementCount();
    await ui.incrementCount();

    await expect(ui.total).toHaveText("3");
});
});
Gherkin/Cucumber Tests with todo_app_sqlite
You can integrate any testing tool you’d like into this flow. This example uses Cucumber, a testing framework based on natural language.

@add_todo
Feature: Add Todo

    Background:
        Given I see the app

    @add_todo-see
    Scenario: Should see the todo
        Given I set the todo as Buy Bread
        When I click the Add button
        Then I see the todo named Buy Bread

    # @allow.skipped
    @add_todo-style
    Scenario: Should see the pending todo
        When I add a todo as Buy Oranges
        Then I see the pending todo
The definitions for these actions are defined in Rust code.

use crate::fixtures::{action, world::AppWorld};
use anyhow::{Ok, Result};
use cucumber::{given, when};

#[given("I see the app")]
#[when("I open the app")]
async fn i_open_the_app(world: &mut AppWorld) -> Result<()> {
let client = &world.client;
action::goto_path(client, "").await?;

    Ok(())
}

#[given(regex = "^I add a todo as (.*)$")]
#[when(regex = "^I add a todo as (.*)$")]
async fn i_add_a_todo_titled(world: &mut AppWorld, text: String) -> Result<()> {
let client = &world.client;
action::add_todo(client, text.as_str()).await?;

    Ok(())
}

// etc.
Learning More
Feel free to check out the CI setup in the Leptos repo to learn more about how to use these tools in your own application. All of these testing methods are run regularly against actual Leptos example apps.


Working with async
So far we’ve only been working with synchronous user interfaces: You provide some input, the app immediately processes it and updates the interface. This is great, but is a tiny subset of what web applications do. In particular, most web apps have to deal with some kind of asynchronous data loading, usually loading something from an API.

Asynchronous data is notoriously hard to integrate with the synchronous parts of your code. Leptos provides a cross-platform spawn_local function that makes it easy to run a Future, but there’s much more to it than that.

In this chapter, we’ll see how Leptos helps smooth out that process for you.


Loading Data with Resources
A Resource is a reactive data structure that reflects the current state of an asynchronous task, allowing you to integrate asynchronous Futures into the synchronous reactive system. Rather than waiting for its data to load with .await, you transform the Future into a signal that returns Some(T) if it has resolved, and None if it’s still pending.

You do this by using the create_resource function. This takes two arguments:

a source signal, which will generate a new Future whenever it changes
a fetcher function, which takes the data from that signal and returns a Future
Here’s an example

// our source signal: some synchronous, local state
let (count, set_count) = create_signal(0);

// our resource
let async_data = create_resource(
count,
// every time `count` changes, this will run
|value| async move {
logging::log!("loading data from API");
load_data(value).await
},
);
To create a resource that simply runs once, you can pass a non-reactive, empty source signal:

let once = create_resource(|| (), |_| async move { load_data().await });
To access the value you can use .get() or .with(|data| /* */). These work just like .get() and .with() on a signal—get clones the value and returns it, with applies a closure to it—but for any Resource<_, T>, they always return Option<T>, not T: because it’s always possible that your resource is still loading.

So, you can show the current state of a resource in your view:

let once = create_resource(|| (), |_| async move { load_data().await });
view! {
<h1>"My Data"</h1>
{move || match once.get() {
None => view! { <p>"Loading..."</p> }.into_view(),
Some(data) => view! { <ShowData data/> }.into_view()
}}
}
Resources also provide a refetch() method that allows you to manually reload the data (for example, in response to a button click) and a loading() method that returns a ReadSignal<bool> indicating whether the resource is currently loading or not.

use gloo_timers::future::TimeoutFuture;
use leptos::*;

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Here, we just multiply a number by 10
async fn load_data(value: i32) -> i32 {
// fake a one-second delay
TimeoutFuture::new(1_000).await;
value * 10
}

#[component]
fn App() -> impl IntoView {
// this count is our synchronous, local state
let (count, set_count) = create_signal(0);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        // the first is the "source signal"
        count,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move { load_data(value).await },
    );
    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on anything: we just load it once
    let stable = create_resource(|| (), |_| async move { load_data(1).await });

    // we can access the resource values with .read()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .read()
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    // the resource's loading() method gives us a
    // signal to indicate whether it's currently loading
    let loading = async_data.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me"
        </button>
        <p>
            <code>"stable"</code>": " {move || stable.read()}
        </p>
        <p>
            <code>"count"</code>": " {count}
        </p>
        <p>
            <code>"async_value"</code>": "
            {async_result}
            <br/>
            {is_loading}
        </p>
    }
}

fn main() {
leptos::mount_to_body(App)
}


<Suspense/>
In the previous chapter, we showed how you can create a simple loading screen to show some fallback while a resource is loading.

let (count, set_count) = create_signal(0);
let once = create_resource(count, |count| async move { load_a(count).await });

view! {
<h1>"My Data"</h1>
{move || match once.get() {
None => view! { <p>"Loading..."</p> }.into_view(),
Some(data) => view! { <ShowData data/> }.into_view()
}}
}
But what if we have two resources, and want to wait for both of them?

let (count, set_count) = create_signal(0);
let (count2, set_count2) = create_signal(0);
let a = create_resource(count, |count| async move { load_a(count).await });
let b = create_resource(count2, |count| async move { load_b(count).await });

view! {
<h1>"My Data"</h1>
{move || match (a.get(), b.get()) {
(Some(a), Some(b)) => view! {
<ShowA a/>
<ShowA b/>
}.into_view(),
_ => view! { <p>"Loading..."</p> }.into_view()
}}
}
That’s not so bad, but it’s kind of annoying. What if we could invert the flow of control?

The <Suspense/> component lets us do exactly that. You give it a fallback prop and children, one or more of which usually involves reading from a resource. Reading from a resource “under” a <Suspense/> (i.e., in one of its children) registers that resource with the <Suspense/>. If it’s still waiting for resources to load, it shows the fallback. When they’ve all loaded, it shows the children.

let (count, set_count) = create_signal(0);
let (count2, set_count2) = create_signal(0);
let a = create_resource(count, |count| async move { load_a(count).await });
let b = create_resource(count2, |count| async move { load_b(count).await });

view! {
<h1>"My Data"</h1>
<Suspense
fallback=move || view! { <p>"Loading..."</p> }
>
<h2>"My Data"</h2>
<h3>"A"</h3>
{move || {
a.get()
.map(|a| view! { <ShowA a/> })
}}
<h3>"B"</h3>
{move || {
b.get()
.map(|b| view! { <ShowB b/> })
}}
</Suspense>
}
Every time one of the resources is reloading, the "Loading..." fallback will show again.

This inversion of the flow of control makes it easier to add or remove individual resources, as you don’t need to handle the matching yourself. It also unlocks some massive performance improvements during server-side rendering, which we’ll talk about during a later chapter.

<Await/>
If you’re simply trying to wait for some Future to resolve before rendering, you may find the <Await/> component helpful in reducing boilerplate. <Await/> essentially combines a resource with the source argument || () with a <Suspense/> with no fallback.

In other words:

It only polls the Future once, and does not respond to any reactive changes.
It does not render anything until the Future resolves.
After the Future resolves, it binds its data to whatever variable name you choose and then renders its children with that variable in scope.
async fn fetch_monkeys(monkey: i32) -> i32 {
// maybe this didn't need to be async
monkey * 2
}
view! {
<Await
// `future` provides the `Future` to be resolved
future=|| fetch_monkeys(3)
// the data is bound to whatever variable name you provide
let:data
>
// you receive the data by reference and can use it in your view here
<p>{*data} " little monkeys, jumping on the bed."</p>
</Await>
}

other code

use gloo_timers::future::TimeoutFuture;
use leptos::*;

async fn important_api_call(name: String) -> String {
TimeoutFuture::new(1_000).await;
name.to_ascii_uppercase()
}

#[component]
fn App() -> impl IntoView {
let (name, set_name) = create_signal("Bill".to_string());

    // this will reload every time `name` changes
    let async_data = create_resource(name, |name| async move { important_api_call(name).await });

    view! {
        <input
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }
            prop:value=name
        />
        <p><code>"name:"</code> {name}</p>
        <Suspense
            // the fallback will show whenever a resource
            // read "under" the suspense is loading
            fallback=move || view! { <p>"Loading..."</p> }
        >
            // the children will be rendered once initially,
            // and then whenever any resources has been resolved
            <p>
                "Your shouting name is "
                {move || async_data.get()}
            </p>
        </Suspense>
    }
}

fn main() {
leptos::mount_to_body(App)
}


<Transition/>
You’ll notice in the <Suspense/> example that if you keep reloading the data, it keeps flickering back to "Loading...". Sometimes this is fine. For other times, there’s <Transition/>.

<Transition/> behaves exactly the same as <Suspense/>, but instead of falling back every time, it only shows the fallback the first time. On all subsequent loads, it continues showing the old data until the new data are ready. This can be really handy to prevent the flickering effect, and to allow users to continue interacting with your application.

This example shows how you can create a simple tabbed contact list with <Transition/>. When you select a new tab, it continues showing the current contact until the new data loads. This can be a much better user experience than constantly falling back to a loading message.

use leptos::*;

async fn important_api_call(id: usize) -> String {
TimeoutFuture::new(1_000).await;
match id {
0 => "Alice",
1 => "Bob",
2 => "Carol",
_ => "User not found",
}
.to_string()
}

#[component]
fn App() -> impl IntoView {
let (tab, set_tab) = create_signal(0);

    // this will reload every time `tab` changes
    let user_data = create_resource(tab, |tab| async move { important_api_call(tab).await });

    view! {
        <div class="buttons">
            <button
                on:click=move |_| set_tab(0)
                class:selected=move || tab() == 0
            >
                "Tab A"
            </button>
            <button
                on:click=move |_| set_tab(1)
                class:selected=move || tab() == 1
            >
                "Tab B"
            </button>
            <button
                on:click=move |_| set_tab(2)
                class:selected=move || tab() == 2
            >
                "Tab C"
            </button>
        </div>
        <Transition
            // the fallback will show initially
            // on subsequent reloads, the current child will
            // continue showing
            fallback=move || view! { <p>"Loading initial data..."</p> }
        >
            <p>
                {move || user_data.get()}
            </p>
        </Transition>
        {move || if user_data.loading().get() {
            "Hang on..."
        } else {
            ""
        }}
    }
}

fn main() {
leptos::mount_to_body(App)
}


Mutating Data with Actions
We’ve talked about how to load async data with resources. Resources immediately load data and work closely with <Suspense/> and <Transition/> components to show whether data is loading in your app. But what if you just want to call some arbitrary async function and keep track of what it’s doing?

Well, you could always use spawn_local. This allows you to just spawn an async task in a synchronous environment by handing the Future off to the browser (or, on the server, Tokio or whatever other runtime you’re using). But how do you know if it’s still pending? Well, you could just set a signal to show whether it’s loading, and another one to show the result...

All of this is true. Or you could use the final async primitive: create_action.

Actions and resources seem similar, but they represent fundamentally different things. If you’re trying to load data by running an async function, either once or when some other value changes, you probably want to use create_resource. If you’re trying to occasionally run an async function in response to something like a user clicking a button, you probably want to use create_action.

Say we have some async function we want to run.

async fn add_todo_request(new_title: &str) -> Uuid {
/* do some stuff on the server to add a new todo */
}
create_action takes an async function that takes a reference to a single argument, which you could think of as its “input type.”

The input is always a single type. If you want to pass in multiple arguments, you can do it with a struct or tuple.

// if there's a single argument, just use that
let action1 = create_action(|input: &String| {
let input = input.clone();
async move { todo!() }
});

// if there are no arguments, use the unit type `()`
let action2 = create_action(|input: &()| async { todo!() });

// if there are multiple arguments, use a tuple
let action3 = create_action(
|input: &(usize, String)| async { todo!() }
);
Because the action function takes a reference but the Future needs to have a 'static lifetime, you’ll usually need to clone the value to pass it into the Future. This is admittedly awkward but it unlocks some powerful features like optimistic UI. We’ll see a little more about that in future chapters.

So in this case, all we need to do to create an action is

let add_todo_action = create_action(|input: &String| {
let input = input.to_owned();
async move { add_todo_request(&input).await }
});
Rather than calling add_todo_action directly, we’ll call it with .dispatch(), as in

add_todo_action.dispatch("Some value".to_string());
You can do this from an event listener, a timeout, or anywhere; because .dispatch() isn’t an async function, it can be called from a synchronous context.

Actions provide access to a few signals that synchronize between the asynchronous action you’re calling and the synchronous reactive system:

let submitted = add_todo_action.input(); // RwSignal<Option<String>>
let pending = add_todo_action.pending(); // ReadSignal<bool>
let todo_id = add_todo_action.value(); // RwSignal<Option<Uuid>>
This makes it easy to track the current state of your request, show a loading indicator, or do “optimistic UI” based on the assumption that the submission will succeed.

let input_ref = create_node_ref::<Input>();

view! {
<form
on:submit=move |ev| {
ev.prevent_default(); // don't reload the page...
let input = input_ref.get().expect("input to exist");
add_todo_action.dispatch(input.value());
}
>
<label>
"What do you need to do?"
<input type="text"
node_ref=input_ref
/>
</label>
<button type="submit">"Add Todo"</button>
</form>
// use our loading state
<p>{move || pending().then("Loading...")}</p>
}
Now, there’s a chance this all seems a little over-complicated, or maybe too restricted. I wanted to include actions here, alongside resources, as the missing piece of the puzzle. In a real Leptos app, you’ll actually most often use actions alongside server functions, create_server_action, and the <ActionForm/> component to create really powerful progressively-enhanced forms. So if this primitive seems useless to you... Don’t worry! Maybe it will make sense later. (Or check out our todo_app_sqlite example now.)

use gloo_timers::future::TimeoutFuture;
use leptos::{html::Input, *};
use uuid::Uuid;

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Think of it as a mutation: some imperative async action you run,
// whereas a resource would be some async data you load
async fn add_todo(text: &str) -> Uuid {
_ = text;
// fake a one-second delay
TimeoutFuture::new(1_000).await;
// pretend this is a post ID or something
Uuid::new_v4()
}

#[component]
fn App() -> impl IntoView {
// an action takes an async function with single argument
// it can be a simple type, a struct, or ()
let add_todo = create_action(|input: &String| {
// the input is a reference, but we need the Future to own it
// this is important: we need to clone and move into the Future
// so it has a 'static lifetime
let input = input.to_owned();
async move { add_todo(&input).await }
});

    // actions provide a bunch of synchronous, reactive variables
    // that tell us different things about the state of the action
    let submitted = add_todo.input();
    let pending = add_todo.pending();
    let todo_id = add_todo.value();

    let input_ref = create_node_ref::<Input>();

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default(); // don't reload the page...
                let input = input_ref.get().expect("input to exist");
                add_todo.dispatch(input.value());
            }
        >
            <label>
                "What do you need to do?"
                <input type="text"
                    node_ref=input_ref
                />
            </label>
            <button type="submit">"Add Todo"</button>
        </form>
        <p>{move || pending().then(|| "Loading...")}</p>
        <p>
            "Submitted: "
            <code>{move || format!("{:#?}", submitted())}</code>
        </p>
        <p>
            "Pending: "
            <code>{move || format!("{:#?}", pending())}</code>
        </p>
        <p>
            "Todo ID: "
            <code>{move || format!("{:#?}", todo_id())}</code>
        </p>
    }
}

fn main() {
leptos::mount_to_body(App)
}


Projecting Children
As you build components you may occasionally find yourself wanting to “project” children through multiple layers of components.

The Problem
Consider the following:

pub fn LoggedIn<F, IV>(fallback: F, children: ChildrenFn) -> impl IntoView
where
F: Fn() -> IV + 'static,
IV: IntoView,
{
view! {
<Suspense
fallback=|| ()
>
<Show
// check whether user is verified
// by reading from the resource
when=move || todo!()
fallback=fallback
>
{children()}
</Show>
</Suspense>
}
}
This is pretty straightforward: when the user is logged in, we want to show children. If the user is not logged in, we want to show fallback. And while we’re waiting to find out, we just render (), i.e., nothing.

In other words, we want to pass the children of <LoggedIn/> through the <Suspense/> component to become the children of the <Show/>. This is what I mean by “projection.”

This won’t compile.

error[E0507]: cannot move out of `fallback`, a captured variable in an `Fn` closure
error[E0507]: cannot move out of `children`, a captured variable in an `Fn` closure
The problem here is that both <Suspense/> and <Show/> need to be able to construct their children multiple times. The first time you construct <Suspense/>’s children, it would take ownership of fallback and children to move them into the invocation of <Show/>, but then they're not available for future <Suspense/> children construction.

The Details
Feel free to skip ahead to the solution.

If you want to really understand the issue here, it may help to look at the expanded view macro. Here’s a cleaned-up version:

Suspense(
::leptos::component_props_builder(&Suspense)
.fallback(|| ())
.children({
// fallback and children are moved into this closure
Box::new(move || {
{
// fallback and children captured here
leptos::Fragment::lazy(|| {
vec![
(Show(
::leptos::component_props_builder(&Show)
.when(|| true)
// but fallback is moved into Show here
.fallback(fallback)
// and children is moved into Show here
.children(children)
.build(),
)
.into_view()),
]
})
}
})
})
.build(),
)
All components own their props; so the <Show/> in this case can’t be called because it only has captured references to fallback and children.

Solution
However, both <Suspense/> and <Show/> take ChildrenFn, i.e., their children should implement the Fn type so they can be called multiple times with only an immutable reference. This means we don’t need to own children or fallback; we just need to be able to pass 'static references to them.

We can solve this problem by using the store_value primitive. This essentially stores a value in the reactive system, handing ownership off to the framework in exchange for a reference that is, like signals, Copy and 'static, which we can access or modify through certain methods.

In this case, it’s really simple:

pub fn LoggedIn<F, IV>(fallback: F, children: ChildrenFn) -> impl IntoView
where
F: Fn() -> IV + 'static,
IV: IntoView,
{
let fallback = store_value(fallback);
let children = store_value(children);
view! {
<Suspense
fallback=|| ()
>
<Show
when=|| todo!()
fallback=move || fallback.with_value(|fallback| fallback())
>
{children.with_value(|children| children())}
</Show>
</Suspense>
}
}
At the top level, we store both fallback and children in the reactive scope owned by LoggedIn. Now we can simply move those references down through the other layers into the <Show/> component and call them there.

A Final Note
Note that this works because <Show/> and <Suspense/> only need an immutable reference to their children (which .with_value can give it), not ownership.

In other cases, you may need to project owned props through a function that takes ChildrenFn and therefore needs to be called more than once. In this case, you may find the clone: helper in theview macro helpful.

Consider this example

#[component]
pub fn App() -> impl IntoView {
let name = "Alice".to_string();
view! {
<Outer>
<Inner>
<Inmost name=name.clone()/>
</Inner>
</Outer>
}
}

#[component]
pub fn Outer(children: ChildrenFn) -> impl IntoView {
children()
}

#[component]
pub fn Inner(children: ChildrenFn) -> impl IntoView {
children()
}

#[component]
pub fn Inmost(name: String) -> impl IntoView {
view! {
<p>{name}</p>
}
}
Even with name=name.clone(), this gives the error

cannot move out of `name`, a captured variable in an `Fn` closure
It’s captured through multiple levels of children that need to run more than once, and there’s no obvious way to clone it into the children.

In this case, the clone: syntax comes in handy. Calling clone:name will clone name before moving it into <Inner/>’s children, which solves our ownership issue.

view! {
<Outer>
<Inner clone:name>
<Inmost name=name.clone()/>
</Inner>
</Outer>
}
These issues can be a little tricky to understand or debug, because of the opacity of the view macro. But in general, they can always be solved.


Global State Management
So far, we've only been working with local state in components, and we’ve seen how to coordinate state between parent and child components. On occasion, there are times where people look for a more general solution for global state management that can work throughout an application.

In general, you do not need this chapter. The typical pattern is to compose your application out of components, each of which manages its own local state, not to store all state in a global structure. However, there are some cases (like theming, saving user settings, or sharing data between components in different parts of your UI) in which you may want to use some kind of global state management.

The three best approaches to global state are

Using the router to drive global state via the URL
Passing signals through context
Creating a global state struct and creating lenses into it with create_slice
Option #1: URL as Global State
In many ways, the URL is actually the best way to store global state. It can be accessed from any component, anywhere in your tree. There are native HTML elements like <form> and <a> that exist solely to update the URL. And it persists across page reloads and between devices; you can share a URL with a friend or send it from your phone to your laptop and any state stored in it will be replicated.

The next few sections of the tutorial will be about the router, and we’ll get much more into these topics.

But for now, we'll just look at options #2 and #3.

Option #2: Passing Signals through Context
In the section on parent-child communication, we saw that you can use provide_context to pass signal from a parent component to a child, and use_context to read it in the child. But provide_context works across any distance. If you want to create a global signal that holds some piece of state, you can provide it and access it via context anywhere in the descendants of the component where you provide it.

A signal provided via context only causes reactive updates where it is read, not in any of the components in between, so it maintains the power of fine-grained reactive updates, even at a distance.

We start by creating a signal in the root of the app and providing it to all its children and descendants using provide_context.

#[component]
fn App() -> impl IntoView {
// here we create a signal in the root that can be consumed
// anywhere in the app.
let (count, set_count) = create_signal(0);
// we'll pass the setter to specific components,
// but provide the count itself to the whole app via context
provide_context(count);

    view! {
        // SetterButton is allowed to modify the count
        <SetterButton set_count/>
        // These consumers can only read from it
        // But we could give them write access by passing `set_count` if we wanted
        <FancyMath/>
        <ListItems/>
    }
}
<SetterButton/> is the kind of counter we’ve written several times now. (See the sandbox below if you don’t understand what I mean.)

<FancyMath/> and <ListItems/> both consume the signal we’re providing via use_context and do something with it.

/// A component that does some "fancy" math with the global count
#[component]
fn FancyMath() -> impl IntoView {
// here we consume the global count signal with `use_context`
let count = use_context::<ReadSignal<u32>>()
// we know we just provided this in the parent component
.expect("there to be a `count` signal provided");
let is_even = move || count() & 1 == 0;

    view! {
        <div class="consumer blue">
            "The number "
            <strong>{count}</strong>
            {move || if is_even() {
                " is"
            } else {
                " is not"
            }}
            " even."
        </div>
    }
}
Note that this same pattern can be applied to more complex state. If you have multiple fields you want to update independently, you can do that by providing some struct of signals:

#[derive(Copy, Clone, Debug)]
struct GlobalState {
count: RwSignal<i32>,
name: RwSignal<String>
}

impl GlobalState {
pub fn new() -> Self {
Self {
count: create_rw_signal(0),
name: create_rw_signal("Bob".to_string())
}
}
}

#[component]
fn App() -> impl IntoView {
provide_context(GlobalState::new());

    // etc.
}
Option #3: Create a Global State Struct and Slices
You may find it cumbersome to wrap each field of a structure in a separate signal like this. In some cases, it can be useful to create a plain struct with non-reactive fields, and then wrap that in a signal.

#[derive(Copy, Clone, Debug, Default)]
struct GlobalState {
count: i32,
name: String
}

#[component]
fn App() -> impl IntoView {
provide_context(create_rw_signal(GlobalState::default()));

    // etc.
}
But there’s a problem: because our whole state is wrapped in one signal, updating the value of one field will cause reactive updates in parts of the UI that only depend on the other.

let state = expect_context::<RwSignal<GlobalState>>();
view! {
<button on:click=move |_| state.update(|state| state.count += 1)>"+1"</button>
<p>{move || state.with(|state| state.name.clone())}</p>
}
In this example, clicking the button will cause the text inside <p> to be updated, cloning state.name again! Because signals are the atomic unit of reactivity, updating any field of the signal triggers updates to everything that depends on the signal.

There’s a better way. You can take fine-grained, reactive slices by using create_memo or create_slice (which uses create_memo but also provides a setter). “Memoizing” a value means creating a new reactive value which will only update when it changes. “Memoizing a slice” means creating a new reactive value which will only update when some field of the state struct updates.

Here, instead of reading from the state signal directly, we create “slices” of that state with fine-grained updates via create_slice. Each slice signal only updates when the particular piece of the larger struct it accesses updates. This means you can create a single root signal, and then take independent, fine-grained slices of it in different components, each of which can update without notifying the others of changes.

/// A component that updates the count in the global state.
#[component]
fn GlobalStateCounter() -> impl IntoView {
let state = expect_context::<RwSignal<GlobalState>>();

    // `create_slice` lets us create a "lens" into the data
    let (count, set_count) = create_slice(

        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.count,
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.count = n,
    );

    view! {
        <div class="consumer blue">
            <button
                on:click=move |_| {
                    set_count(count() + 1);
                }
            >
                "Increment Global Count"
            </button>
            <br/>
            <span>"Count is: " {count}</span>
        </div>
    }
}
Clicking this button only updates state.count, so if we create another slice somewhere else that only takes state.name, clicking the button won’t cause that other slice to update. This allows you to combine the benefits of a top-down data flow and of fine-grained reactive updates.

Note: There are some significant drawbacks to this approach. Both signals and memos need to own their values, so a memo will need to clone the field’s value on every change. The most natural way to manage state in a framework like Leptos is always to provide signals that are as locally-scoped and fine-grained as they can be, not to hoist everything up into global state. But when you do need some kind of global state, create_slice can be a useful tool.

use leptos::*;

// So far, we've only been working with local state in components
// We've only seen how to communicate between parent and child components
// But there are also more general ways to manage global state
//
// The three best approaches to global state are
// 1. Using the router to drive global state via the URL
// 2. Passing signals through context
// 3. Creating a global state struct and creating lenses into it with `create_slice`
//
// Option #1: URL as Global State
// The next few sections of the tutorial will be about the router.
// So for now, we'll just look at options #2 and #3.

// Option #2: Pass Signals through Context
//
// In virtual DOM libraries like React, using the Context API to manage global
// state is a bad idea: because the entire app exists in a tree, changing
// some value provided high up in the tree can cause the whole app to render.
//
// In fine-grained reactive libraries like Leptos, this is simply not the case.
// You can create a signal in the root of your app and pass it down to other
// components using provide_context(). Changing it will only cause rerendering
// in the specific places it is actually used, not the whole app.
#[component]
fn Option2() -> impl IntoView {
// here we create a signal in the root that can be consumed
// anywhere in the app.
let (count, set_count) = create_signal(0);
// we'll pass the setter to specific components,
// but provide the count itself to the whole app via context
provide_context(count);

    view! {
        <h2>"Option 2: Passing Signals"</h2>
        // SetterButton is allowed to modify the count
        <SetterButton set_count/>
        // These consumers can only read from it
        // But we could give them write access by passing `set_count` if we wanted
        <div style="display: flex">
            <FancyMath/>
            <ListItems/>
        </div>
    }
}

/// A button that increments our global counter.
#[component]
fn SetterButton(set_count: WriteSignal<u32>) -> impl IntoView {
view! {
<div class="provider red">
<button on:click=move |_| set_count.update(|count| *count += 1)>
"Increment Global Count"
</button>
</div>
}
}

/// A component that does some "fancy" math with the global count
#[component]
fn FancyMath() -> impl IntoView {
// here we consume the global count signal with `use_context`
let count = use_context::<ReadSignal<u32>>()
// we know we just provided this in the parent component
.expect("there to be a `count` signal provided");
let is_even = move || count() & 1 == 0;

    view! {
        <div class="consumer blue">
            "The number "
            <strong>{count}</strong>
            {move || if is_even() {
                " is"
            } else {
                " is not"
            }}
            " even."
        </div>
    }
}

/// A component that shows a list of items generated from the global count.
#[component]
fn ListItems() -> impl IntoView {
// again, consume the global count signal with `use_context`
let count = use_context::<ReadSignal<u32>>().expect("there to be a `count` signal provided");

    let squares = move || {
        (0..count())
            .map(|n| view! { <li>{n}<sup>"2"</sup> " is " {n * n}</li> })
            .collect::<Vec<_>>()
    };

    view! {
        <div class="consumer green">
            <ul>{squares}</ul>
        </div>
    }
}

// Option #3: Create a Global State Struct
//
// You can use this approach to build a single global data structure
// that holds the state for your whole app, and then access it by
// taking fine-grained slices using `create_slice` or `create_memo`,
// so that changing one part of the state doesn't cause parts of your
// app that depend on other parts of the state to change.

#[derive(Default, Clone, Debug)]
struct GlobalState {
count: u32,
name: String,
}

#[component]
fn Option3() -> impl IntoView {
// we'll provide a single signal that holds the whole state
// each component will be responsible for creating its own "lens" into it
let state = create_rw_signal(GlobalState::default());
provide_context(state);

    view! {
        <h2>"Option 3: Passing Signals"</h2>
        <div class="red consumer" style="width: 100%">
            <h3>"Current Global State"</h3>
            <pre>
                {move || {
                    format!("{:#?}", state.get())
                }}
            </pre>
        </div>
        <div style="display: flex">
            <GlobalStateCounter/>
            <GlobalStateInput/>
        </div>
    }
}

/// A component that updates the count in the global state.
#[component]
fn GlobalStateCounter() -> impl IntoView {
let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");

    // `create_slice` lets us create a "lens" into the data
    let (count, set_count) = create_slice(

        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.count,
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.count = n,
    );

    view! {
        <div class="consumer blue">
            <button
                on:click=move |_| {
                    set_count(count() + 1);
                }
            >
                "Increment Global Count"
            </button>
            <br/>
            <span>"Count is: " {count}</span>
        </div>
    }
}

/// A component that updates the name in the global state.
#[component]
fn GlobalStateInput() -> impl IntoView {
let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");

    // this slice is completely independent of the `count` slice
    // that we created in the other component
    // neither of them will cause the other to rerun
    let (name, set_name) = create_slice(
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.name.clone(),
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.name = n,
    );

    view! {
        <div class="consumer green">
            <input
                type="text"
                prop:value=name
                on:input=move |ev| {
                    set_name(event_target_value(&ev));
                }
            />
            <br/>
            <span>"Name is: " {name}</span>
        </div>
    }
}
// This `main` function is the entry point into the app
// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
leptos::mount_to_body(|| view! { <Option2/><Option3/> })
}


The Basics
Routing drives most websites. A router is the answer to the question, “Given this URL, what should appear on the page?”

A URL consists of many parts. For example, the URL https://my-cool-blog.com/blog/search?q=Search#results consists of

a scheme: https
a domain: my-cool-blog.com
a path: /blog/search
a query (or search): ?q=Search
a hash: #results
The Leptos Router works with the path and query (/blog/search?q=Search). Given this piece of the URL, what should the app render on the page?

The Philosophy
In most cases, the path should drive what is displayed on the page. From the user’s perspective, for most applications, most major changes in the state of the app should be reflected in the URL. If you copy and paste the URL and open it in another tab, you should find yourself more or less in the same place.

In this sense, the router is really at the heart of the global state management for your application. More than anything else, it drives what is displayed on the page.

The router handles most of this work for you by mapping the current location to particular components.


Defining Routes
Getting Started
It’s easy to get started with the router.

First things first, make sure you’ve added the leptos_router package to your dependencies. Like leptos, the router relies on activating a csr, hydrate, or ssr feature. For example, if you’re adding the router to a client-side rendered app, you’ll want to run

cargo add leptos_router --features=csr
It’s important that the router is a separate package from leptos itself. This means that everything in the router can be defined in user-land code. If you want to create your own router, or use no router, you’re completely free to do that!

And import the relevant types from the router, either with something like

use leptos_router::{Route, RouteProps, Router, RouterProps, Routes, RoutesProps};
or simply

use leptos_router::*;
Providing the <Router/>
Routing behavior is provided by the <Router/> component. This should usually be somewhere near the root of your application, the rest of the app.

You shouldn’t try to use multiple <Router/>s in your app. Remember that the router drives global state: if you have multiple routers, which one decides what to do when the URL changes?

Let’s start with a simple <App/> component using the router:

use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
view! {
<Router>
<nav>
/* ... */
</nav>
<main>
/* ... */
</main>
</Router>
}
}
Defining <Routes/>
The <Routes/> component is where you define all the routes to which a user can navigate in your application. Each possible route is defined by a <Route/> component.

You should place the <Routes/> component at the location within your app where you want routes to be rendered. Everything outside <Routes/> will be present on every page, so you can leave things like a navigation bar or menu outside the <Routes/>.

use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
view! {
<Router>
<nav>
/* ... */
</nav>
<main>
// all our routes will appear inside <main>
<Routes>
/* ... */
</Routes>
</main>
</Router>
}
}
Individual routes are defined by providing children to <Routes/> with the <Route/> component. <Route/> takes a path and a view. When the current location matches path, the view will be created and displayed.

The path can include

a static path (/users),
dynamic, named parameters beginning with a colon (/:id),
and/or a wildcard beginning with an asterisk (/user/*any)
The view is a function that returns a view. Any component with no props works here, as does a closure that returns some view.

<Routes>
  <Route path="/" view=Home/>
  <Route path="/users" view=Users/>
  <Route path="/users/:id" view=UserProfile/>
  <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
</Routes>
view takes a Fn() -> impl IntoView. If a component has no props, it can be passed directly into the view. In this case, view=Home is just a shorthand for || view! { <Home/> }.

Now if you navigate to / or to /users you’ll get the home page or the <Users/>. If you go to /users/3 or /blahblah you’ll get a user profile or your 404 page (<NotFound/>). On every navigation, the router determines which <Route/> should be matched, and therefore what content should be displayed where the <Routes/> component is defined.

Note that you can define your routes in any order. The router scores each route to see how good a match it is, rather than simply trying to match them top to bottom.

Simple enough?

Conditional Routes
leptos_router is based on the assumption that you have one and only one <Routes/> component in your app. It uses this to generate routes on the server side, optimize route matching by caching calculated branches, and render your application.

You should not conditionally render <Routes/> using another component like <Show/> or <Suspense/>.

// ❌ don't do this!
view! {
<Show when=|| is_loaded() fallback=|| view! { <p>"Loading"</p> }>
<Routes>
<Route path="/" view=Home/>
</Routes>
</Show>
}
Instead, you can use nested routing to render your <Routes/> once, and conditionally render the router outlet:

// ✅ do this instead!
view! {
<Routes>
// parent route
<Route path="/" view=move || {
view! {
// only show the outlet if data have loaded
<Show when=|| is_loaded() fallback=|| view! { <p>"Loading"</p> }>
<Outlet/>
</Show>
}
}>
// nested child route
<Route path="/" view=Home/>
</Route>
</Routes>
}
If this looks bizarre, don’t worry! The next section of the book is about this kind of nested routing.


Nested Routing
We just defined the following set of routes:

<Routes>
  <Route path="/" view=Home/>
  <Route path="/users" view=Users/>
  <Route path="/users/:id" view=UserProfile/>
  <Route path="/*any" view=NotFound/>
</Routes>
There’s a certain amount of duplication here: /users and /users/:id. This is fine for a small app, but you can probably already tell it won’t scale well. Wouldn’t it be nice if we could nest these routes?

Well... you can!

<Routes>
  <Route path="/" view=Home/>
  <Route path="/users" view=Users>
    <Route path=":id" view=UserProfile/>
  </Route>
  <Route path="/*any" view=NotFound/>
</Routes>
But wait. We’ve just subtly changed what our application does.

The next section is one of the most important in this entire routing section of the guide. Read it carefully, and feel free to ask questions if there’s anything you don’t understand.

Nested Routes as Layout
Nested routes are a form of layout, not a method of route definition.

Let me put that another way: The goal of defining nested routes is not primarily to avoid repeating yourself when typing out the paths in your route definitions. It is actually to tell the router to display multiple <Route/>s on the page at the same time, side by side.

Let’s look back at our practical example.

<Routes>
  <Route path="/users" view=Users/>
  <Route path="/users/:id" view=UserProfile/>
</Routes>
This means:

If I go to /users, I get the <Users/> component.
If I go to /users/3, I get the <UserProfile/> component (with the parameter id set to 3; more on that later)
Let’s say I use nested routes instead:

<Routes>
  <Route path="/users" view=Users>
    <Route path=":id" view=UserProfile/>
  </Route>
</Routes>
This means:

If I go to /users/3, the path matches two <Route/>s: <Users/> and <UserProfile/>.
If I go to /users, the path is not matched.
I actually need to add a fallback route

<Routes>
  <Route path="/users" view=Users>
    <Route path=":id" view=UserProfile/>
    <Route path="" view=NoUser/>
  </Route>
</Routes>
Now:

If I go to /users/3, the path matches <Users/> and <UserProfile/>.
If I go to /users, the path matches <Users/> and <NoUser/>.
When I use nested routes, in other words, each path can match multiple routes: each URL can render the views provided by multiple <Route/> components, at the same time, on the same page.

This may be counter-intuitive, but it’s very powerful, for reasons you’ll hopefully see in a few minutes.

Why Nested Routing?
Why bother with this?

Most web applications contain levels of navigation that correspond to different parts of the layout. For example, in an email app you might have a URL like /contacts/greg, which shows a list of contacts on the left of the screen, and contact details for Greg on the right of the screen. The contact list and the contact details should always appear on the screen at the same time. If there’s no contact selected, maybe you want to show a little instructional text.

You can easily define this with nested routes

<Routes>
  <Route path="/contacts" view=ContactList>
    <Route path=":id" view=ContactInfo/>
    <Route path="" view=|| view! {
      <p>"Select a contact to view more info."</p>
    }/>
  </Route>
</Routes>
You can go even deeper. Say you want to have tabs for each contact’s address, email/phone, and your conversations with them. You can add another set of nested routes inside :id:

<Routes>
  <Route path="/contacts" view=ContactList>
    <Route path=":id" view=ContactInfo>
      <Route path="" view=EmailAndPhone/>
      <Route path="address" view=Address/>
      <Route path="messages" view=Messages/>
    </Route>
    <Route path="" view=|| view! {
      <p>"Select a contact to view more info."</p>
    }/>
  </Route>
</Routes>
The main page of the Remix website, a React framework from the creators of React Router, has a great visual example if you scroll down, with three levels of nested routing: Sales > Invoices > an invoice.

<Outlet/>
Parent routes do not automatically render their nested routes. After all, they are just components; they don’t know exactly where they should render their children, and “just stick it at the end of the parent component” is not a great answer.

Instead, you tell a parent component where to render any nested components with an <Outlet/> component. The <Outlet/> simply renders one of two things:

if there is no nested route that has been matched, it shows nothing
if there is a nested route that has been matched, it shows its view
That’s all! But it’s important to know and to remember, because it’s a common source of “Why isn’t this working?” frustration. If you don’t provide an <Outlet/>, the nested route won’t be displayed.

#[component]
pub fn ContactList() -> impl IntoView {
let contacts = todo!();

view! {
<div style="display: flex">
// the contact list
<For each=contacts
key=|contact| contact.id
children=|contact| todo!()
/>
// the nested child, if any
// don’t forget this!
<Outlet/>
</div>
}
}
Refactoring Route Definitions
You don’t need to define all your routes in one place if you don’t want to. You can refactor any <Route/> and its children out into a separate component.

For example, you can refactor the example above to use two separate components:

#[component]
fn App() -> impl IntoView {
view! {
<Router>
<Routes>
<Route path="/contacts" view=ContactList>
<ContactInfoRoutes/>
<Route path="" view=|| view! {
<p>"Select a contact to view more info."</p>
}/>
</Route>
</Routes>
</Router>
}
}

#[component(transparent)]
fn ContactInfoRoutes() -> impl IntoView {
view! {
<Route path=":id" view=ContactInfo>
<Route path="" view=EmailAndPhone/>
<Route path="address" view=Address/>
<Route path="messages" view=Messages/>
</Route>
}
}
This second component is a #[component(transparent)], meaning it just returns its data, not a view: in this case, it's a RouteDefinition struct, which is what the <Route/> returns. As long as it is marked #[component(transparent)], this sub-route can be defined wherever you want, and inserted as a component into your tree of route definitions.

Nested Routing and Performance
All of this is nice, conceptually, but again—what’s the big deal?

Performance.

In a fine-grained reactive library like Leptos, it’s always important to do the least amount of rendering work you can. Because we’re working with real DOM nodes and not diffing a virtual DOM, we want to “rerender” components as infrequently as possible. Nested routing makes this extremely easy.

Imagine my contact list example. If I navigate from Greg to Alice to Bob and back to Greg, the contact information needs to change on each navigation. But the <ContactList/> should never be rerendered. Not only does this save on rendering performance, it also maintains state in the UI. For example, if I have a search bar at the top of <ContactList/>, navigating from Greg to Alice to Bob won’t clear the search.

In fact, in this case, we don’t even need to rerender the <Contact/> component when moving between contacts. The router will just reactively update the :id parameter as we navigate, allowing us to make fine-grained updates. As we navigate between contacts, we’ll update single text nodes to change the contact’s name, address, and so on, without doing any additional rerendering.

This sandbox includes a couple features (like nested routing) discussed in this section and the previous one, and a couple we’ll cover in the rest of this chapter. The router is such an integrated system that it makes sense to provide a single example, so don’t be surprised if there’s anything you don’t understand.


use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
view! {
<Router>
<h1>"Contact App"</h1>
// this <nav> will show on every routes,
// because it's outside the <Routes/>
// note: we can just use normal <a> tags
// and the router will use client-side navigation
<nav>
<a href="/">"Home"</a>
<a href="/contacts">"Contacts"</a>
</nav>
<main>
<Routes>
// / just has an un-nested "Home"
<Route path="/" view=|| view! {
<h3>"Home"</h3>
}/>
// /contacts has nested routes
<Route
path="/contacts"
view=ContactList
>
// if no id specified, fall back
<Route path=":id" view=ContactInfo>
<Route path="" view=|| view! {
<div class="tab">
"(Contact Info)"
</div>
}/>
<Route path="conversations" view=|| view! {
<div class="tab">
"(Conversations)"
</div>
}/>
</Route>
// if no id specified, fall back
<Route path="" view=|| view! {
<div class="select-user">
"Select a user to view contact info."
</div>
}/>
</Route>
</Routes>
</main>
</Router>
}
}

#[component]
fn ContactList() -> impl IntoView {
view! {
<div class="contact-list">
// here's our contact list component itself
<h3>"Contacts"</h3>
<div class="contact-list-contacts">
<A href="alice">"Alice"</A>
<A href="bob">"Bob"</A>
<A href="steve">"Steve"</A>
</div>

            // <Outlet/> will show the nested child route
            // we can position this outlet wherever we want
            // within the layout
            <Outlet/>
        </div>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
// we can access the :id param reactively with `use_params_map`
let params = use_params_map();
let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // imagine we're loading data from an API here
    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found.",
    };

    view! {
        <h4>{name}</h4>
        <div class="contact-info">
            <div class="tabs">
                <A href="" exact=true>"Contact Info"</A>
                <A href="conversations">"Conversations"</A>
            </div>

            // <Outlet/> here is the tabs that are nested
            // underneath the /contacts/:id route
            <Outlet/>
        </div>
    }
}

fn main() {
leptos::mount_to_body(App)
}


Params and Queries
Static paths are useful for distinguishing between different pages, but almost every application wants to pass data through the URL at some point.

There are two ways you can do this:

named route params like id in /users/:id
named route queries like q in /search?q=Foo
Because of the way URLs are built, you can access the query from any <Route/> view. You can access route params from the <Route/> that defines them or any of its nested children.

Accessing params and queries is pretty simple with a couple of hooks:

use_query or use_query_map
use_params or use_params_map
Each of these comes with a typed option (use_query and use_params) and an untyped option (use_query_map and use_params_map).

The untyped versions hold a simple key-value map. To use the typed versions, derive the Params trait on a struct.

Params is a very lightweight trait to convert a flat key-value map of strings into a struct by applying FromStr to each field. Because of the flat structure of route params and URL queries, it’s significantly less flexible than something like serde; it also adds much less weight to your binary.

use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct ContactParams {
id: usize
}

#[derive(Params, PartialEq)]
struct ContactSearch {
q: String
}
Note: The Params derive macro is located at leptos::Params, and the Params trait is at leptos_router::Params. If you avoid using glob imports like use leptos::*;, make sure you’re importing the right one for the derive macro.

If you are not using the nightly feature, you will get the error

no function or associated item named `into_param` found for struct `std::string::String` in the current scope
At the moment, supporting both T: FromStr and Option<T> for typed params requires a nightly feature. You can fix this by simply changing the struct to use q: Option<String> instead of q: String.

Now we can use them in a component. Imagine a URL that has both params and a query, like /contacts/:id?q=Search.

The typed versions return Memo<Result<T, _>>. It’s a Memo so it reacts to changes in the URL. It’s a Result because the params or query need to be parsed from the URL, and may or may not be valid.

let params = use_params::<ContactParams>();
let query = use_query::<ContactSearch>();

// id: || -> usize
let id = move || {
params.with(|params| {
params.as_ref()
.map(|params| params.id)
.unwrap_or_default()
})
};
The untyped versions return Memo<ParamsMap>. Again, it’s memo to react to changes in the URL. ParamsMap behaves a lot like any other map type, with a .get() method that returns Option<&String>.

let params = use_params_map();
let query = use_query_map();

// id: || -> Option<String>
let id = move || {
params.with(|params| params.get("id").cloned())
};
This can get a little messy: deriving a signal that wraps an Option<_> or Result<_> can involve a couple steps. But it’s worth doing this for two reasons:

It’s correct, i.e., it forces you to consider the cases, “What if the user doesn’t pass a value for this query field? What if they pass an invalid value?”
It’s performant. Specifically, when you navigate between different paths that match the same <Route/> with only params or the query changing, you can get fine-grained updates to different parts of your app without rerendering. For example, navigating between different contacts in our contact-list example does a targeted update to the name field (and eventually contact info) without needing to replace or rerender the wrapping <Contact/>. This is what fine-grained reactivity is for.
This is the same example from the previous section. The router is such an integrated system that it makes sense to provide a single example highlighting multiple features, even if we haven’t explained them all yet.

use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
view! {
<Router>
<h1>"Contact App"</h1>
// this <nav> will show on every routes,
// because it's outside the <Routes/>
// note: we can just use normal <a> tags
// and the router will use client-side navigation
<nav>
<a href="/">"Home"</a>
<a href="/contacts">"Contacts"</a>
</nav>
<main>
<Routes>
// / just has an un-nested "Home"
<Route path="/" view=|| view! {
<h3>"Home"</h3>
}/>
// /contacts has nested routes
<Route
path="/contacts"
view=ContactList
>
// if no id specified, fall back
<Route path=":id" view=ContactInfo>
<Route path="" view=|| view! {
<div class="tab">
"(Contact Info)"
</div>
}/>
<Route path="conversations" view=|| view! {
<div class="tab">
"(Conversations)"
</div>
}/>
</Route>
// if no id specified, fall back
<Route path="" view=|| view! {
<div class="select-user">
"Select a user to view contact info."
</div>
}/>
</Route>
</Routes>
</main>
</Router>
}
}

#[component]
fn ContactList() -> impl IntoView {
view! {
<div class="contact-list">
// here's our contact list component itself
<h3>"Contacts"</h3>
<div class="contact-list-contacts">
<A href="alice">"Alice"</A>
<A href="bob">"Bob"</A>
<A href="steve">"Steve"</A>
</div>

            // <Outlet/> will show the nested child route
            // we can position this outlet wherever we want
            // within the layout
            <Outlet/>
        </div>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
// we can access the :id param reactively with `use_params_map`
let params = use_params_map();
let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // imagine we're loading data from an API here
    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found.",
    };

    view! {
        <h4>{name}</h4>
        <div class="contact-info">
            <div class="tabs">
                <A href="" exact=true>"Contact Info"</A>
                <A href="conversations">"Conversations"</A>
            </div>

            // <Outlet/> here is the tabs that are nested
            // underneath the /contacts/:id route
            <Outlet/>
        </div>
    }
}

fn main() {
leptos::mount_to_body(App)
}


The <A/> Component
Client-side navigation works perfectly fine with ordinary HTML <a> elements. The router adds a listener that handles every click on a <a> element and tries to handle it on the client side, i.e., without doing another round trip to the server to request HTML. This is what enables the snappy “single-page app” navigations you’re probably familiar with from most modern web apps.

The router will bail out of handling an <a> click under a number of situations

the click event has had prevent_default() called on it
the Meta, Alt, Ctrl, or Shift keys were held during click
the <a> has a target or download attribute, or rel="external"
the link has a different origin from the current location
In other words, the router will only try to do a client-side navigation when it’s pretty sure it can handle it, and it will upgrade every <a> element to get this special behavior.

This also means that if you need to opt out of client-side routing, you can do so easily. For example, if you have a link to another page on the same domain, but which isn’t part of your Leptos app, you can just use <a rel="external"> to tell the router it isn’t something it can handle.

The router also provides an <A> component, which does two additional things:

Correctly resolves relative nested routes. Relative routing with ordinary <a> tags can be tricky. For example, if you have a route like /post/:id, <A href="1"> will generate the correct relative route, but <a href="1"> likely will not (depending on where it appears in your view.) <A/> resolves routes relative to the path of the nested route within which it appears.
Sets the aria-current attribute to page if this link is the active link (i.e., it’s a link to the page you’re on). This is helpful for accessibility and for styling. For example, if you want to set the link a different color if it’s a link to the page you’re currently on, you can match this attribute with a CSS selector.
Navigating Programmatically
Your most-used methods of navigating between pages should be with <a> and <form> elements or with the enhanced <A/> and <Form/> components. Using links and forms to navigate is the best solution for accessibility and graceful degradation.

On occasion, though, you’ll want to navigate programmatically, i.e., call a function that can navigate to a new page. In that case, you should use the use_navigate function.

let navigate = leptos_router::use_navigate();
navigate("/somewhere", Default::default());
You should almost never do something like <button on:click=move |_| navigate(/* ... */)>. Any on:click that navigates should be an <a>, for reasons of accessibility.

The second argument here is a set of NavigateOptions, which includes options to resolve the navigation relative to the current route as the <A/> component does, replace it in the navigation stack, include some navigation state, and maintain the current scroll state on navigation.

Once again, this is the same example. Check out the relative <A/> components, and take a look at the CSS in index.html to see the ARIA-based styling.

Struct leptos_router::NavigateOptionsCopy item path
source · [−]
pub struct NavigateOptions {
pub resolve: bool,
pub replace: bool,
pub scroll: bool,
pub state: State,
}
Options that can be used to configure a navigation. Used with use_navigate.

Fields
resolve: bool
Whether the URL being navigated to should be resolved relative to the current route.

replace: bool
If true the new location will replace the current route in the history stack, meaning the “back” button will skip over the current route. (Defaults to false).

scroll: bool
If true, the router will scroll to the top of the window at the end of navigation. Defaults to true.

state: State
State that should be pushed onto the history stack during navigation.

Trait Implementations
source
impl Clone for NavigateOptions
source
fn clone(&self) -> NavigateOptions
Returns a copy of the value. Read more
1.0.0 · source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
source
impl Debug for NavigateOptions
source
fn fmt(&self, f: &mut Formatter<'_>) -> Result
Formats the value using the given formatter. Read more
source
impl Default for NavigateOptions
source
fn default() -> Self
Returns the “default value” for a type. Read more
Auto Trait Implementations
impl Freeze for NavigateOptions
impl RefUnwindSafe for NavigateOptions
impl !Send for NavigateOptions
impl !Sync for NavigateOptions
impl Unpin for NavigateOptions
impl UnwindSafe for NavigateOptions
Blanket Implementations
source§
impl<T> Any for T
where
T: 'static + ?Sized,
source
impl<T> Borrow<T> for T
where
T: ?Sized,
source
impl<T> BorrowMut<T> for T
where
T: ?Sized,
source
impl<T> From<T> for T
source
impl<T> Instrument for T
source
impl<T, U> Into<U> for T
where
U: From<T>,
source
impl<T> ToOwned for T
where
T: Clone,
source
impl<T, U> TryFrom<U> for T
where
U: Into<T>,
source
impl<T, U> TryInto<U> for T
where
U: TryFrom<T>,
source
impl<T> WithSubscriber for T
source
impl<El> ElementDescriptorBounds for El
where
El: Debug,

other code

use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
view! {
<Router>
<h1>"Contact App"</h1>
// this <nav> will show on every routes,
// because it's outside the <Routes/>
// note: we can just use normal <a> tags
// and the router will use client-side navigation
<nav>
<a href="/">"Home"</a>
<a href="/contacts">"Contacts"</a>
</nav>
<main>
<Routes>
// / just has an un-nested "Home"
<Route path="/" view=|| view! {
<h3>"Home"</h3>
}/>
// /contacts has nested routes
<Route
path="/contacts"
view=ContactList
>
// if no id specified, fall back
<Route path=":id" view=ContactInfo>
<Route path="" view=|| view! {
<div class="tab">
"(Contact Info)"
</div>
}/>
<Route path="conversations" view=|| view! {
<div class="tab">
"(Conversations)"
</div>
}/>
</Route>
// if no id specified, fall back
<Route path="" view=|| view! {
<div class="select-user">
"Select a user to view contact info."
</div>
}/>
</Route>
</Routes>
</main>
</Router>
}
}

#[component]
fn ContactList() -> impl IntoView {
view! {
<div class="contact-list">
// here's our contact list component itself
<h3>"Contacts"</h3>
<div class="contact-list-contacts">
<A href="alice">"Alice"</A>
<A href="bob">"Bob"</A>
<A href="steve">"Steve"</A>
</div>

            // <Outlet/> will show the nested child route
            // we can position this outlet wherever we want
            // within the layout
            <Outlet/>
        </div>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
// we can access the :id param reactively with `use_params_map`
let params = use_params_map();
let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    // imagine we're loading data from an API here
    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found.",
    };

    view! {
        <h4>{name}</h4>
        <div class="contact-info">
            <div class="tabs">
                <A href="" exact=true>"Contact Info"</A>
                <A href="conversations">"Conversations"</A>
            </div>

            // <Outlet/> here is the tabs that are nested
            // underneath the /contacts/:id route
            <Outlet/>
        </div>
    }
}

fn main() {
leptos::mount_to_body(App)
}


The <Form/> Component
Links and forms sometimes seem completely unrelated. But, in fact, they work in very similar ways.

In plain HTML, there are three ways to navigate to another page:

An <a> element that links to another page: Navigates to the URL in its href attribute with the GET HTTP method.
A <form method="GET">: Navigates to the URL in its action attribute with the GET HTTP method and the form data from its inputs encoded in the URL query string.
A <form method="POST">: Navigates to the URL in its action attribute with the POST HTTP method and the form data from its inputs encoded in the body of the request.
Since we have a client-side router, we can do client-side link navigations without reloading the page, i.e., without a full round-trip to the server and back. It makes sense that we can do client-side form navigations in the same way.

The router provides a <Form> component, which works like the HTML <form> element, but uses client-side navigations instead of full page reloads. <Form/> works with both GET and POST requests. With method="GET", it will navigate to the URL encoded in the form data. With method="POST" it will make a POST request and handle the server’s response.

<Form/> provides the basis for some components like <ActionForm/> and <MultiActionForm/> that we’ll see in later chapters. But it also enables some powerful patterns of its own.

For example, imagine that you want to create a search field that updates search results in real time as the user searches, without a page reload, but that also stores the search in the URL so a user can copy and paste it to share results with someone else.

It turns out that the patterns we’ve learned so far make this easy to implement.

async fn fetch_results() {
// some async function to fetch our search results
}

#[component]
pub fn FormExample() -> impl IntoView {
// reactive access to URL query strings
let query = use_query_map();
// search stored as ?q=
let search = move || query().get("q").cloned().unwrap_or_default();
// a resource driven by the search string
let search_results = create_resource(search, fetch_results);

    view! {
        <Form method="GET" action="">
            <input type="search" name="q" value=search/>
            <input type="submit"/>
        </Form>
        <Transition fallback=move || ()>
            /* render search results */
        </Transition>
    }
}
Whenever you click Submit, the <Form/> will “navigate” to ?q={search}. But because this navigation is done on the client side, there’s no page flicker or reload. The URL query string changes, which triggers search to update. Because search is the source signal for the search_results resource, this triggers search_results to reload its resource. The <Transition/> continues displaying the current search results until the new ones have loaded. When they are complete, it switches to displaying the new result.

This is a great pattern. The data flow is extremely clear: all data flows from the URL to the resource into the UI. The current state of the application is stored in the URL, which means you can refresh the page or text the link to a friend and it will show exactly what you’re expecting. And once we introduce server rendering, this pattern will prove to be really fault-tolerant, too: because it uses a <form> element and URLs under the hood, it actually works really well without even loading your WASM on the client.

We can actually take it a step further and do something kind of clever:

view! {
<Form method="GET" action="">
<input type="search" name="q" value=search
oninput="this.form.requestSubmit()"
/>
</Form>
}
You’ll notice that this version drops the Submit button. Instead, we add an oninput attribute to the input. Note that this is not on:input, which would listen for the input event and run some Rust code. Without the colon, oninput is the plain HTML attribute. So the string is actually a JavaScript string. this.form gives us the form the input is attached to. requestSubmit() fires the submit event on the <form>, which is caught by <Form/> just as if we had clicked a Submit button. Now the form will “navigate” on every keystroke or input to keep the URL (and therefore the search) perfectly in sync with the user’s input as they type.

use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
view! {
<Router>
<h1><code>"<Form/>"</code></h1>
<main>
<Routes>
<Route path="" view=FormExample/>
</Routes>
</main>
</Router>
}
}

#[component]
pub fn FormExample() -> impl IntoView {
// reactive access to URL query
let query = use_query_map();
let name = move || query().get("name").cloned().unwrap_or_default();
let number = move || query().get("number").cloned().unwrap_or_default();
let select = move || query().get("select").cloned().unwrap_or_default();

    view! {
        // read out the URL query strings
        <table>
            <tr>
                <td><code>"name"</code></td>
                <td>{name}</td>
            </tr>
            <tr>
                <td><code>"number"</code></td>
                <td>{number}</td>
            </tr>
            <tr>
                <td><code>"select"</code></td>
                <td>{select}</td>
            </tr>
        </table>
        // <Form/> will navigate whenever submitted
        <h2>"Manual Submission"</h2>
        <Form method="GET" action="">
            // input names determine query string key
            <input type="text" name="name" value=name/>
            <input type="number" name="number" value=number/>
            <select name="select">
                // `selected` will set which starts as selected
                <option selected=move || select() == "A">
                    "A"
                </option>
                <option selected=move || select() == "B">
                    "B"
                </option>
                <option selected=move || select() == "C">
                    "C"
                </option>
            </select>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
        // This <Form/> uses some JavaScript to submit
        // on every input
        <h2>"Automatic Submission"</h2>
        <Form method="GET" action="">
            <input
                type="text"
                name="name"
                value=name
                // this oninput attribute will cause the
                // form to submit on every input to the field
                oninput="this.form.requestSubmit()"
            />
            <input
                type="number"
                name="number"
                value=number
                oninput="this.form.requestSubmit()"
            />
            <select name="select"
                onchange="this.form.requestSubmit()"
            >
                <option selected=move || select() == "A">
                    "A"
                </option>
                <option selected=move || select() == "B">
                    "B"
                </option>
                <option selected=move || select() == "C">
                    "C"
                </option>
            </select>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
    }
}

fn main() {
leptos::mount_to_body(App)
}


Interlude: Styling
Anyone creating a website or application soon runs into the question of styling. For a small app, a single CSS file is probably plenty to style your user interface. But as an application grows, many developers find that plain CSS becomes increasingly hard to manage.

Some frontend frameworks (like Angular, Vue, and Svelte) provide built-in ways to scope your CSS to particular components, making it easier to manage styles across a whole application without styles meant to modify one small component having a global effect. Other frameworks (like React or Solid) don’t provide built-in CSS scoping, but rely on libraries in the ecosystem to do it for them. Leptos is in this latter camp: the framework itself has no opinions about CSS at all, but provides a few tools and primitives that allow others to build styling libraries.

Here are a few different approaches to styling your Leptos app, other than plain CSS.

TailwindCSS: Utility-first CSS
TailwindCSS is a popular utility-first CSS library. It allows you to style your application by using inline utility classes, with a custom CLI tool that scans your files for Tailwind class names and bundles the necessary CSS.

This allows you to write components like this:

#[component]
fn Home() -> impl IntoView {
let (count, set_count) = create_signal(0);

    view! {
        <main class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                {move || if count() == 0 {
                    "Click me!".to_string()
                } else {
                    count().to_string()
                }}
            </button>
        </main>
    }
}
It can be a little complicated to set up the Tailwind integration at first, but you can check out our two examples of how to use Tailwind with a client-side-rendered trunk application or with a server-rendered cargo-leptos application. cargo-leptos also has some built-in Tailwind support that you can use as an alternative to Tailwind’s CLI.

Stylers: Compile-time CSS Extraction
Stylers is a compile-time scoped CSS library that lets you declare scoped CSS in the body of your component. Stylers will extract this CSS at compile time into CSS files that you can then import into your app, which means that it doesn’t add anything to the WASM binary size of your application.

This allows you to write components like this:

use stylers::style;

#[component]
pub fn App() -> impl IntoView {
let styler_class = style! { "App",
#two{
color: blue;
}
div.one{
color: red;
content: raw_str(r#"\hello"#);
font: "1.3em/1.2" Arial, Helvetica, sans-serif;
}
div {
border: 1px solid black;
margin: 25px 50px 75px 100px;
background-color: lightblue;
}
h2 {
color: purple;
}
@media only screen and (max-width: 1000px) {
h3 {
background-color: lightblue;
color: blue
}
}
};

    view! { class = styler_class,
        <div class="one">
            <h1 id="two">"Hello"</h1>
            <h2>"World"</h2>
            <h2>"and"</h2>
            <h3>"friends!"</h3>
        </div>
    }
}
Stylance: Scoped CSS Written in CSS Files
Stylers lets you write CSS inline in your Rust code, extracts it at compile time, and scopes it. Stylance allows you to write your CSS in CSS files alongside your components, import those files into your components, and scope the CSS classes to your components.

This works well with the live-reloading features of trunk and cargo-leptos because edited CSS files can be updated immediately in the browser.

import_style!(style, "app.module.scss");

#[component]
fn HomePage() -> impl IntoView {
view! {
<div class=style::jumbotron/>
}
}
You can edit the CSS directly without causing a Rust recompile.

.jumbotron {
background: blue;
}
Styled: Runtime CSS Scoping
Styled is a runtime scoped CSS library that integrates well with Leptos. It lets you declare scoped CSS in the body of your component function, and then applies those styles at runtime.

use styled::style;

#[component]
pub fn MyComponent() -> impl IntoView {
let styles = style!(
div {
background-color: red;
color: white;
}
);

    styled::view! { styles,
        <div>"This text should be red with white text."</div>
    }
}
Contributions Welcome
Leptos has no opinions on how you style your website or app, but we’re very happy to provide support to any tools you’re trying to create to make it easier. If you’re working on a CSS or styling approach that you’d like to add to this list, please let us know!

Styled: Easy Styling for Leptos Components
If you're looking for an easy way to apply scoped styles to your Leptos components, Styled is the Leptos macro you need. With Styled, you can apply high-level selectors like button or div to specific components, keeping your markup clean and organized.

Installation
Use cargo add in your project root

cargo add styled stylist
Usage
First create a basic Leptos component. This will serve as the foundation for this little guide.

#[component]
pub fn MyComponent(cx: Scope) -> impl IntoView{
view! {
cx,
<div>"hello"</div>
}
}
Next, import the style macro, powered by an awesome crate called Stylist, to create your styles. Just add this to the top of your file.

use styled::style;
You can then use the style macro to create a Result containing your styles. Let's modify our component:

#[component]
pub fn MyComponent(cx: Scope) -> impl IntoView{

let styles = style!(
div {
background-color: red;
color: white;
}
);

view! {
cx,
<div>"hello"</div>
}
}
Now, let's apply those styles with our styled::view! macro!

#[component]
pub fn MyComponent(cx: Scope) -> impl IntoView {

    let styles = style!(
      div {
        background-color: red;
        color: white;
      }
    );

    styled::view! {
        cx,
        styles,
        <div>"This text should be red with white text."</div>
    }
}
Now we can define another component that also uses the div CSS selector but it's styles will only apply to the elements inside of it's enclosing styled::view! macro.

#[component]
pub fn AnotherComponent(cx: Scope) -> impl IntoView {

    // note were using a plain div selector and it wont clash with MyComponent's div style!
    let styles = style!(
      div {
        background-color: blue;
        color: gray;
      }
    );

    styled::view! {
        cx,
        styles,
        <div>"This text should be blue with gray text."</div>
    }
}
Longer Example
// /src/components/button.rs

use crate::theme::get_theme;
use leptos::*;
use styled::style;

#[derive(PartialEq)]
pub enum Variant {
PRIMARY,
SECONDARY,
ALERT,
DISABLED,
}

impl Variant {
pub fn is(&self, variant: &Variant) -> bool {
self == variant
}
}

struct ButtonColors {
text: String,
background: String,
border: String,
}

fn get_colors(variant: &Variant) -> ButtonColors {
let theme = get_theme().unwrap();
match variant {
Variant::PRIMARY => ButtonColors {
text: theme.white(),
background: theme.black(),
border: theme.transparent(),
},
Variant::SECONDARY => ButtonColors {
text: theme.black(),
background: theme.white(),
border: theme.gray.lightest(),
},
Variant::ALERT => ButtonColors {
text: theme.white(),
background: theme.red(),
border: theme.transparent(),
},
Variant::DISABLED => ButtonColors {
text: theme.white(),
background: theme.red(),
border: theme.transparent(),
},
}
}

#[component]
pub fn Button(cx: Scope, variant: Variant) -> impl IntoView {
let disabled = variant.is(&Variant::DISABLED);

    let styles = styles(&variant);

    styled::view! {
        cx,
        styles,
        <button disabled=disabled>"Button"</button>
    }
}

fn styles<'a>(variant: &Variant) -> styled::Result<styled::Style> {
let colors = get_colors(variant);

    style!(
            button {
                color: ${colors.text};
                background-color: ${colors.background};
                border: 1px solid ${colors.border};
                outline: none;
                height: 48px;
                min-width: 154px;
                font-size: 14px;
                font-weight: 700;
                text-align: center;
                box-shadow: rgba(0, 0, 0, 0.05) 0px 1px 2px 0px;
                position: relative;
                box-sizing: border-box;
                vertical-align: middle;
                text-align: center;
                text-overflow: ellipsis;
                text-transform: uppercase;
                overflow: hidden;
                cursor: pointer;
                transition: box-shadow 0.2s;
                margin: 10px;
            }

            & button:active {
                transform: scale(0.99);
            }


            & button::-moz-focus-inner {
                border: none;
            }

            & button::before {
                content: "";
                position: absolute;
                top: 0;
                bottom: 0;
                left: 0;
                right: 0;
                background-color: rgb(255, 255, 255);
                opacity: 0;
                transition: opacity 0.2s;
            }

            & button::after {
                content: "";
                position: absolute;
                left: 50%;
                top: 50%;
                border-radius: 50%;
                padding: 50%;
                background-color: ${colors.text};
                opacity: 0;
                transform: translate(-50%, -50%) scale(1);
                transition: opacity 1s, transform 0.5s;
            }

            & button:hover,
            & button:focus {
                box-shadow: 0 2px 4px -1px rgba(0, 0, 0, 0.2), 0 4px 5px 0 rgba(0, 0, 0, 0.14), 0 1px 10px 0 rgba(0, 0, 0, 0.12);
            }

            & button:hover::before {
                opacity: 0.08;
            }

            & button:hover:focus::before {
                opacity: 0.3;
            }

            & button:active {
                box-shadow: 0 5px 5px -3px rgba(0, 0, 0, 0.2), 0 8px 10px 1px rgba(0, 0, 0, 0.14), 0 3px 14px 2px rgba(0, 0, 0, 0.12);
            }

            & button:active::after {
                opacity: 0.32;
                transform: translate(-50%, -50%) scale(0);
                transition: transform 0s;
            }

            & button:disabled {
                color: rgba(0, 0, 0, 0.28);
                background-color: rgba(0, 0, 0, 0.12);
                box-shadow: none;
                cursor: initial;
            }

            & button:disabled::before {
                opacity: 0;
            }

            & button:disabled::after {
                opacity: 0;
            }

    )
}
// /src/theme/mod.rs
use csscolorparser::Color;

pub fn get_theme() -> Result<Theme, csscolorparser::ParseColorError> {
let theme = Theme {
teal: Colors {
main: Color::from_html("#6FDDDB")?,
darker: Color::from_html("#2BB4B2")?,
lighter: Color::from_html("#7EE1DF")?,
lightest: Color::from_html("#B2EDEC")?,
},
pink: Colors {
main: Color::from_html("#E93EF5")?,
darker: Color::from_html("#C70BD4")?,
lighter: Color::from_html("#F5A4FA")?,
lightest: Color::from_html("#FCE1FD")?,
},
green: Colors {
main: Color::from_html("#54D072")?,
darker: Color::from_html("#30AF4F")?,
lighter: Color::from_html("#82DD98")?,
lightest: Color::from_html("#B4EAC1")?,
},
purple: Colors {
main: Color::from_html("#8C18FB")?,
darker: Color::from_html("#7204DB")?,
lighter: Color::from_html("#B162FC")?,
lightest: Color::from_html("#D0A1FD")?,
},
yellow: Colors {
main: Color::from_html("#E1E862")?,
darker: Color::from_html("#BAC31D")?,
lighter: Color::from_html("#EFF3AC")?,
lightest: Color::from_html("#FAFBE3")?,
},
gray: Colors {
main: Color::from_html("#4a4a4a")?,
darker: Color::from_html("#3d3d3d")?,
lighter: Color::from_html("#939393")?,
lightest: Color::from_html("#c4c4c4")?,
},
red: Color::from_html("#FF5854")?,
black: Color::from_html("#000000")?,
white: Color::from_html("#FFFFFF")?,
transparent: Color::from_html("transparent")?,
};

    Ok(theme)
}

pub struct Theme {
pub teal: Colors,
pub pink: Colors,
pub green: Colors,
pub purple: Colors,
pub yellow: Colors,
pub gray: Colors,
pub red: Color,
pub black: Color,
pub white: Color,
pub transparent: Color,
}

pub struct Colors {
pub main: Color,
pub darker: Color,
pub lighter: Color,
pub lightest: Color,
}

impl Colors {
pub fn main(&self) -> String {
self.main.to_hex_string()
}
pub fn darker(&self) -> String {
self.darker.to_hex_string()
}
pub fn lighter(&self) -> String {
self.lighter.to_hex_string()
}
pub fn lightest(&self) -> String {
self.lightest.to_hex_string()
}
}

impl Theme {
pub fn red(&self) -> String {
self.red.to_hex_string()
}
pub fn black(&self) -> String {
self.black.to_hex_string()
}
pub fn white(&self) -> String {
self.white.to_hex_string()
}
pub fn transparent(&self) -> String {
self.transparent.to_hex_string()
}
}
// /src/app.rs

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
view! { cx,
<Button variant={button::Variant::PRIMARY}/>
<Button variant={button::Variant::SECONDARY}/>
<Button variant={button::Variant::ALERT}/>
}
}

Metadata
So far, everything we’ve rendered has been inside the <body> of the HTML document. And this makes sense. After all, everything you can see on a web page lives inside the <body>.

However, there are plenty of occasions where you might want to update something inside the <head> of the document using the same reactive primitives and component patterns you use for your UI.

That’s where the leptos_meta package comes in.

Metadata Components
leptos_meta provides special components that let you inject data from inside components anywhere in your application into the <head>:

<Title/> allows you to set the document’s title from any component. It also takes a formatter function that can be used to apply the same format to the title set by other pages. So, for example, if you put <Title formatter=|text| format!("{text} — My Awesome Site")/> in your <App/> component, and then <Title text="Page 1"/> and <Title text="Page 2"/> on your routes, you’ll get Page 1 — My Awesome Site and Page 2 — My Awesome Site.

<Link/> takes the standard attributes of the <link> element.

<Stylesheet/> creates a <link rel="stylesheet"> with the href you give.

<Style/> creates a <style> with the children you pass in (usually a string). You can use this to import some custom CSS from another file at compile time <Style>{include_str!("my_route.css")}</Style>.

<Meta/> lets you set <meta> tags with descriptions and other metadata.

<Script/> and <script>
leptos_meta also provides a <Script/> component, and it’s worth pausing here for a second. All of the other components we’ve considered inject <head>-only elements in the <head>. But a <script> can also be included in the body.

There’s a very simple way to determine whether you should use a capital-S <Script/> component or a lowercase-s <script> element: the <Script/> component will be rendered in the <head>, and the <script> element will be rendered wherever in the <body> of your user interface you put it in, alongside other normal HTML elements. These cause JavaScript to load and run at different times, so use whichever is appropriate to your needs.

<Body/> and <Html/>
There are even a couple elements designed to make semantic HTML and styling easier. <Html/> lets you set the lang and dir on your <html> tag from your application code. <Html/> and <Body/> both have class props that let you set their respective class attributes, which is sometimes needed by CSS frameworks for styling.

<Body/> and <Html/> both also have attributes props which can be used to set any number of additional attributes on them via the attr: syntax:

<Html
lang="he"
dir="rtl"
attr:data-theme="dark"
/>
Metadata and Server Rendering
Now, some of this is useful in any scenario, but some of it is especially important for search-engine optimization (SEO). Making sure you have things like appropriate <title> and <meta> tags is crucial. Modern search engine crawlers do handle client-side rendering, i.e., apps that are shipped as an empty index.html and rendered entirely in JS/WASM. But they prefer to receive pages in which your app has been rendered to actual HTML, with metadata in the <head>.

This is exactly what leptos_meta is for. And in fact, during server rendering, this is exactly what it does: collect all the <head> content you’ve declared by using its components throughout your application, and then inject it into the actual <head>.

But I’m getting ahead of myself. We haven’t actually talked about server-side rendering yet. The next chapter will talk about integrating with JavaScript libraries. Then we’ll wrap up the discussion of the client side, and move onto server side rendering.


Integrating with JavaScript: wasm-bindgen, web_sys and HtmlElement
Leptos provides a variety of tools to allow you to build declarative web applications without leaving the world of the framework. Things like the reactive system, component and view macros, and router allow you to build user interfaces without directly interacting with the Web APIs provided by the browser. And they let you do it all directly in Rust, which is great—assuming you like Rust. (And if you’ve gotten this far in the book, we assume you like Rust.)

Ecosystem crates like the fantastic set of utilities provided by leptos-use can take you even further, by providing Leptos-specific reactive wrappers around many Web APIs.

Nevertheless, in many cases you will need to access JavaScript libraries or Web APIs directly. This chapter can help.

Using JS Libraries with wasm-bindgen
Your Rust code can be compiled to a WebAssembly (WASM) module and loaded to run in the browser. However, WASM does not have direct access to browser APIs. Instead, the Rust/WASM ecosystem depends on generating bindings from your Rust code to the JavaScript browser environment that hosts it.

The wasm-bindgen crate is at the center of that ecosystem. It provides both an interface for marking parts of Rust code with annotations telling it how to call JS, and a CLI tool for generating the necessary JS glue code. You’ve been using this without knowing it all along: both trunk and cargo-leptos rely on wasm-bindgen under the hood.

If there is a JavaScript library that you want to call from Rust, you should refer to the wasm-bindgen docs on importing functions from JS. It is relatively easy to import individual functions, classes, or values from JavaScript to use in your Rust app.

It is not always easy to integrate JS libraries into your app directly. In particular, any library that depends on a particular JS framework like React may be hard to integrated. Libraries that manipulate DOM state in some way (for example, rich text editors) should also be used with care: both Leptos and the JS library will probably assume that they are the ultimate source of truth for the app’s state, so you should be careful to separate their responsibilities.

Accessing Web APIs with web-sys
If you just need to access some browser APIs without pulling in a separate JS library, you can do so using the web_sys crate. This provides bindings for all of the Web APIs provided by the browser, with 1:1 mappings from browser types and functions to Rust structs and methods.

In general, if you’re asking “how do I do X with Leptos?” where do X is accessing some Web API, looking up a vanilla JavaScript solution and translating it to Rust using the web-sys docs is a good approach.

After this section, you might find the wasm-bindgen guide chapter on web-sys useful for additional reading.

Enabling features
web_sys is heavily feature-gated to keep compile times low. If you would like to use one of its many APIs, you may need to enable a feature to use it.

The features required to use an item are always listed in its documentation. For example, to use Element::get_bounding_rect_client, you need to enable the DomRect and Element features.

Leptos already enables a whole bunch of features - if the required feature is already enabled here, you won't have to enable it in your own app. Otherwise, add it to your Cargo.toml and you’re good to go!

[dependencies.web-sys]
version = "0.3"
features = ["DomRect"]
However, as the JavaScript standard evolves and APIs are being written, you may want to use browser features that are technically not fully stable yet, such as WebGPU. web_sys will follow the (potentially frequently changing) standard, which means that no stability guarantees are made.

In order to use this, you need to add RUSTFLAGS=--cfg=web_sys_unstable_apis as an environment variable. This can either be done by adding it to every command, or add it to .cargo/config.toml in your repository.

As part of a command:

RUSTFLAGS=--cfg=web_sys_unstable_apis cargo # ...
In .cargo/config.toml:

[env]
RUSTFLAGS = "--cfg=web_sys_unstable_apis"
Accessing raw HtmlElements from your view
The declarative style of the framework means that you don’t need to directly manipulate DOM nodes to build up your user interface. However, in some cases you want direct access to the underlying DOM element that represents part of your view. The section of the book on “uncontrolled inputs” showed how to do this using the NodeRef type.

You may notice that NodeRef::get returns an Option<leptos::HtmlElement<T>>. This is not the same type as a web_sys::HtmlElement, although they are related. So what is this HtmlElement<T> type, and how do you use it?

Overview
web_sys::HtmlElement is the Rust equivalent of the browser’s HTMLElement interface, which is implemented for all HTML elements. It provides access to a minimal set of functions and APIs that are guaranteed to be available for any HTML element. Each particular HTML element then has its own element class, which implements additional functionality. The goal of leptos::HtmlElement<T> is to bridge the gap between elements in your view and these more specific JavaScript types, so that you can access the particular functionality of those elements.

This is implement by using the Rust Deref trait to allow you to dereference a leptos::HtmlElement<T> to the appropriately-typed JS object for that particular element type T.

Definition
Understanding this relationship involves understanding some related traits.

The following simply defines what types are allowed inside the T of leptos::HtmlElement<T> and how it links to web_sys.

pub struct HtmlElement<El> where El: ElementDescriptor { /* ... */ }

pub trait ElementDescriptor: ElementDescriptorBounds { /* ... */ }

pub trait ElementDescriptorBounds: Debug {}
impl<El> ElementDescriptorBounds for El where El: Debug {}

// this is implemented for every single element in `leptos::{html, svg, math}::*`
impl ElementDescriptor for leptos::html::Div { /* ... */ }

// same with this, derefs to the corresponding `web_sys::Html*Element`
impl Deref for leptos::html::Div {
type Target = web_sys::HtmlDivElement;
// ...
}
The following is from web_sys:

impl Deref for web_sys::HtmlDivElement {
type Target = web_sys::HtmlElement;
// ...
}

impl Deref for web_sys::HtmlElement {
type Target = web_sys::Element;
// ...
}

impl Deref for web_sys::Element {
type Target = web_sys::Node;
// ...
}

impl Deref for web_sys::Node {
type Target = web_sys::EventTarget;
// ...
}
web_sys uses long deref chains to emulate the inheritance used in JavaScript. If you can't find the method you're looking for on one type, take a look further down the deref chain. The leptos::html::* types all deref into web_sys::Html*Element or web_sys::HtmlElement. By calling element.method(), Rust will automatically add more derefs as needed to call the correct method!

However, some methods have the same name, such as leptos::HtmlElement::style and web_sys::HtmlElement::style. In these cases, Rust will pick the one that requires the least amount of derefs, which is leptos::HtmlElement::style if you're getting an element straight from a NodeRef. If you wish to use the web_sys method instead, you can manually deref with (*element).style().

If you want to have even more control over which type you are calling a method from, AsRef<T> is implemented for all types that are part of the deref chain, so you can explicitly state which type you want.

See also: The wasm-bindgen Guide: Inheritance in web-sys.

Clones
The web_sys::HtmlElement (and by extension the leptos::HtmlElement too) actually only store references to the HTML element it affects. Therefore, calling .clone() doesn't actually make a new HTML element, it simply gets another reference to the same one. Calling methods that change the element from any of its clones will affect the original element.

Unfortunately, web_sys::HtmlElement does not implement Copy, so you may need to add a bunch of clones especially when using it in closures. Don't worry though, these clones are cheap!

Casting
You can get less specific types through Deref or AsRef, so use those when possible. However, if you need to cast to a more specific type (e.g. from an EventTarget to a HtmlInputElement), you will need to use the methods provided by wasm_bindgen::JsCast (re-exported through web_sys::wasm_bindgen::JsCast). You'll probably only need the dyn_ref method.

use web_sys::wasm_bindgen::JsCast;

let on_click = |ev: MouseEvent| {
let target: HtmlInputElement = ev.current_target().unwrap().dyn_ref().unwrap();
// or, just use the existing `leptos::event_target_*` functions
}
See the event_target_* functions here, if you're curious.

leptos::HtmlElement
The leptos::HtmlElement adds some extra convenience methods to make it easier to manipulate common attributes. These methods were built for the builder syntax, so it takes and returns self. You can just do _ = element.clone().<method>() to ignore the element it returns - it'll still affect the original element, even though it doesn't look like it (see previous section on Clones)!

Here are some of the common methods you may want to use, for example in event listeners or use: directives.

id: overwrites the id on the element.
classes: adds the classes to the element. You can specify multiple classes with a space-separated string. You can also use class to conditionally add a single class: do not add multiple with this method.
attr: sets a key=value attribute to the element.
prop: sets a property on the element: see the distinction between properties and attributes here.
on: adds an event listener to the element. Specify the event type through one of leptos::ev::* (it's the ones in all lowercase).
child: adds an element as the last child of the element.
Take a look at the rest of the leptos::HtmlElement methods too. If none of them fit your requirements, also take a look at leptos-use. Otherwise, you’ll have to use the web_sys APIs.


Wrapping Up Part 1: Client-Side Rendering
So far, everything we’ve written has been rendered almost entirely in the browser. When we create an app using Trunk, it’s served using a local development server. If you build it for production and deploy it, it’s served by whatever server or CDN you’re using. In either case, what’s served is an HTML page with

the URL of your Leptos app, which has been compiled to WebAssembly (WASM)
the URL of the JavaScript used to initialize this WASM blob
an empty <body> element
When the JS and WASM have loaded, Leptos will render your app into the <body>. This means that nothing appears on the screen until JS/WASM have loaded and run. This has some drawbacks:

It increases load time, as your user’s screen is blank until additional resources have been downloaded.
It’s bad for SEO, as load times are longer and the HTML you serve has no meaningful content.
It’s broken for users for whom JS/WASM don’t load for some reason (e.g., they’re on a train and just went into a tunnel before WASM finished loading; they’re using an older device that doesn’t support WASM; they have JavaScript or WASM turned off for some reason; etc.)
These downsides apply across the web ecosystem, but especially to WASM apps.

However, depending on the requirements of your project, you may be fine with these limitations.

If you just want to deploy your Client-Side Rendered website, skip ahead to the chapter on "Deployment" - there, you'll find directions on how best to deploy your Leptos CSR site.

But what do you do if you want to return more than just an empty <body> tag in your index.html page? Use “Server-Side Rendering”!

Whole books could be (and probably have been) written about this topic, but at its core, it’s really simple: rather than returning an empty <body> tag, with SSR, you'll return an initial HTML page that reflects the actual starting state of your app or site, so that while JS/WASM are loading, and until they load, the user can access the plain HTML version.

Part 2 of this book, on Leptos SSR, will cover this topic in some detail!


Part 2: Server Side Rendering
The second part of the book is all about how to turn your beautiful UIs into full-stack Rust + Leptos powered websites and applications.

As you read in the last chapter, there are some limitations to using client-side rendered Leptos apps - over the next few chapters, you'll see how we can overcome those limitations and get the best performance and SEO out of your Leptos apps.

Info

When working with Leptos on the server side, you're free to choose either the Actix-web or the Axum integrations - the full feature set of Leptos is available with either option.

If, however, you need deploy to a WinterCG-compatible runtime like Deno, Cloudflare, etc., then choose the Axum integration as this deployment option is only available with Axum on the server. Lastly, if you'd like to go full-stack WASM/WASI and deploy to WASM-based serverless runtimes, then Axum is your go-to choice here too.

NB: this is a limitation of the web frameworks themselves, not Leptos.


Introducing cargo-leptos
So far, we’ve just been running code in the browser and using Trunk to coordinate the build process and run a local development process. If we’re going to add server-side rendering, we’ll need to run our application code on the server as well. This means we’ll need to build two separate binaries, one compiled to native code and running the server, the other compiled to WebAssembly (WASM) and running in the user’s browser. Additionally, the server needs to know how to serve this WASM version (and the JavaScript required to initialize it) to the browser.

This is not an insurmountable task but it adds some complication. For convenience and an easier developer experience, we built the cargo-leptos build tool. cargo-leptos basically exists to coordinate the build process for your app, handling recompiling the server and client halves when you make changes, and adding some built-in support for things like Tailwind, SASS, and testing.

Getting started is pretty easy. Just run

cargo install cargo-leptos
And then to create a new project, you can run either

# for an Actix template
cargo leptos new --git leptos-rs/start
or

# for an Axum template
cargo leptos new --git leptos-rs/start-axum
Now cd into the directory you’ve created and run

cargo leptos watch
Note: Remember that Leptos has a nightly feature, which each of these starters use. If you're using the stable Rust compiler, that’s fine; just remove the nightly feature from each of the Leptos dependencies in your new Cargo.toml and you should be all set.

Once your app has compiled you can open up your browser to http://localhost:3000 to see it.

cargo-leptos has lots of additional features and built in tools. You can learn more in its README.

But what exactly is happening when you open our browser to localhost:3000? Well, read on to find out.

deep

Features
Parallel build of server and client in watch mode for fast developer feedback.
CSS hot-reload (no page-reload, only CSS updated).
Build server and client for hydration (client-side rendering mode not supported).
Support for both workspace and single-package setup.
SCSS compilation using dart-sass.
CSS transformation and minification using Lightning CSS.
Builds server and client (wasm) binaries using Cargo.
Generates JS - Wasm bindings with wasm-bindgen
Includes support for JS Snippets for when you want to call some JS code from your WASM.
Optimises the wasm with wasm-opt from Binaryen
watch command for automatic rebuilds with browser live-reload.
test command for running tests of the lib and bin packages that makes up the Leptos project.
build build the server and client.
end-to-end command for building, running the server and calling a bash shell hook. The hook would typically launch Playwright or similar.
new command for creating a new project based on templates, using cargo-generate. Current templates include
https://github.com/leptos-rs/start: An Actix starter
https://github.com/leptos-rs/start-axum: An Axum starter
https://github.com/leptos-rs/start-axum-workspace: An Axum starter keeping client and server code in separate crates in a workspace
'no_downloads' feature to allow user management of optional dependencies
Getting started
Install:

cargo install --locked cargo-leptos

If you, for any reason, need the bleeding-edge super fresh version:

cargo install --git https://github.com/leptos-rs/cargo-leptos --locked cargo-leptos

Help:

cargo leptos --help

For setting up your project, have a look at the examples


Dependencies
The dependencies for sass, wasm-opt and cargo-generate are automatically installed in a cache directory when they are used if they are not already installed and found by which. Different versions of the dependencies might accumulate in this directory, so feel free to delete it.

OS	Example
Linux	/home/alice/.cache/cargo-leptos
macOS	/Users/Alice/Library/Caches/cargo-leptos
Windows	C:\Users\Alice\AppData\Local\cargo-leptos
If you wish to make it mandatory to install your dependencies, or are using Nix or NixOs, you can install it with the no_downloads feature enabled to prevent cargo-leptos from trying to download and install them.

cargo install --features no_downloads --locked cargo-leptos


Single-package setup
The single-package setup is where the code for both the frontend and the server is defined in a single package.

Configuration parameters are defined in the package Cargo.toml section [package.metadata.leptos]. See the Parameters reference for a full list of parameters that can be used. All paths are relative to the package root (i.e. to the Cargo.toml file)


Workspace setup
When using a workspace setup both single-package and multi-package projects are supported. The latter is when the frontend and the server reside in different packages.

All workspace members whose Cargo.toml define the [package.metadata.leptos] section are automatically included as Leptos single-package projects. The multi-package projects are defined on the workspace level in the Cargo.toml's section [[workspace.metadata.leptos]] which takes three mandatory parameters:

[[workspace.metadata.leptos]]
# project name
name = "leptos-project"
bin-package = "server"
lib-package = "front"

# more configuration parameters...
Note the double braces: several projects can be defined and one package can be used in several projects.


Build features
When building with cargo-leptos, the frontend, library package, is compiled into wasm using target wasm-unknown-unknown and the features --no-default-features --features=hydrate The server binary is compiled with the features --no-default-features --features=ssr


Parameters reference
These parameters are used either in the workspace section [[workspace.metadata.leptos]] or the package, for single-package setups, section [package.metadata.leptos].

Note that the Cargo Manifest uses the word target with two different meanings. As a package's configured [[bin]] targets and as the compiled output target triple. Here, the latter is referred to as target-triple.

Compilation parameters
# Sets the name of the binary target used.
#
# Optional, only necessary if the bin-package defines more than one target
bin-target = "my-bin-name"

# Enables additional file hashes on outputted css, js, and wasm files
#
# Optional: Defaults to false. Can also be set with the LEPTOS_HASH_FILES=false env var
hash-files = false

# Sets the name for the file cargo-leptos uses to track the most recent hashes
#
# Optional: Defaults to "hash.txt". Can also be set with the LEPTOS_HASH_FILE_NAME="hash.txt" env var
hash-file-name = false

# The features to use when compiling all targets
#
# Optional. Can be extended with the command line parameter --features
features = []

# The features to use when compiling the bin target
#
# Optional. Can be over-ridden with the command line parameter --bin-features
bin-features = ["ssr"]

# If the --no-default-features flag should be used when compiling the bin target
#
# Optional. Defaults to false.
bin-default-features = false

# The profile to use for the bin target when compiling for release
#
# Optional. Defaults to "release".
bin-profile-release = "my-release-profile"

# The profile to use for the bin target when compiling for debug
#
# Optional. Defaults to "debug".
bin-profile-dev = "my-debug-profile"

# The target triple to use when compiling the bin target
#
# Optional. Env: LEPTOS_BIN_TARGET_TRIPLE
bin-target-triple = "x86_64-unknown-linux-gnu"

# The features to use when compiling the lib target
#
# Optional. Can be over-ridden with the command line parameter --lib-features
lib-features = ["hydrate"]

# If the --no-default-features flag should be used when compiling the lib target
#
# Optional. Defaults to false.
lib-default-features = false

# The profile to use for the lib target when compiling for release
#
# Optional. Defaults to "release".
lib-profile-release = "my-release-profile"

# The profile to use for the lib target when compiling for debug
#
# Optional. Defaults to "debug".
lib-profile-dev = "my-debug-profile"

# Fixes cargo bug that prevents incremental compilation (see #203)
#
# Optional. Defaults to false prior to 0.2.3, unconditionally enabled (with the setting becoming deprecated) since 0.2.3 and #216
separate-front-target-dir = true

# Pass additional parameters to the cargo process compiling to WASM
# 
# Optional. No default
lib-cargo-args = ["--timings"]

# Pass additional parameters to the cargo process to build the server
# 
# Optional. No default
bin-cargo-args = ["--timings"]

# The command to run instead of "cargo" when building the server
#
# Optional. No default. Env: LEPTOS_BIN_CARGO_COMMAND
bin-cargo-command = "cross"
Site parameters
These parameters can be overridden by setting the corresponding environment variable. They can also be set in a .env file as cargo-leptos reads the first it finds in the package or workspace directory and any parent directory.

# Sets the name of the output js, wasm and css files.
#
# Optional, defaults to the lib package name or, in a workspace, the project name. Env: LEPTOS_OUTPUT_NAME.
output-name = "myproj"

# The site root folder is where cargo-leptos generate all output.
# NOTE: It is relative to the workspace root when running in a workspace.
# WARNING: all content of this folder will be erased on a rebuild!
#
# Optional, defaults to "/site" in the Cargo target directory. Env: LEPTOS_SITE_ROOT.
site-root = "target/site"

# The site-root relative folder where all compiled output (JS, WASM and CSS) is written.
#
# Optional, defaults to "pkg". Env: LEPTOS_SITE_PKG_DIR.
site-pkg-dir = "pkg"

# The source style file. If it ends with _.sass_ or _.scss_ then it will be compiled by `dart-sass`
# into CSS and processed by lightning css. When release is set, then it will also be minified.
#
# Optional. Env: LEPTOS_STYLE_FILE.
style-file = "style/main.scss"

# The tailwind input file.
#
# Optional, Activates the tailwind build
tailwind-input-file = "style/tailwind.css"

# The tailwind config file.
#
# Optional, defaults to "tailwind.config.js" which if is not present
# is generated for you
tailwind-config-file = "tailwind.config.js"

# The browserlist https://browsersl.ist query used for optimizing the CSS.
#
# Optional, defaults to "defaults". Env: LEPTOS_BROWSERQUERY.
browserquery = "defaults"

# Assets source dir. All files found here will be copied and synchronized to site-root.
# The assets-dir cannot have a sub directory with the same name/path as site-pkg-dir.
#
# Optional. Env: LEPTOS_ASSETS_DIR.
assets-dir = "assets"

# JS source dir. `wasm-bindgen` has the option to include JS snippets from JS files
# with `#[wasm_bindgen(module = "/js/foo.js")]`. A change in any JS file in this dir
# will trigger a rebuild.
#
# Optional. Defaults to "src"
js-dir = "src"

# Additional files your application could depends on.
# A change to any file in those directories will trigger a rebuild.
#
# Optional.
watch-additional-files = ["additional_files", "custom_config.json"]

# The IP and port where the server serves the content. Use it in your server setup.
#
# Optional, defaults to 127.0.0.1:3000. Env: LEPTOS_SITE_ADDR.
site-addr = "127.0.0.1:3000"

# The port number used by the reload server (only used in watch mode).
#
# Optional, defaults 3001. Env: LEPTOS_RELOAD_PORT
reload-port = 3001

# The command used for running end-to-end tests. See the section about End-to-end testing.
#
# Optional. Env: LEPTOS_END2END_CMD.
end2end-cmd = "npx playwright test"

# The directory from which the end-to-end tests are run.
#
# Optional. Env: LEPTOS_END2END_DIR
end2end-dir = "integration"

Environment variables
The following environment variables are set when compiling the lib (front) or bin (server) and when the server is run.

Echoed from the Leptos config:

LEPTOS_OUTPUT_NAME
LEPTOS_SITE_ROOT
LEPTOS_SITE_PKG_DIR
LEPTOS_SITE_ADDR
LEPTOS_RELOAD_PORT
Directories used when building:

LEPTOS_LIB_DIR: The path (relative to the working directory) to the library package
LEPTOS_BIN_DIR: The path (relative to the working directory) to the binary package
Note when using directories:

cargo-leptos changes the working directory to the project root or if in a workspace, the workspace root before building and running.
the two are set to the same value when running in a single-package config.
Avoid using them at run-time unless you can guarantee that the entire project struct is available at runtime as well.
Internally the versions of the external tools called by cargo-leptos are hardcoded. Use these environment variables to override the versions cargo-leptos should use (e.g. LEPTOS_SASS_VERSION=1.69.5):

LEPTOS_CARGO_GENERATE_VERSION
LEPTOS_TAILWIND_VERSION
LEPTOS_SASS_VERSION
LEPTOS_WASM_OPT_VERSION
End-to-end testing
cargo-leptos provides end-to-end testing support for convenience. It is a simple wrapper around a shell command end2end-cmd that is executed in a specific directory end2end-dir.

The end2end-cmd can be any shell command. For running Playwright it would be npx playwright test.

What it does is equivalent to running this manually:

in a terminal, run cargo leptos watch
in a separate terminal, change to the end2end-dir and run the end2end-cmd.
When testing the setup, please try the above first. If that works but cargo leptos end-to-end doesn't then please create a GitHub ticket.


The Life of a Page Load
Before we get into the weeds it might be helpful to have a higher-level overview. What exactly happens between the moment you type in the URL of a server-rendered Leptos app, and the moment you click a button and a counter increases?

I’m assuming some basic knowledge of how the Internet works here, and won’t get into the weeds about HTTP or whatever. Instead, I’ll try to show how different parts of the Leptos APIs map onto each part of the process.

This description also starts from the premise that your app is being compiled for two separate targets:

A server version, often running on Actix or Axum, compiled with the Leptos ssr feature
A browser version, compiled to WebAssembly (WASM) with the Leptos hydrate feature
The cargo-leptos build tool exists to coordinate the process of compiling your app for these two different targets.

On the Server
Your browser makes a GET request for that URL to your server. At this point, the browser knows almost nothing about the page that’s going to be rendered. (The question “How does the browser know where to ask for the page?” is an interesting one, but out of the scope of this tutorial!)
The server receives that request, and checks whether it has a way to handle a GET request at that path. This is what the .leptos_routes() methods in leptos_axum and leptos_actix are for. When the server starts up, these methods walk over the routing structure you provide in <Routes/>, generating a list of all possible routes your app can handle and telling the server’s router “for each of these routes, if you get a request... hand it off to Leptos.”
The server sees that this route can be handled by Leptos. So it renders your root component (often called something like <App/>), providing it with the URL that’s being requested and some other data like the HTTP headers and request metadata.
Your application runs once on the server, building up an HTML version of the component tree that will be rendered at that route. (There’s more to be said here about resources and <Suspense/> in the next chapter.)
The server returns this HTML page, also injecting information on how to load the version of your app that has been compiled to WASM so that it can run in the browser.
The HTML page that’s returned is essentially your app, “dehydrated” or “freeze-dried”: it is HTML without any of the reactivity or event listeners you’ve added. The browser will “rehydrate” this HTML page by adding the reactive system and attaching event listeners to that server-rendered HTML. Hence the two feature flags that apply to the two halves of this process: ssr on the server for “server-side rendering”, and hydrate in the browser for that process of rehydration.

In the Browser
The browser receives this HTML page from the server. It immediately goes back to the server to begin loading the JS and WASM necessary to run the interactive, client side version of the app.
In the meantime, it renders the HTML version.
When the WASM version has reloaded, it does the same route-matching process that the server did. Because the <Routes/> component is identical on the server and in the client, the browser version will read the URL and render the same page that was already returned by the server.
During this initial “hydration” phase, the WASM version of your app doesn’t re-create the DOM nodes that make up your application. Instead, it walks over the existing HTML tree, “picking up” existing elements and adding the necessary interactivity.
Note that there are some trade-offs here. Before this hydration process is complete, the page will appear interactive but won’t actually respond to interactions. For example, if you have a counter button and click it before WASM has loaded, the count will not increment, because the necessary event listeners and reactivity have not been added yet. We’ll look at some ways to build in “graceful degradation” in future chapters.

Client-Side Navigation
The next step is very important. Imagine that the user now clicks a link to navigate to another page in your application.

The browser will not make another round trip to the server, reloading the full page as it would for navigating between plain HTML pages or an application that uses server rendering (for example with PHP) but without a client-side half.

Instead, the WASM version of your app will load the new page, right there in the browser, without requesting another page from the server. Essentially, your app upgrades itself from a server-loaded “multi-page app” into a browser-rendered “single-page app.” This yields the best of both worlds: a fast initial load time due to the server-rendered HTML, and fast secondary navigations because of the client-side routing.

Some of what will be described in the following chapters—like the interactions between server functions, resources, and <Suspense/>—may seem overly complicated. You might find yourself asking, “If my page is being rendered to HTML on the server, why can’t I just .await this on the server? If I can just call library X in a server function, why can’t I call it in my component?” The reason is pretty simple: to enable the upgrade from server rendering to client rendering, everything in your application must be able to run either on the server or in the browser.

This is not the only way to create a website or web framework, of course. But it’s the most common way, and we happen to think it’s quite a good way, to create the smoothest possible experience for your users.


Async Rendering and SSR “Modes”
Server-rendering a page that uses only synchronous data is pretty simple: You just walk down the component tree, rendering each element to an HTML string. But this is a pretty big caveat: it doesn’t answer the question of what we should do with pages that includes asynchronous data, i.e., the sort of stuff that would be rendered under a <Suspense/> node on the client.

When a page loads async data that it needs to render, what should we do? Should we wait for all the async data to load, and then render everything at once? (Let’s call this “async” rendering) Should we go all the way in the opposite direction, just sending the HTML we have immediately down to the client and letting the client load the resources and fill them in? (Let’s call this “synchronous” rendering) Or is there some middle-ground solution that somehow beats them both? (Hint: There is.)

If you’ve ever listened to streaming music or watched a video online, I’m sure you realize that HTTP supports streaming, allowing a single connection to send chunks of data one after another without waiting for the full content to load. You may not realize that browsers are also really good at rendering partial HTML pages. Taken together, this means that you can actually enhance your users’ experience by streaming HTML: and this is something that Leptos supports out of the box, with no configuration at all. And there’s actually more than one way to stream HTML: you can stream the chunks of HTML that make up your page in order, like frames of a video, or you can stream them... well, out of order.

Let me say a little more about what I mean.

Leptos supports all the major ways of rendering HTML that includes asynchronous data:

Synchronous Rendering
Async Rendering
In-Order streaming
Out-of-Order Streaming (and a partially-blocked variant)
Synchronous Rendering
Synchronous: Serve an HTML shell that includes fallback for any <Suspense/>. Load data on the client using create_local_resource, replacing fallback once resources are loaded.
Pros: App shell appears very quickly: great TTFB (time to first byte).
Cons
Resources load relatively slowly; you need to wait for JS + WASM to load before even making a request.
No ability to include data from async resources in the <title> or other <meta> tags, hurting SEO and things like social media link previews.
If you’re using server-side rendering, the synchronous mode is almost never what you actually want, from a performance perspective. This is because it misses out on an important optimization. If you’re loading async resources during server rendering, you can actually begin loading the data on the server. Rather than waiting for the client to receive the HTML response, then loading its JS + WASM, then realize it needs the resources and begin loading them, server rendering can actually begin loading the resources when the client first makes the response. In this sense, during server rendering an async resource is like a Future that begins loading on the server and resolves on the client. As long as the resources are actually serializable, this will always lead to a faster total load time.

This is why create_resource requires resources data to be serializable by default, and why you need to explicitly use create_local_resource for any async data that is not serializable and should therefore only be loaded in the browser itself. Creating a local resource when you could create a serializable resource is always a deoptimization.

Async Rendering
async: Load all resources on the server. Wait until all data are loaded, and render HTML in one sweep.
Pros: Better handling for meta tags (because you know async data even before you render the <head>). Faster complete load than synchronous because async resources begin loading on server.
Cons: Slower load time/TTFB: you need to wait for all async resources to load before displaying anything on the client. The page is totally blank until everything is loaded.
In-Order Streaming
In-order streaming: Walk through the component tree, rendering HTML until you hit a <Suspense/>. Send down all the HTML you’ve got so far as a chunk in the stream, wait for all the resources accessed under the <Suspense/> to load, then render it to HTML and keep walking until you hit another <Suspense/> or the end of the page.
Pros: Rather than a blank screen, shows at least something before the data are ready.
Cons
Loads the shell more slowly than synchronous rendering (or out-of-order streaming) because it needs to pause at every <Suspense/>.
Unable to show fallback states for <Suspense/>.
Can’t begin hydration until the entire page has loaded, so earlier pieces of the page will not be interactive until the suspended chunks have loaded.
Out-of-Order Streaming
Out-of-order streaming: Like synchronous rendering, serve an HTML shell that includes fallback for any <Suspense/>. But load data on the server, streaming it down to the client as it resolves, and streaming down HTML for <Suspense/> nodes, which is swapped in to replace the fallback.
Pros: Combines the best of synchronous and async.
Fast initial response/TTFB because it immediately sends the whole synchronous shell
Fast total time because resources begin loading on the server.
Able to show the fallback loading state and dynamically replace it, instead of showing blank sections for un-loaded data.
Cons: Requires JavaScript to be enabled for suspended fragments to appear in correct order. (This small chunk of JS streamed down in a <script> tag alongside the <template> tag that contains the rendered <Suspense/> fragment, so it does not need to load any additional JS files.)
Partially-blocked streaming: “Partially-blocked” streaming is useful when you have multiple separate <Suspense/> components on the page. It is triggered by setting ssr=SsrMode::PartiallyBlocked on a route, and depending on blocking resources within the view. If one of the <Suspense/> components reads from one or more “blocking resources” (see below), the fallback will not be sent; rather, the server will wait until that <Suspense/> has resolved and then replace the fallback with the resolved fragment on the server, which means that it is included in the initial HTML response and appears even if JavaScript is disabled or not supported. Other <Suspense/> stream in out of order, similar to the SsrMode::OutOfOrder default.
This is useful when you have multiple <Suspense/> on the page, and one is more important than the other: think of a blog post and comments, or product information and reviews. It is not useful if there’s only one <Suspense/>, or if every <Suspense/> reads from blocking resources. In those cases it is a slower form of async rendering.

Pros: Works if JavaScript is disabled or not supported on the user’s device.
Cons
Slower initial response time than out-of-order.
Marginally overall response due to additional work on the server.
No fallback state shown.
Using SSR Modes
Because it offers the best blend of performance characteristics, Leptos defaults to out-of-order streaming. But it’s really simple to opt into these different modes. You do it by adding an ssr property onto one or more of your <Route/> components, like in the ssr_modes example.

<Routes>
    // We’ll load the home page with out-of-order streaming and <Suspense/>
    <Route path="" view=HomePage/>

    // We'll load the posts with async rendering, so they can set
    // the title and metadata *after* loading the data
    <Route
        path="/post/:id"
        view=Post
        ssr=SsrMode::Async
    />
</Routes>
For a path that includes multiple nested routes, the most restrictive mode will be used: i.e., if even a single nested route asks for async rendering, the whole initial request will be rendered async. async is the most restricted requirement, followed by in-order, and then out-of-order. (This probably makes sense if you think about it for a few minutes.)

Blocking Resources
Any Leptos versions later than 0.2.5 (i.e., git main and 0.3.x or later) introduce a new resource primitive with create_blocking_resource. A blocking resource still loads asynchronously like any other async/.await in Rust; it doesn’t block a server thread or anything. Instead, reading from a blocking resource under a <Suspense/> blocks the HTML stream from returning anything, including its initial synchronous shell, until that <Suspense/> has resolved.

Now from a performance perspective, this is not ideal. None of the synchronous shell for your page will load until that resource is ready. However, rendering nothing means that you can do things like set the <title> or <meta> tags in your <head> in actual HTML. This sounds a lot like async rendering, but there’s one big difference: if you have multiple <Suspense/> sections, you can block on one of them but still render a placeholder and then stream in the other.

For example, think about a blog post. For SEO and for social sharing, I definitely want my blog post’s title and metadata in the initial HTML <head>. But I really don’t care whether comments have loaded yet or not; I’d like to load those as lazily as possible.

With blocking resources, I can do something like this:

#[component]
pub fn BlogPost() -> impl IntoView {
let post_data = create_blocking_resource(/* load blog post */);
let comments_data = create_resource(/* load blog comments */);
view! {
<Suspense fallback=|| ()>
{move || {
post_data.with(|data| {
view! {
<Title text=data.title/>
<Meta name="description" content=data.excerpt/>
<article>
/* render the post content */
</article>
}
})
}}
</Suspense>
<Suspense fallback=|| "Loading comments...">
/* render comments data here */
</Suspense>
}
}
The first <Suspense/>, with the body of the blog post, will block my HTML stream, because it reads from a blocking resource. Meta tags and other head elements awaiting the blocking resource will be rendered before the stream is sent.

Combined with the following route definition, which uses SsrMode::PartiallyBlocked, the blocking resource will be fully rendered on the server side, making it accessible to users who disable WebAssembly or JavaScript.

<Routes>
    // We’ll load the home page with out-of-order streaming and <Suspense/>
    <Route path="" view=HomePage/>

    // We'll load the posts with async rendering, so they can set
    // the title and metadata *after* loading the data
    <Route
        path="/post/:id"
        view=Post
        ssr=SsrMode::PartiallyBlocked
    />
</Routes>
The second <Suspense/>, with the comments, will not block the stream. Blocking resources gave me exactly the power and granularity I needed to optimize my page for SEO and user experience.


Where do you expect where do I run? to log?

In the command line where you’re running the server?
In the browser console when you load the page?
Neither?
Both?
Try it out.

...

...

...

Okay, consider the spoiler alerted.

You’ll notice of course that it logs in both places, assuming everything goes according to plan. In fact on the server it logs twice—first during the initial server startup, when Leptos renders your app once to extract the route tree, then a second time when you make a request. Each time you reload the page, where do I run? should log once on the server and once on the client.

If you think about the description in the last couple sections, hopefully this makes sense. Your application runs once on the server, where it builds up a tree of HTML which is sent to the client. During this initial render, where do I run? logs on the server.

Once the WASM binary has loaded in the browser, your application runs a second time, walking over the same user interface tree and adding interactivity.

Does that sound like a waste? It is, in a sense. But reducing that waste is a genuinely hard problem. It’s what some JS frameworks like Qwik are intended to solve, although it’s probably too early to tell whether it’s a net performance gain as opposed to other approaches.

The Potential for Bugs
Okay, hopefully all of that made sense. But what does it have to do with the title of this chapter, which is “Hydration bugs (and how to avoid them)”?

Remember that the application needs to run on both the server and the client. This generates a few different sets of potential issues you need to know how to avoid.

Mismatches between server and client code
One way to create a bug is by creating a mismatch between the HTML that’s sent down by the server and what’s rendered on the client. It’s actually fairly hard to do this unintentionally, I think (at least judging by the bug reports I get from people.) But imagine I do something like this

#[component]
pub fn App() -> impl IntoView {
let data = if cfg!(target_arch = "wasm32") {
vec![0, 1, 2]
} else {
vec![]
};
data.into_iter()
.map(|value| view! { <span>{value}</span> })
.collect_view()
}
In other words, if this is being compiled to WASM, it has three items; otherwise it’s empty.

When I load the page in the browser, I see nothing. If I open the console I see a bunch of warnings:

element with id 0-3 not found, ignoring it for hydration
element with id 0-4 not found, ignoring it for hydration
element with id 0-5 not found, ignoring it for hydration
component with id _0-6c not found, ignoring it for hydration
component with id _0-6o not found, ignoring it for hydration
The WASM version of your app, running in the browser, expects to find three items; but the HTML has none.

Solution
It’s pretty rare that you do this intentionally, but it could happen from somehow running different logic on the server and in the browser. If you’re seeing warnings like this and you don’t think it’s your fault, it’s much more likely that it’s a bug with <Suspense/> or something. Feel free to go ahead and open an issue or discussion on GitHub for help.

Not all client code can run on the server
Imagine you happily import a dependency like gloo-net that you’ve been used to using to make requests in the browser, and use it in a create_resource in a server-rendered app.

You’ll probably instantly see the dreaded message

panicked at 'cannot call wasm-bindgen imported functions on non-wasm targets'
Uh-oh.

But of course this makes sense. We’ve just said that your app needs to run on the client and the server.

Solution
There are a few ways to avoid this:

Only use libraries that can run on both the server and the client. reqwest, for example, works for making HTTP requests in both settings.
Use different libraries on the server and the client, and gate them using the #[cfg] macro. (Click here for an example.)
Wrap client-only code in create_effect. Because create_effect only runs on the client, this can be an effective way to access browser APIs that are not needed for initial rendering.
For example, say that I want to store something in the browser’s localStorage whenever a signal changes.

#[component]
pub fn App() -> impl IntoView {
use gloo_storage::Storage;
let storage = gloo_storage::LocalStorage::raw();
logging::log!("{storage:?}");
}
This panics because I can’t access LocalStorage during server rendering.

But if I wrap it in an effect...

#[component]
pub fn App() -> impl IntoView {
use gloo_storage::Storage;
create_effect(move |_| {
let storage = gloo_storage::LocalStorage::raw();
logging::log!("{storage:?}");
});
}
It’s fine! This will render appropriately on the server, ignoring the client-only code, and then access the storage and log a message on the browser.

Not all server code can run on the client
WebAssembly running in the browser is a pretty limited environment. You don’t have access to a file-system or to many of the other things the standard library may be used to having. Not every crate can even be compiled to WASM, let alone run in a WASM environment.

In particular, you’ll sometimes see errors about the crate mio or missing things from core. This is generally a sign that you are trying to compile something to WASM that can’t be compiled to WASM. If you’re adding server-only dependencies, you’ll want to mark them optional = true in your Cargo.toml and then enable them in the ssr feature definition. (Check out one of the template Cargo.toml files to see more details.)

You can use create_effect to specify that something should only run on the client, and not in the server. Is there a way to specify that something should run only on the server, and not the client?

In fact, there is. The next chapter will cover the topic of server functions in some detail. (In the meantime, you can check out their docs here.)

Crate leptos_serverCopy item path
source · [−]
Structs
Action	An action synchronizes an imperative async call to the synchronous reactive system.
MultiAction	An action that synchronizes multiple imperative async calls to the reactive system, tracking the progress of each one.
Submission	An action that has been submitted by dispatching it to a MultiAction.
Enums
ServerFnError	Type for errors that can occur when using server functions.
ServerFnErrorErr	Type for errors that can occur when using server functions.
Functions
create_action	Creates an Action to synchronize an imperative async call to the synchronous reactive system.
create_multi_action	Creates an MultiAction to synchronize an imperative async call to the synchronous reactive system.
create_server_action	Creates an Action that can be used to call a server function.
create_server_multi_action	Creates an MultiAction that can be used to call a server function.

Struct leptos_server::ActionCopy item path
source · [−]
pub struct Action<I, O>(/* private fields */)
where
I: 'static,
O: 'static;
An action synchronizes an imperative async call to the synchronous reactive system.

If you’re trying to load data by running an async function reactively, you probably want to use a Resource instead. If you’re trying to occasionally run an async function in response to something like a user clicking a button, you’re in the right place.

async fn send_new_todo_to_api(task: String) -> usize {
// do something...
// return a task id
42
}
let save_data = create_action(|task: &String| {
// `task` is given as `&String` because its value is available in `input`
send_new_todo_to_api(task.clone())
});

// the argument currently running
let input = save_data.input();
// the most recent returned result
let result_of_call = save_data.value();
// whether the call is pending
let pending = save_data.pending();
// how many times the action has run
// useful for reactively updating something else in response to a `dispatch` and response
let version = save_data.version();

// before we do anything
assert_eq!(input.get(), None); // no argument yet
assert_eq!(pending.get(), false); // isn't pending a response
assert_eq!(result_of_call.get(), None); // there's no "last value"
assert_eq!(version.get(), 0);
// dispatch the action
save_data.dispatch("My todo".to_string());

// when we're making the call
// assert_eq!(input.get(), Some("My todo".to_string()));
// assert_eq!(pending.get(), true); // is pending
// assert_eq!(result_of_call.get(), None); // has not yet gotten a response

// after call has resolved
assert_eq!(input.get(), None); // input clears out after resolved
assert_eq!(pending.get(), false); // no longer pending
assert_eq!(result_of_call.get(), Some(42));
assert_eq!(version.get(), 1);
The input to the async function should always be a single value, but it can be of any type. The argument is always passed by reference to the function, because it is stored in Action::input as well.

// if there's a single argument, just use that
let action1 = create_action(|input: &String| {
let input = input.clone();
async move { todo!() }
});

// if there are no arguments, use the unit type `()`
let action2 = create_action(|input: &()| async { todo!() });

// if there are multiple arguments, use a tuple
let action3 = create_action(|input: &(usize, String)| async { todo!() });
Implementations
source
impl<I, O> Action<I, O>
where
I: 'static,
O: 'static,
source
pub fn dispatch(&self, input: I)
Calls the async function with a reference to the input type as its argument.

source
pub fn new<F, Fu>(action_fn: F) -> Self
where
F: Fn(&I) -> Fu + 'static,
Fu: Future<Output = O> + 'static,
Create an Action.

Action is a type of Signal which represent imperative calls to an asynchronous function. Where a Resource is driven as a function of a Signal, Actions are Action::dispatched by events or handlers.


let act = Action::new(|n: &u8| {
let n = n.to_owned();
async move { n * 2 }
});
act.dispatch(3);
assert_eq!(act.value().get(), Some(6));

// Remember that async functions already return a future if they are
// not `await`ed. You can save keystrokes by leaving out the `async move`

let act2 = Action::new(|n: &String| yell(n.to_owned()));
act2.dispatch(String::from("i'm in a doctest"));
assert_eq!(act2.value().get(), Some("I'M IN A DOCTEST".to_string()));

async fn yell(n: String) -> String {
n.to_uppercase()
}
source
pub fn pending(&self) -> ReadSignal<bool>
Whether the action has been dispatched and is currently waiting for its future to be resolved.

source
pub fn set_pending(&self, pending: bool)
Updates whether the action is currently pending. If the action has been dispatched multiple times, and some of them are still pending, it will not update the pending signal.

source
pub fn url(&self) -> Option<String>
The URL associated with the action (typically as part of a server function.) This enables integration with the ActionForm component in leptos_router.

source
pub fn version(&self) -> RwSignal<usize>
How many times the action has successfully resolved.

source
pub fn input(&self) -> RwSignal<Option<I>>
The current argument that was dispatched to the async function. Some while we are waiting for it to resolve, None if it has resolved.

source
pub fn value(&self) -> RwSignal<Option<O>>
The most recent return value of the async function.

source
impl<I> Action<I, Result<I::Output, ServerFnError<I::Error>>>
where
I: ServerFn + 'static,
source
pub fn server() -> Action<I, Result<I::Output, ServerFnError<I::Error>>>
where
I: ServerFn + Clone,
I::Error: Clone + 'static,
Create an Action to imperatively call a server function.

The struct representing your server function’s arguments should be provided to the Action. Unless specified as an argument to the server macro, the generated struct is your function’s name converted to CamelCase.


// The type argument can be on the right of the equal sign.
let act = Action::<Add, _>::server();
let args = Add { lhs: 5, rhs: 7 };
act.dispatch(args);
assert_eq!(act.value().get(), Some(Ok(12)));

// Or on the left of the equal sign.
let act: Action<Sub, _> = Action::server();
let args = Sub { lhs: 20, rhs: 5 };
act.dispatch(args);
assert_eq!(act.value().get(), Some(Ok(15)));

let not_dispatched = Action::<Add, _>::server();
assert_eq!(not_dispatched.value().get(), None);

#[server]
async fn add(lhs: u8, rhs: u8) -> Result<u8, ServerFnError> {
Ok(lhs + rhs)
}

#[server]
async fn sub(lhs: u8, rhs: u8) -> Result<u8, ServerFnError> {
Ok(lhs - rhs)
}
source
pub fn using_server_fn(self) -> Self
where
I::Error: Clone + 'static,
Associates the URL of the given server function with this action. This enables integration with the ActionForm component in leptos_router.

Trait Implementations
source
impl<I, O> Clone for Action<I, O>
where
I: 'static,
O: 'static,
source
fn clone(&self) -> Self
Returns a copy of the value. Read more
1.0.0 · source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
source
impl<I, O> Copy for Action<I, O>
where
I: 'static,
O: 'static,
Auto Trait Implementations
impl<I, O> Freeze for Action<I, O>
impl<I, O> !RefUnwindSafe for Action<I, O>
impl<I, O> !Send for Action<I, O>
impl<I, O> !Sync for Action<I, O>
impl<I, O> Unpin for Action<I, O>
where
I: Unpin,
O: Unpin,
impl<I, O> !UnwindSafe for Action<I, O>
Blanket Implementations
source
impl<T> Any for T
where
T: 'static + ?Sized,
source
impl<T> Borrow<T> for T
where
T: ?Sized,
source
impl<T> BorrowMut<T> for T
where
T: ?Sized,
source
impl<T> From<T> for T
source
impl<T> Instrument for T
source
impl<T, U> Into<U> for T
where
U: From<T>,
source
impl<T> ToOwned for T
where
T: Clone,
source
impl<T, U> TryFrom<U> for T
where
U: Into<T>,
source
impl<T, U> TryInto<U> for T
where
U: TryFrom<T>,
source
impl<T> WithSubscriber for T

Struct leptos_server::MultiActionCopy item path
source · [−]
pub struct MultiAction<I, O>(/* private fields */)
where
I: 'static,
O: 'static;
An action that synchronizes multiple imperative async calls to the reactive system, tracking the progress of each one.

Where an Action fires a single call, a MultiAction allows you to keep track of multiple in-flight actions.

If you’re trying to load data by running an async function reactively, you probably want to use a Resource instead. If you’re trying to occasionally run an async function in response to something like a user adding a task to a todo list, you’re in the right place.

async fn send_new_todo_to_api(task: String) -> usize {
// do something...
// return a task id
42
}
let add_todo = create_multi_action(|task: &String| {
// `task` is given as `&String` because its value is available in `input`
send_new_todo_to_api(task.clone())
});

add_todo.dispatch("Buy milk".to_string());
add_todo.dispatch("???".to_string());
add_todo.dispatch("Profit!!!".to_string());
The input to the async function should always be a single value, but it can be of any type. The argument is always passed by reference to the function, because it is stored in Submission::input as well.

// if there's a single argument, just use that
let action1 = create_multi_action(|input: &String| {
let input = input.clone();
async move { todo!() }
});

// if there are no arguments, use the unit type `()`
let action2 = create_multi_action(|input: &()| async { todo!() });

// if there are multiple arguments, use a tuple
let action3 =
create_multi_action(|input: &(usize, String)| async { todo!() });
Implementations
source
impl<I, O> MultiAction<I, O>
where
I: 'static,
O: 'static,
source
pub fn dispatch(&self, input: I)
Calls the async function with a reference to the input type as its argument.

source
pub fn submissions(&self) -> ReadSignal<Vec<Submission<I, O>>>
The set of all submissions to this multi-action.

source
pub fn url(&self) -> Option<String>
The URL associated with the action (typically as part of a server function.) This enables integration with the MultiActionForm component in leptos_router.

source
pub fn version(&self) -> RwSignal<usize>
How many times an action has successfully resolved.

source
pub fn using_server_fn<T: ServerFn>(self) -> Self
Associates the URL of the given server function with this action. This enables integration with the MultiActionForm component in leptos_router.

Trait Implementations
source
impl<I, O> Clone for MultiAction<I, O>
where
I: 'static,
O: 'static,
source
fn clone(&self) -> Self
Returns a copy of the value. Read more
1.0.0 · source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
source
impl<I, O> Copy for MultiAction<I, O>
where
I: 'static,
O: 'static,
Auto Trait Implementations
impl<I, O> Freeze for MultiAction<I, O>
impl<I, O> !RefUnwindSafe for MultiAction<I, O>
impl<I, O> !Send for MultiAction<I, O>
impl<I, O> !Sync for MultiAction<I, O>
impl<I, O> Unpin for MultiAction<I, O>
where
I: Unpin,
O: Unpin,
impl<I, O> !UnwindSafe for MultiAction<I, O>
Blanket Implementations
source
impl<T> Any for T
where
T: 'static + ?Sized,
source
impl<T> Borrow<T> for T
where
T: ?Sized,
source
impl<T> BorrowMut<T> for T
where
T: ?Sized,
source
impl<T> From<T> for T
source
impl<T> Instrument for T
source
impl<T, U> Into<U> for T
where
U: From<T>,
source
impl<T> ToOwned for T
where
T: Clone,
source
impl<T, U> TryFrom<U> for T
where
U: Into<T>,
source
impl<T, U> TryInto<U> for T
where
U: TryFrom<T>,
source
impl<T> WithSubscriber for T

Struct leptos_server::SubmissionCopy item path
source · [−]
pub struct Submission<I, O>
where
I: 'static,
O: 'static,
{
pub input: RwSignal<Option<I>>,
pub value: RwSignal<Option<O>>,
pub canceled: RwSignal<bool>,
/* private fields */
}
An action that has been submitted by dispatching it to a MultiAction.

Fields
input: RwSignal<Option<I>>
The current argument that was dispatched to the async function. Some while we are waiting for it to resolve, None if it has resolved.

value: RwSignal<Option<O>>
The most recent return value of the async function.

canceled: RwSignal<bool>
Controls this submission has been canceled.

Implementations
source
impl<I, O> Submission<I, O>
where
I: 'static,
O: 'static,
source
pub fn pending(&self) -> ReadSignal<bool>
Whether this submission is currently waiting to resolve.

source
pub fn cancel(&self)
Cancels the submission, preventing it from resolving.

Trait Implementations
source
impl<I, O> Clone for Submission<I, O>
source
fn clone(&self) -> Self
Returns a copy of the value. Read more
1.0.0 · source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
source
impl<I, O> Copy for Submission<I, O>
Auto Trait Implementations
impl<I, O> Freeze for Submission<I, O>
impl<I, O> RefUnwindSafe for Submission<I, O>
where
I: RefUnwindSafe,
O: RefUnwindSafe,
impl<I, O> Send for Submission<I, O>
where
I: Send,
O: Send,
impl<I, O> Sync for Submission<I, O>
where
I: Sync,
O: Sync,
impl<I, O> Unpin for Submission<I, O>
where
I: Unpin,
O: Unpin,
impl<I, O> UnwindSafe for Submission<I, O>
where
I: UnwindSafe,
O: UnwindSafe,
Blanket Implementations
source
impl<T> Any for T
where
T: 'static + ?Sized,
source
impl<T> Borrow<T> for T
where
T: ?Sized,
source
impl<T> BorrowMut<T> for T
where
T: ?Sized,
source
impl<T> From<T> for T
source
impl<T> Instrument for T
source
impl<T, U> Into<U> for T
where
U: From<T>,
source
impl<T> ToOwned for T
where
T: Clone,
source
impl<T, U> TryFrom<U> for T
where
U: Into<T>,
source
impl<T, U> TryInto<U> for T
where
U: TryFrom<T>,
source
impl<T> WithSubscriber for T

Enum leptos_server::ServerFnErrorCopy item path
source · [−]
pub enum ServerFnError<E = NoCustomError> {
WrappedServerError(E),
Registration(String),
Request(String),
Response(String),
ServerError(String),
Deserialization(String),
Serialization(String),
Args(String),
MissingArg(String),
}
Type for errors that can occur when using server functions.

Unlike ServerFnErrorErr, this does not implement Error. This means that other error types can easily be converted into it using the ? operator.

Variants
WrappedServerError(E)
A user-defined custom error type, which defaults to NoCustomError.

Registration(String)
Error while trying to register the server function (only occurs in case of poisoned RwLock).

Request(String)
Occurs on the client if there is a network error while trying to run function on server.

Response(String)
Occurs on the server if there is an error creating an HTTP response.

ServerError(String)
Occurs when there is an error while actually running the function on the server.

Deserialization(String)
Occurs on the client if there is an error deserializing the server’s response.

Serialization(String)
Occurs on the client if there is an error serializing the server function arguments.

Args(String)
Occurs on the server if there is an error deserializing one of the arguments that’s been sent.

MissingArg(String)
Occurs on the server if there’s a missing argument.

Implementations
source
impl ServerFnError
source
pub fn new(msg: impl ToString) -> ServerFnError
Constructs a new ServerFnError::ServerError from some other type.

Trait Implementations
source
impl<E> Clone for ServerFnError<E>
where
E: Clone,
source
fn clone(&self) -> ServerFnError<E>
Returns a copy of the value. Read more
1.0.0 · source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
source
impl<E> Debug for ServerFnError<E>
where
E: Debug,
source
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
Formats the value using the given formatter. Read more
source
impl<'de, E> Deserialize<'de> for ServerFnError<E>
where
E: Deserialize<'de>,
source
fn deserialize<__D>(
__deserializer: __D
) -> Result<ServerFnError<E>, <__D as Deserializer<'de>>::Error>
where
__D: Deserializer<'de>,
Deserialize this value from the given Serde deserializer. Read more
source
impl<CustErr> Display for ServerFnError<CustErr>
where
CustErr: Display,
source
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
Formats the value using the given formatter. Read more
source
impl<E> Error for ServerFnError<E>
where
E: Error + 'static,
ServerFnError<E>: Display,
source
fn source(&self) -> Option<&(dyn Error + 'static)>
The lower-level source of this error, if any. Read more
1.0.0 · source
fn description(&self) -> &str
👎Deprecated since 1.42.0: use the Display impl or to_string()
Read more
1.0.0 · source
fn cause(&self) -> Option<&dyn Error>
👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
source
fn provide<'a>(&'a self, request: &mut Request<'a>)
🔬This is a nightly-only experimental API. (error_generic_member_access)
Provides type based access to context intended for error reports. Read more
source
impl<CustErr> From<CustErr> for ServerFnError<CustErr>
source
fn from(value: CustErr) -> ServerFnError<CustErr>
Converts to this type from the input type.
source
impl<E> From<E> for ServerFnError
where
E: Error,
source
fn from(value: E) -> ServerFnError
Converts to this type from the input type.
source
impl<CustErr> From<ServerFnError<CustErr>> for ServerFnErrorErr<CustErr>
source
fn from(value: ServerFnError<CustErr>) -> ServerFnErrorErr<CustErr>
Converts to this type from the input type.
source
impl<CustErr> From<ServerFnUrlError<CustErr>> for ServerFnError<CustErr>
source
fn from(error: ServerFnUrlError<CustErr>) -> ServerFnError<CustErr>
Converts to this type from the input type.
source
impl<E> PartialEq for ServerFnError<E>
where
E: PartialEq,
source
fn eq(&self, other: &ServerFnError<E>) -> bool
This method tests for self and other values to be equal, and is used by ==.
1.0.0 · source
fn ne(&self, other: &Rhs) -> bool
This method tests for !=. The default implementation is almost always sufficient, and should not be overridden without very good reason.
source
impl<E> Serialize for ServerFnError<E>
where
E: Serialize,
source
fn serialize<__S>(
&self,
__serializer: __S
) -> Result<<__S as Serializer>::Ok, <__S as Serializer>::Error>
where
__S: Serializer,
Serialize this value into the given Serde serializer. Read more
source
impl<CustErr> ServerFnErrorSerde for ServerFnError<CustErr>
where
CustErr: FromStr + Display,
source
fn ser(&self) -> Result<String, Error>
Converts the custom error type to a String.
source
fn de(data: &str) -> ServerFnError<CustErr>
Deserializes the custom error type from a String.
source
impl<E> Eq for ServerFnError<E>
where
E: Eq,
source
impl<E> StructuralPartialEq for ServerFnError<E>
Auto Trait Implementations
impl<E> Freeze for ServerFnError<E>
where
E: Freeze,
impl<E> RefUnwindSafe for ServerFnError<E>
where
E: RefUnwindSafe,
impl<E> Send for ServerFnError<E>
where
E: Send,
impl<E> Sync for ServerFnError<E>
where
E: Sync,
impl<E> Unpin for ServerFnError<E>
where
E: Unpin,
impl<E> UnwindSafe for ServerFnError<E>
where
E: UnwindSafe,
Blanket Implementations
source
impl<T> Any for T
where
T: 'static + ?Sized,
source
impl<T> Borrow<T> for T
where
T: ?Sized,
source
impl<T> BorrowMut<T> for T
where
T: ?Sized,
source
impl<Q, K> Equivalent<K> for Q
where
Q: Eq + ?Sized,
K: Borrow<Q> + ?Sized,
source
impl<Q, K> Equivalent<K> for Q
where
Q: Eq + ?Sized,
K: Borrow<Q> + ?Sized,
source
impl<T> From<!> for T
source
impl<T> From<T> for T
source
impl<CustErr, T, Request> FromReq<Cbor, Request, CustErr> for T
where
Request: Req<CustErr> + Send + 'static,
T: DeserializeOwned,
source
impl<CustErr, T, Request> FromReq<Json, Request, CustErr> for T
where
Request: Req<CustErr> + Send + 'static,
T: DeserializeOwned,
source
impl<CustErr, T, Request> FromReq<Streaming, Request, CustErr> for T
where
Request: Req<CustErr> + Send + 'static,
T: From<ByteStream> + 'static,
source
impl<CustErr, T, Request> FromReq<StreamingText, Request, CustErr> for T
where
Request: Req<CustErr> + Send + 'static,
T: From<TextStream> + 'static,
source
impl<CustErr, T, Response> FromRes<Cbor, Response, CustErr> for T
where
Response: ClientRes<CustErr> + Send,
T: DeserializeOwned + Send,
source
impl<CustErr, T, Response> FromRes<Json, Response, CustErr> for T
where
Response: ClientRes<CustErr> + Send,
T: DeserializeOwned + Send,
source
impl<T> Instrument for T
source
impl<T, U> Into<U> for T
where
U: From<T>,
source
impl<CustErr, T, Request> IntoReq<Cbor, Request, CustErr> for T
where
Request: ClientReq<CustErr>,
T: Serialize + Send,
source
impl<CustErr, T, Request> IntoReq<Json, Request, CustErr> for T
where
Request: ClientReq<CustErr>,
T: Serialize + Send,
source
impl<CustErr, T, Response> IntoRes<Cbor, Response, CustErr> for T
where
Response: Res<CustErr>,
T: Serialize + Send,
source
impl<CustErr, T, Response> IntoRes<Json, Response, CustErr> for T
where
Response: Res<CustErr>,
T: Serialize + Send,
source
impl<T> Serializable for T
where
T: DeserializeOwned + Serialize,
source
impl<T> ToOwned for T
where
T: Clone,
source
impl<T> ToString for T
where
T: Display + ?Sized,
source
impl<T, U> TryFrom<U> for T
where
U: Into<T>,
source
impl<T, U> TryInto<U> for T
where
U: TryFrom<T>,
source
impl<T> WithSubscriber for T
source
impl<T> DeserializeOwned for T
where
T: for<'de> Deserialize<'de>,

Enum leptos_server::ServerFnErrorErrCopy item path
source · [−]
pub enum ServerFnErrorErr<E = NoCustomError> {
WrappedServerError(E),
Registration(String),
Request(String),
ServerError(String),
Deserialization(String),
Serialization(String),
Args(String),
MissingArg(String),
Response(String),
}
Type for errors that can occur when using server functions.

Unlike ServerFnError, this implements std::error::Error. This means it can be used in situations in which the Error trait is required, but it’s not possible to create a blanket implementation that converts other errors into this type.

ServerFnError and ServerFnErrorErr mutually implement From, so it is easy to convert between the two types.

Variants
WrappedServerError(E)
A user-defined custom error type, which defaults to NoCustomError.

Registration(String)
Error while trying to register the server function (only occurs in case of poisoned RwLock).

Request(String)
Occurs on the client if there is a network error while trying to run function on server.

ServerError(String)
Occurs when there is an error while actually running the function on the server.

Deserialization(String)
Occurs on the client if there is an error deserializing the server’s response.

Serialization(String)
Occurs on the client if there is an error serializing the server function arguments.

Args(String)
Occurs on the server if there is an error deserializing one of the arguments that’s been sent.

MissingArg(String)
Occurs on the server if there’s a missing argument.

Response(String)
Occurs on the server if there is an error creating an HTTP response.

Trait Implementations
source
impl<E> Clone for ServerFnErrorErr<E>
where
E: Clone,
source
fn clone(&self) -> ServerFnErrorErr<E>
Returns a copy of the value. Read more
1.0.0 · source
fn clone_from(&mut self, source: &Self)
Performs copy-assignment from source. Read more
source
impl<E> Debug for ServerFnErrorErr<E>
where
E: Debug,
source
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
Formats the value using the given formatter. Read more
source
impl<E> Display for ServerFnErrorErr<E>
where
E: Display,
source
fn fmt(&self, __formatter: &mut Formatter<'_>) -> Result<(), Error>
Formats the value using the given formatter. Read more
source
impl<E> Error for ServerFnErrorErr<E>
where
ServerFnErrorErr<E>: Debug + Display,
1.30.0 · source
fn source(&self) -> Option<&(dyn Error + 'static)>
The lower-level source of this error, if any. Read more
1.0.0 · source
fn description(&self) -> &str
👎Deprecated since 1.42.0: use the Display impl or to_string()
Read more
1.0.0 · source
fn cause(&self) -> Option<&dyn Error>
👎Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
source
fn provide<'a>(&'a self, request: &mut Request<'a>)
🔬This is a nightly-only experimental API. (error_generic_member_access)
Provides type based access to context intended for error reports. Read more
source
impl<CustErr> From<ServerFnError<CustErr>> for ServerFnErrorErr<CustErr>
source
fn from(value: ServerFnError<CustErr>) -> ServerFnErrorErr<CustErr>
Converts to this type from the input type.
source
impl<CustErr> From<ServerFnUrlError<CustErr>> for ServerFnErrorErr<CustErr>
source
fn from(error: ServerFnUrlError<CustErr>) -> ServerFnErrorErr<CustErr>
Converts to this type from the input type.
source
impl<E> PartialEq for ServerFnErrorErr<E>
where
E: PartialEq,
source
fn eq(&self, other: &ServerFnErrorErr<E>) -> bool
This method tests for self and other values to be equal, and is used by ==.
1.0.0 · source
fn ne(&self, other: &Rhs) -> bool
This method tests for !=. The default implementation is almost always sufficient, and should not be overridden without very good reason.
source
impl<E> Eq for ServerFnErrorErr<E>
where
E: Eq,
source
impl<E> StructuralPartialEq for ServerFnErrorErr<E>
Auto Trait Implementations
impl<E> Freeze for ServerFnErrorErr<E>
where
E: Freeze,
impl<E> RefUnwindSafe for ServerFnErrorErr<E>
where
E: RefUnwindSafe,
impl<E> Send for ServerFnErrorErr<E>
where
E: Send,
impl<E> Sync for ServerFnErrorErr<E>
where
E: Sync,
impl<E> Unpin for ServerFnErrorErr<E>
where
E: Unpin,
impl<E> UnwindSafe for ServerFnErrorErr<E>
where
E: UnwindSafe,
Blanket Implementations
source
impl<T> Any for T
where
T: 'static + ?Sized,
source
impl<T> Borrow<T> for T
where
T: ?Sized,
source
impl<T> BorrowMut<T> for T
where
T: ?Sized,
source
impl<Q, K> Equivalent<K> for Q
where
Q: Eq + ?Sized,
K: Borrow<Q> + ?Sized,
source
impl<Q, K> Equivalent<K> for Q
where
Q: Eq + ?Sized,
K: Borrow<Q> + ?Sized,
source
impl<T> From<T> for T
source
impl<T> Instrument for T
source
impl<T, U> Into<U> for T
where
U: From<T>,
source
impl<T> ToOwned for T
where
T: Clone,
source
impl<T> ToString for T
where
T: Display + ?Sized,
source
impl<T, U> TryFrom<U> for T
where
U: Into<T>,
source
impl<T, U> TryInto<U> for T
where
U: TryFrom<T>,
source
impl<T> WithSubscriber for T

Function leptos_server::create_actionCopy item path
source · [−]
pub fn create_action<I, O, F, Fu>(action_fn: F) -> Action<I, O>
where
I: 'static,
O: 'static,
F: Fn(&I) -> Fu + 'static,
Fu: Future<Output = O> + 'static,
Creates an Action to synchronize an imperative async call to the synchronous reactive system.

If you’re trying to load data by running an async function reactively, you probably want to use a create_resource instead. If you’re trying to occasionally run an async function in response to something like a user clicking a button, you’re in the right place.

async fn send_new_todo_to_api(task: String) -> usize {
// do something...
// return a task id
42
}
let save_data = create_action(|task: &String| {
// `task` is given as `&String` because its value is available in `input`
send_new_todo_to_api(task.clone())
});

// the argument currently running
let input = save_data.input();
// the most recent returned result
let result_of_call = save_data.value();
// whether the call is pending
let pending = save_data.pending();
// how many times the action has run
// useful for reactively updating something else in response to a `dispatch` and response
let version = save_data.version();

// before we do anything
assert_eq!(input.get(), None); // no argument yet
assert_eq!(pending.get(), false); // isn't pending a response
assert_eq!(result_of_call.get(), None); // there's no "last value"
assert_eq!(version.get(), 0);
// dispatch the action
save_data.dispatch("My todo".to_string());

// when we're making the call
// assert_eq!(input.get(), Some("My todo".to_string()));
// assert_eq!(pending.get(), true); // is pending
// assert_eq!(result_of_call.get(), None); // has not yet gotten a response

// after call has resolved
assert_eq!(input.get(), None); // input clears out after resolved
assert_eq!(pending.get(), false); // no longer pending
assert_eq!(result_of_call.get(), Some(42));
assert_eq!(version.get(), 1);
The input to the async function should always be a single value, but it can be of any type. The argument is always passed by reference to the function, because it is stored in Action::input as well.

// if there's a single argument, just use that
let action1 = create_action(|input: &String| {
let input = input.clone();
async move { todo!() }
});

// if there are no arguments, use the unit type `()`
let action2 = create_action(|input: &()| async { todo!() });

// if there are multiple arguments, use a tuple
let action3 = create_action(|input: &(usize, String)| async { todo!() });

Function leptos_server::create_multi_actionCopy item path
source · [−]
pub fn create_multi_action<I, O, F, Fu>(action_fn: F) -> MultiAction<I, O>
where
I: 'static,
O: 'static,
F: Fn(&I) -> Fu + 'static,
Fu: Future<Output = O> + 'static,
Creates an MultiAction to synchronize an imperative async call to the synchronous reactive system.

If you’re trying to load data by running an async function reactively, you probably want to use a create_resource instead. If you’re trying to occasionally run an async function in response to something like a user clicking a button, you’re in the right place.

async fn send_new_todo_to_api(task: String) -> usize {
// do something...
// return a task id
42
}
let add_todo = create_multi_action(|task: &String| {
// `task` is given as `&String` because its value is available in `input`
send_new_todo_to_api(task.clone())
});

add_todo.dispatch("Buy milk".to_string());
add_todo.dispatch("???".to_string());
add_todo.dispatch("Profit!!!".to_string());

assert_eq!(add_todo.submissions().get().len(), 3);
The input to the async function should always be a single value, but it can be of any type. The argument is always passed by reference to the function, because it is stored in Submission::input as well.

// if there's a single argument, just use that
let action1 = create_multi_action(|input: &String| {
let input = input.clone();
async move { todo!() }
});

// if there are no arguments, use the unit type `()`
let action2 = create_multi_action(|input: &()| async { todo!() });

// if there are multiple arguments, use a tuple
let action3 =
create_multi_action(|input: &(usize, String)| async { todo!() });

Function leptos_server::create_server_actionCopy item path
source · [−]
pub fn create_server_action<S>(
) -> Action<S, Result<S::Output, ServerFnError<S::Error>>>
where
S: Clone + ServerFn,
S::Error: Clone + 'static,
Creates an Action that can be used to call a server function.


#[server(MyServerFn)]
async fn my_server_fn() -> Result<(), ServerFnError> {
todo!()
}

let my_server_action = create_server_action::<MyServerFn>();

Function leptos_server::create_server_multi_actionCopy item path
source · [−]
pub fn create_server_multi_action<S>(
) -> MultiAction<S, Result<S::Output, ServerFnError<S::Error>>>
where
S: Clone + ServerFn,
Creates an MultiAction that can be used to call a server function.

ⓘ

#[server(MyServerFn)]
async fn my_server_fn() -> Result<(), ServerFnError> {
todo!()
}

let my_server_multi_action = create_server_multi_action::<MyServerFn>();


Working with the Server
The previous section described the process of server-side rendering, using the server to generate an HTML version of the page that will become interactive in the browser. So far, everything has been “isomorphic”; in other words, your app has had the “same (iso) shape (morphe)” on the client and the server.

But a server can do a lot more than just render HTML! In fact, a server can do a whole bunch of things your browser can’t, like reading from and writing to a SQL database.

If you’re used to building JavaScript frontend apps, you’re probably used to calling out to some kind of REST API to do this sort of server work. If you’re used to building sites with PHP or Python or Ruby (or Java or C# or...), this server-side work is your bread and butter, and it’s the client-side interactivity that tends to be an afterthought.

With Leptos, you can do both: not only in the same language, not only sharing the same types, but even in the same files!

This section will talk about how to build the uniquely-server-side parts of your application.


Server Functions
If you’re creating anything beyond a toy app, you’ll need to run code on the server all the time: reading from or writing to a database that only runs on the server, running expensive computations using libraries you don’t want to ship down to the client, accessing APIs that need to be called from the server rather than the client for CORS reasons or because you need a secret API key that’s stored on the server and definitely shouldn’t be shipped down to a user’s browser.

Traditionally, this is done by separating your server and client code, and by setting up something like a REST API or GraphQL API to allow your client to fetch and mutate data on the server. This is fine, but it requires you to write and maintain your code in multiple separate places (client-side code for fetching, server-side functions to run), as well as creating a third thing to manage, which is the API contract between the two.

Leptos is one of a number of modern frameworks that introduce the concept of server functions. Server functions have two key characteristics:

Server functions are co-located with your component code, so that you can organize your work by feature, not by technology. For example, you might have a “dark mode” feature that should persist a user’s dark/light mode preference across sessions, and be applied during server rendering so there’s no flicker. This requires a component that needs to be interactive on the client, and some work to be done on the server (setting a cookie, maybe even storing a user in a database.) Traditionally, this feature might end up being split between two different locations in your code, one in your “frontend” and one in your “backend.” With server functions, you’ll probably just write them both in one dark_mode.rs and forget about it.
Server functions are isomorphic, i.e., they can be called either from the server or the browser. This is done by generating code differently for the two platforms. On the server, a server function simply runs. In the browser, the server function’s body is replaced with a stub that actually makes a fetch request to the server, serializing the arguments into the request and deserializing the return value from the response. But on either end, the function can simply be called: you can create an add_todo function that writes to your database, and simply call it from a click handler on a button in the browser!
Using Server Functions
Actually, I kind of like that example. What would it look like? It’s pretty simple, actually.

// todo.rs

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
let mut conn = db().await?;

    match sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
        .bind(title)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[component]
pub fn BusyButton() -> impl IntoView {
view! {
<button on:click=move |_| {
spawn_local(async {
add_todo("So much to do!".to_string()).await;
});
}>
"Add Todo"
</button>
}
}
You’ll notice a couple things here right away:

Server functions can use server-only dependencies, like sqlx, and can access server-only resources, like our database.
Server functions are async. Even if they only did synchronous work on the server, the function signature would still need to be async, because calling them from the browser must be asynchronous.
Server functions return Result<T, ServerFnError>. Again, even if they only do infallible work on the server, this is true, because ServerFnError’s variants include the various things that can be wrong during the process of making a network request.
Server functions can be called from the client. Take a look at our click handler. This is code that will only ever run on the client. But it can call the function add_todo (using spawn_local to run the Future) as if it were an ordinary async function:
move |_| {
spawn_local(async {
add_todo("So much to do!".to_string()).await;
});
}
Server functions are top-level functions defined with fn. Unlike event listeners, derived signals, and most everything else in Leptos, they are not closures! As fn calls, they have no access to the reactive state of your app or anything else that is not passed in as an argument. And again, this makes perfect sense: When you make a request to the server, the server doesn’t have access to client state unless you send it explicitly. (Otherwise we’d have to serialize the whole reactive system and send it across the wire with every request, which—while it served classic ASP for a while—is a really bad idea.)
Server function arguments and return values both need to be serializable with serde. Again, hopefully this makes sense: while function arguments in general don’t need to be serialized, calling a server function from the browser means serializing the arguments and sending them over HTTP.
There are a few things to note about the way you define a server function, too.

Server functions are created by using the #[server] macro to annotate a top-level function, which can be defined anywhere.
We provide the macro a type name. The type name is used internally as a container to hold, serialize, and deserialize the arguments.
We provide the macro a path. This is a prefix for the path at which we’ll mount a server function handler on our server. (See examples for Actix and Axum.)
You’ll need to have serde as a dependency with the derive featured enabled for the macro to work properly. You can easily add it to Cargo.toml with cargo add serde --features=derive.
Server Function URL Prefixes
You can optionally define a specific URL prefix to be used in the definition of the server function. This is done by providing an optional 2nd argument to the #[server] macro. By default the URL prefix will be /api, if not specified. Here are some examples:

#[server(AddTodo)]         // will use the default URL prefix of `/api`
#[server(AddTodo, "/foo")] // will use the URL prefix of `/foo`
Server Function Encodings
By default, the server function call is a POST request that serializes the arguments as URL-encoded form data in the body of the request. (This means that server functions can be called from HTML forms, which we’ll see in a future chapter.) But there are a few other methods supported. Optionally, we can provide another argument to the #[server] macro to specify an alternate encoding:

#[server(AddTodo, "/api", "Url")]
#[server(AddTodo, "/api", "GetJson")]
#[server(AddTodo, "/api", "Cbor")]
#[server(AddTodo, "/api", "GetCbor")]
The four options use different combinations of HTTP verbs and encoding methods:

Name	Method	Request	Response
Url (default)	POST	URL encoded	JSON
GetJson	GET	URL encoded	JSON
Cbor	POST	CBOR	CBOR
GetCbor	GET	URL encoded	CBOR
In other words, you have two choices:

GET or POST? This has implications for things like browser or CDN caching; while POST requests should not be cached, GET requests can be.
Plain text (arguments sent with URL/form encoding, results sent as JSON) or a binary format (CBOR, encoded as a base64 string)?
But remember: Leptos will handle all the details of this encoding and decoding for you. When you use a server function, it looks just like calling any other asynchronous function!

Why not PUT or DELETE? Why URL/form encoding, and not JSON?

These are reasonable questions. Much of the web is built on REST API patterns that encourage the use of semantic HTTP methods like DELETE to delete an item from a database, and many devs are accustomed to sending data to APIs in the JSON format.

The reason we use POST or GET with URL-encoded data by default is the <form> support. For better or for worse, HTML forms don’t support PUT or DELETE, and they don’t support sending JSON. This means that if you use anything but a GET or POST request with URL-encoded data, it can only work once WASM has loaded. As we’ll see in a later chapter, this isn’t always a great idea.

The CBOR encoding is supported for historical reasons; an earlier version of server functions used a URL encoding that didn’t support nested objects like structs or vectors as server function arguments, which CBOR did. But note that the CBOR forms encounter the same issue as PUT, DELETE, or JSON: they do not degrade gracefully if the WASM version of your app is not available.

Server Functions Endpoint Paths
By default, a unique path will be generated. You can optionally define a specific endpoint path to be used in the URL. This is done by providing an optional 4th argument to the #[server] macro. Leptos will generate the complete path by concatenating the URL prefix (2nd argument) and the endpoint path (4th argument). For example,

#[server(MyServerFnType, "/api", "Url", "hello")]
will generate a server function endpoint at /api/hello that accepts a POST request.

Can I use the same server function endpoint path with multiple encodings?

No. Different server functions must have unique paths. The #[server] macro automatically generates unique paths, but you need to be careful if you choose to specify the complete path manually, as the server looks up server functions by their path.

An Important Note on Security
Server functions are a cool technology, but it’s very important to remember. Server functions are not magic; they’re syntax sugar for defining a public API. The body of a server function is never made public; it’s just part of your server binary. But the server function is a publicly accessible API endpoint, and its return value is just a JSON or similar blob. Do not return information from a server function unless it is public, or you've implemented proper security procedures. These procedures might include authenticating incoming requests, ensuring proper encryption, rate limiting access, and more.

Integrating Server Functions with Leptos
So far, everything I’ve said is actually framework agnostic. (And in fact, the Leptos server function crate has been integrated into Dioxus as well!) Server functions are simply a way of defining a function-like RPC call that leans on Web standards like HTTP requests and URL encoding.

But in a way, they also provide the last missing primitive in our story so far. Because a server function is just a plain Rust async function, it integrates perfectly with the async Leptos primitives we discussed earlier. So you can easily integrate your server functions with the rest of your applications:

Create resources that call the server function to load data from the server
Read these resources under <Suspense/> or <Transition/> to enable streaming SSR and fallback states while data loads.
Create actions that call the server function to mutate data on the server
The final section of this book will make this a little more concrete by introducing patterns that use progressively-enhanced HTML forms to run these server actions.

But in the next few chapters, we’ll actually take a look at some of the details of what you might want to do with your server functions, including the best ways to integrate with the powerful extractors provided by the Actix and Axum server frameworks.


Extractors
The server functions we looked at in the last chapter showed how to run code on the server, and integrate it with the user interface you’re rendering in the browser. But they didn’t show you much about how to actually use your server to its full potential.

Server Frameworks
We call Leptos a “full-stack” framework, but “full-stack” is always a misnomer (after all, it never means everything from the browser to your power company.) For us, “full stack” means that your Leptos app can run in the browser, and can run on the server, and can integrate the two, drawing together the unique features available in each; as we’ve seen in the book so far, a button click on the browser can drive a database read on the server, both written in the same Rust module. But Leptos itself doesn’t provide the server (or the database, or the operating system, or the firmware, or the electrical cables...)

Instead, Leptos provides integrations for the two most popular Rust web server frameworks, Actix Web (leptos_actix) and Axum (leptos_axum). We’ve built integrations with each server’s router so that you can simply plug your Leptos app into an existing server with .leptos_routes(), and easily handle server function calls.

If you haven’t seen our Actix and Axum templates, now’s a good time to check them out.

Using Extractors
Both Actix and Axum handlers are built on the same powerful idea of extractors. Extractors “extract” typed data from an HTTP request, allowing you to access server-specific data easily.

Leptos provides extract helper functions to let you use these extractors directly in your server functions, with a convenient syntax very similar to handlers for each framework.

Actix Extractors
The extract function in leptos_actix takes a handler function as its argument. The handler follows similar rules to an Actix handler: it is an async function that receives arguments that will be extracted from the request and returns some value. The handler function receives that extracted data as its arguments, and can do further async work on them inside the body of the async move block. It returns whatever value you return back out into the server function.

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MyQuery {
foo: String,
}

#[server]
pub async fn actix_extract() -> Result<String, ServerFnError> {
use actix_web::dev::ConnectionInfo;
use actix_web::web::{Data, Query};
use leptos_actix::extract;

    let (Query(search), connection): (Query<MyQuery>, ConnectionInfo) = extract().await?;
    Ok(format!("search = {search:?}\nconnection = {connection:?}",))
}
Axum Extractors
The syntax for the leptos_axum::extract function is very similar.

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MyQuery {
foo: String,
}

#[server]
pub async fn axum_extract() -> Result<String, ServerFnError> {
use axum::{extract::Query, http::Method};
use leptos_axum::extract;

    let (method, query): (Method, Query<MyQuery>) = extract().await?;

    Ok(format!("{method:?} and {query:?}"))
}
These are relatively simple examples accessing basic data from the server. But you can use extractors to access things like headers, cookies, database connection pools, and more, using the exact same extract() pattern.

The Axum extract function only supports extractors for which the state is (). If you need an extractor that uses State, you should use extract_with_state. This requires you to provide the state. You can do this by extending the existing LeptosOptions state using the Axum FromRef pattern, which providing the state as context during render and server functions with custom handlers.

use axum::extract::FromRef;

/// Derive FromRef to allow multiple items in state, using Axum’s
/// SubStates pattern.
#[derive(FromRef, Debug, Clone)]
pub struct AppState{
pub leptos_options: LeptosOptions,
pub pool: SqlitePool
}
Click here for an example of providing context in custom handlers.

Axum State
Axum's typical pattern for dependency injection is to provide a State, which can then be extracted in your route handler. Leptos provides its own method of dependency injection via context. Context can often be used instead of State to provide shared server data (for example, a database connection pool).

let connection_pool = /* some shared state here */;

let app = Router::new()
.leptos_routes_with_context(
&app_state,
routes,
move || provide_context(connection_pool.clone()),
App,
)
// etc.
This context can then be accessed with a simple use_context::<T>() inside your server functions.

If you need to use State in a server function—for example, if you have an existing Axum extractor that requires State, that is also possible using Axum's FromRef pattern and extract_with_state. Essentially you'll need to provide the state both via context and via Axum router state:

#[derive(FromRef, Debug, Clone)]
pub struct MyData {
pub value: usize,
pub leptos_options: LeptosOptions,
}

let app_state = MyData {
value: 42,
leptos_options,
};

// build our application with a route
let app = Router::new()
.leptos_routes_with_context(
&app_state,
routes,
{
let app_state = app_state.clone();
move || provide_context(app_state.clone());
},
App,
)
.fallback(file_and_error_handler)
.with_state(app_state);

// ...
#[server]
pub async fn uses_state() -> Result<(), ServerFnError> {
let state = expect_context::<AppState>();
let SomeStateExtractor(data) = extract_with_state(&state).await?;
// todo
}
A Note about Data-Loading Patterns
Because Actix and (especially) Axum are built on the idea of a single round-trip HTTP request and response, you typically run extractors near the “top” of your application (i.e., before you start rendering) and use the extracted data to determine how that should be rendered. Before you render a <button>, you load all the data your app could need. And any given route handler needs to know all the data that will need to be extracted by that route.

But Leptos integrates both the client and the server, and it’s important to be able to refresh small pieces of your UI with new data from the server without forcing a full reload of all the data. So Leptos likes to push data loading “down” in your application, as far towards the leaves of your user interface as possible. When you click a <button>, it can refresh just the data it needs. This is exactly what server functions are for: they give you granular access to data to be loaded and reloaded.

The extract() functions let you combine both models by using extractors in your server functions. You get access to the full power of route extractors, while decentralizing knowledge of what needs to be extracted down to your individual components. This makes it easier to refactor and reorganize routes: you don’t need to specify all the data a route needs up front.


Responses and Redirects
Extractors provide an easy way to access request data inside server functions. Leptos also provides a way to modify the HTTP response, using the ResponseOptions type (see docs for Actix or Axum) types and the redirect helper function (see docs for Actix or Axum).

ResponseOptions
ResponseOptions is provided via context during the initial server rendering response and during any subsequent server function call. It allows you to easily set the status code for the HTTP response, or to add headers to the HTTP response, e.g., to set cookies.

#[server(TeaAndCookies)]
pub async fn tea_and_cookies() -> Result<(), ServerFnError> {
use actix_web::{cookie::Cookie, http::header, http::header::HeaderValue};
use leptos_actix::ResponseOptions;

    // pull ResponseOptions from context
    let response = expect_context::<ResponseOptions>();

    // set the HTTP status code
    response.set_status(StatusCode::IM_A_TEAPOT);

    // set a cookie in the HTTP response
    let mut cookie = Cookie::build("biscuits", "yes").finish();
    if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
        response.insert_header(header::SET_COOKIE, cookie);
    }
}
redirect
One common modification to an HTTP response is to redirect to another page. The Actix and Axum integrations provide a redirect function to make this easy to do. redirect simply sets an HTTP status code of 302 Found and sets the Location header.

Here’s a simplified example from our session_auth_axum example.

#[server(Login, "/api")]
pub async fn login(
username: String,
password: String,
remember: Option<String>,
) -> Result<(), ServerFnError> {
// pull the DB pool and auth provider from context
let pool = pool()?;
let auth = auth()?;

    // check whether the user exists
    let user: User = User::get_from_username(username, &pool)
        .await
        .ok_or_else(|| {
            ServerFnError::ServerError("User does not exist.".into())
        })?;

    // check whether the user has provided the correct password
    match verify(password, &user.password)? {
        // if the password is correct...
        true => {
            // log the user in
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());

            // and redirect to the home page
            leptos_axum::redirect("/");
            Ok(())
        }
        // if not, return an error
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}
This server function can then be used from your application. This redirect works well with the progressively-enhanced <ActionForm/> component: without JS/WASM, the server response will redirect because of the status code and header. With JS/WASM, the <ActionForm/> will detect the redirect in the server function response, and use client-side navigation to redirect to the new page.


Progressive Enhancement (and Graceful Degradation)
I’ve been driving around Boston for about fifteen years. If you don’t know Boston, let me tell you: Massachusetts has some of the most aggressive drivers(and pedestrians!) in the world. I’ve learned to practice what’s sometimes called “defensive driving”: assuming that someone’s about to swerve in front of you at an intersection when you have the right of way, preparing for a pedestrian to cross into the street at any moment, and driving accordingly.

“Progressive enhancement” is the “defensive driving” of web design. Or really, that’s “graceful degradation,” although they’re two sides of the same coin, or the same process, from two different directions.

Progressive enhancement, in this context, means beginning with a simple HTML site or application that works for any user who arrives at your page, and gradually enhancing it with layers of additional features: CSS for styling, JavaScript for interactivity, WebAssembly for Rust-powered interactivity; using particular Web APIs for a richer experience if they’re available and as needed.

Graceful degradation means handling failure gracefully when parts of that stack of enhancement aren’t available. Here are some sources of failure your users might encounter in your app:

Their browser doesn’t support WebAssembly because it needs to be updated.
Their browser can’t support WebAssembly because browser updates are limited to newer OS versions, which can’t be installed on the device. (Looking at you, Apple.)
They have WASM turned off for security or privacy reasons.
They have JavaScript turned off for security or privacy reasons.
JavaScript isn’t supported on their device (for example, some accessibility devices only support HTML browsing)
The JavaScript (or WASM) never arrived at their device because they walked outside and lost WiFi.
They stepped onto a subway car after loading the initial page and subsequent navigations can’t load data.
... and so on.
How much of your app still works if one of these holds true? Two of them? Three?

If the answer is something like “95%... okay, then 90%... okay, then 75%,” that’s graceful degradation. If the answer is “my app shows a blank screen unless everything works correctly,” that’s... rapid unscheduled disassembly.

Graceful degradation is especially important for WASM apps, because WASM is the newest and least-likely-to-be-supported of the four languages that run in the browser (HTML, CSS, JS, WASM).

Luckily, we’ve got some tools to help.

Defensive Design
There are a few practices that can help your apps degrade more gracefully:

Server-side rendering. Without SSR, your app simply doesn’t work without both JS and WASM loading. In some cases this may be appropriate (think internal apps gated behind a login) but in others it’s simply broken.
Native HTML elements. Use HTML elements that do the things that you want, without additional code: <a> for navigation (including to hashes within the page), <details> for an accordion, <form> to persist information in the URL, etc.
URL-driven state. The more of your global state is stored in the URL (as a route param or part of the query string), the more of the page can be generated during server rendering and updated by an <a> or a <form>, which means that not only navigations but state changes can work without JS/WASM.
SsrMode::PartiallyBlocked or SsrMode::InOrder. Out-of-order streaming requires a small amount of inline JS, but can fail if 1) the connection is broken halfway through the response or 2) the client’s device doesn’t support JS. Async streaming will give a complete HTML page, but only after all resources load. In-order streaming begins showing pieces of the page sooner, in top-down order. “Partially-blocked” SSR builds on out-of-order streaming by replacing <Suspense/> fragments that read from blocking resources on the server. This adds marginally to the initial response time (because of the O(n) string replacement work), in exchange for a more complete initial HTML response. This can be a good choice for situations in which there’s a clear distinction between “more important” and “less important” content, e.g., blog post vs. comments, or product info vs. reviews. If you choose to block on all the content, you’ve essentially recreated async rendering.
Leaning on <form>s. There’s been a bit of a <form> renaissance recently, and it’s no surprise. The ability of a <form> to manage complicated POST or GET requests in an easily-enhanced way makes it a powerful tool for graceful degradation. The example in the <Form/> chapter, for example, would work fine with no JS/WASM: because it uses a <form method="GET"> to persist state in the URL, it works with pure HTML by making normal HTTP requests and then progressively enhances to use client-side navigations instead.
There’s one final feature of the framework that we haven’t seen yet, and which builds on this characteristic of forms to build powerful applications: the <ActionForm/>.


<ActionForm/>
<ActionForm/> is a specialized <Form/> that takes a server action, and automatically dispatches it on form submission. This allows you to call a server function directly from a <form>, even without JS/WASM.

The process is simple:

Define a server function using the #[server] macro (see Server Functions.)
Create an action using create_server_action, specifying the type of the server function you’ve defined.
Create an <ActionForm/>, providing the server action in the action prop.
Pass the named arguments to the server function as form fields with the same names.
Note: <ActionForm/> only works with the default URL-encoded POST encoding for server functions, to ensure graceful degradation/correct behavior as an HTML form.

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
todo!()
}

#[component]
fn AddTodo() -> impl IntoView {
let add_todo = create_server_action::<AddTodo>();
// holds the latest *returned* value from the server
let value = add_todo.value();
// check if the server has returned an error
let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <ActionForm action=add_todo>
            <label>
                "Add a Todo"
                // `title` matches the `title` argument to `add_todo`
                <input type="text" name="title"/>
            </label>
            <input type="submit" value="Add"/>
        </ActionForm>
    }
}
It’s really that easy. With JS/WASM, your form will submit without a page reload, storing its most recent submission in the .input() signal of the action, its pending status in .pending(), and so on. (See the Action docs for a refresher, if you need.) Without JS/WASM, your form will submit with a page reload. If you call a redirect function (from leptos_axum or leptos_actix) it will redirect to the correct page. By default, it will redirect back to the page you’re currently on. The power of HTML, HTTP, and isomorphic rendering mean that your <ActionForm/> simply works, even with no JS/WASM.

Client-Side Validation
Because the <ActionForm/> is just a <form>, it fires a submit event. You can use either HTML validation, or your own client-side validation logic in an on:submit. Just call ev.prevent_default() to prevent submission.

The FromFormData trait can be helpful here, for attempting to parse your server function’s data type from the submitted form.

let on_submit = move |ev| {
let data = AddTodo::from_event(&ev);
// silly example of validation: if the todo is "nope!", nope it
if data.is_err() || data.unwrap().title == "nope!" {
// ev.prevent_default() will prevent form submission
ev.prevent_default();
}
}
Complex Inputs
Server function arguments that are structs with nested serializable fields should make use of indexing notation of serde_qs.

use leptos::*;
use leptos_router::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct HeftyData {
first_name: String,
last_name: String,
}

#[component]
fn ComplexInput() -> impl IntoView {
let submit = Action::<VeryImportantFn, _>::server();

    view! {
      <ActionForm action=submit>
        <input type="text" name="hefty_arg[first_name]" value="leptos"/>
        <input
          type="text"
          name="hefty_arg[last_name]"
          value="closures-everywhere"
        />
        <input type="submit"/>
      </ActionForm>
    }
}

#[server]
async fn very_important_fn(
hefty_arg: HeftyData,
) -> Result<(), ServerFnError> {
assert_eq!(hefty_arg.first_name.as_str(), "leptos");
assert_eq!(hefty_arg.last_name.as_str(), "closures-everywhere");
Ok(())
}


Deployment
There are as many ways to deploy a web application as there are developers, let alone applications. But there are a couple useful tips to keep in mind when deploying an app.

General Advice
Remember: Always deploy Rust apps built in --release mode, not debug mode. This has a huge effect on both performance and binary size.
Test locally in release mode as well. The framework applies certain optimizations in release mode that it does not apply in debug mode, so it’s possible for bugs to surface at this point. (If your app behaves differently or you do encounter a bug, it’s likely a framework-level bug and you should open a GitHub issue with a reproduction.)
See the chapter on "Optimizing WASM Binary Size" for additional tips and tricks to further improve the time-to-interactive metric for your WASM app on first load.
We asked users to submit their deployment setups to help with this chapter.


Deploying a Client-Side-Rendered App
If you’ve been building an app that only uses client-side rendering, working with Trunk as a dev server and build tool, the process is quite easy.

trunk build --release
trunk build will create a number of build artifacts in a dist/ directory. Publishing dist somewhere online should be all you need to deploy your app. This should work very similarly to deploying any JavaScript application.

We've created several example repositories which show how to set up and deploy a Leptos CSR app to various hosting services.

Note: Leptos does not endorse the use of any particular hosting service - feel free to use any service that supports static site deploys.

Examples:

Github Pages
Vercel
Spin (serverless WebAssembly)
Github Pages
Deploying a Leptos CSR app to Github pages is a simple affair. First, go to your Github repo's settings and click on "Pages" in the left side menu. In the "Build and deployment" section of the page, change the "source" to "Github Actions". Then copy the following into a file such as .github/workflows/gh-pages-deploy.yml

Example

name: Release to Github Pages

on:
push:
branches: [main]
workflow_dispatch:

permissions:
contents: write # for committing to gh-pages branch.
pages: write
id-token: write

# Allow only one concurrent deployment, skipping runs queued between the run in-progress and latest queued.
# However, do NOT cancel in-progress runs as we want to allow these production deployments to complete.
concurrency:
group: "pages"
cancel-in-progress: false

jobs:
Github-Pages-Release:

    timeout-minutes: 10

    environment:
    name: github-pages
    url: ${{ steps.deployment.outputs.page_url }}

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4 # repo checkout

    # Install Rust Nightly Toolchain, with Clippy & Rustfmt
    - name: Install nightly Rust
        uses: dtolnay/rust-toolchain@nightly
        with:
        components: clippy, rustfmt

    - name: Add WASM target
        run: rustup target add wasm32-unknown-unknown

    - name: lint
        run: cargo clippy & cargo fmt


    # If using tailwind...
    # - name: Download and install tailwindcss binary
    #   run: npm install -D tailwindcss && npx tailwindcss -i <INPUT/PATH.css> -o <OUTPUT/PATH.css>  # run tailwind


    - name: Download and install Trunk binary
        run: wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.18.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

    - name: Build with Trunk
        # "${GITHUB_REPOSITORY#*/}" evaluates into the name of the repository
        # using --public-url something will allow trunk to modify all the href paths like from favicon.ico to repo_name/favicon.ico .
        # this is necessary for github pages where the site is deployed to username.github.io/repo_name and all files must be requested
        # relatively as favicon.ico. if we skip public-url option, the href paths will instead request username.github.io/favicon.ico which
        # will obviously return error 404 not found.
        run: ./trunk build --release --public-url "${GITHUB_REPOSITORY#*/}"


    # Deploy to gh-pages branch
    # - name: Deploy 🚀
    #   uses: JamesIves/github-pages-deploy-action@v4
    #   with:
    #     folder: dist


    # Deploy with Github Static Pages

    - name: Setup Pages
        uses: actions/configure-pages@v4
        with:
        enablement: true
        # token:

    - name: Upload artifact
        uses: actions/upload-pages-artifact@v2
        with:
        # Upload dist dir
        path: './dist'

    - name: Deploy to GitHub Pages 🚀
        id: deployment
        uses: actions/deploy-pages@v3
For more on deploying to Github Pages see the example repo here

Vercel
Step 1: Set Up Vercel
In the Vercel Web UI...

Create a new project
Ensure
The "Build Command" is left empty with Override on
The "Output Directory" is changed to dist (which is the default output directory for Trunk builds) and the Override is on

Step 2: Add Vercel Credentials for GitHub Actions
Note: Both the preview and deploy actions will need your Vercel credentials setup in GitHub secrets

Retrieve your Vercel Access Token by going to "Account Settings" > "Tokens" and creating a new token - save the token to use in sub-step 5, below.

Install the Vercel CLI using the npm i -g vercel command, then run vercel login to login to your acccount.

Inside your folder, run vercel link to create a new Vercel project; in the CLI, you will be asked to 'Link to an existing project?' - answer yes, then enter the name you created in step 1. A new .vercel folder will be created for you.

Inside the generated .vercel folder, open the the project.json file and save the "projectId" and "orgId" for the next step.

Inside GitHub, go the repo's "Settings" > "Secrets and Variables" > "Actions" and add the following as Repository secrets:

save your Vercel Access Token (from sub-step 1) as the VERCEL_TOKEN secret
from the .vercel/project.json add "projectID" as VERCEL_PROJECT_ID
from the .vercel/project.json add "orgId" as VERCEL_ORG_ID
For full instructions see "How can I use Github Actions with Vercel"

Step 3: Add Github Action Scripts
Finally, you're ready to simply copy and paste the two files - one for deployment, one for PR previews - from below or from the example repo's .github/workflows/ folder into your own github workflows folder - then, on your next commit or PR deploys will occur automatically.

Production deployment script: vercel_deploy.yml

Example

name: Release to Vercel

on:
push:
branches:
- main
env:
CARGO_TERM_COLOR: always
VERCEL_ORG_ID: ${{ secrets.VERCEL_ORG_ID }}
VERCEL_PROJECT_ID: ${{ secrets.VERCEL_PROJECT_ID }}

jobs:
Vercel-Production-Deployment:
runs-on: ubuntu-latest
environment: production
steps:
- name: git-checkout
uses: actions/checkout@v3

    - uses: dtolnay/rust-toolchain@nightly
        with:
        components: clippy, rustfmt
    - uses: Swatinem/rust-cache@v2
    - name: Setup Rust
        run: |
        rustup target add wasm32-unknown-unknown
        cargo clippy
        cargo fmt --check

    - name: Download and install Trunk binary
        run: wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.18.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-


    - name: Build with Trunk
        run: ./trunk build --release

    - name: Install Vercel CLI
        run: npm install --global vercel@latest

    - name: Pull Vercel Environment Information
        run: vercel pull --yes --environment=production --token=${{ secrets.VERCEL_TOKEN }}

    - name: Deploy to Vercel & Display URL
        id: deployment
        working-directory: ./dist
        run: |
        vercel deploy --prod --token=${{ secrets.VERCEL_TOKEN }} >> $GITHUB_STEP_SUMMARY
        echo $GITHUB_STEP_SUMMARY
Preview deployments script: vercel_preview.yml

Example

# For more info re: vercel action see:
# https://github.com/amondnet/vercel-action

name: Leptos CSR Vercel Preview

on:
pull_request:
branches: [ "main" ]

workflow_dispatch:

env:
CARGO_TERM_COLOR: always
VERCEL_ORG_ID: ${{ secrets.VERCEL_ORG_ID }}
VERCEL_PROJECT_ID: ${{ secrets.VERCEL_PROJECT_ID }}

jobs:
fmt:
name: Rustfmt
runs-on: ubuntu-latest
steps:
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@nightly
with:
components: rustfmt
- name: Enforce formatting
run: cargo fmt --check

clippy:
name: Clippy
runs-on: ubuntu-latest
steps:
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@nightly
with:
components: clippy
- uses: Swatinem/rust-cache@v2
- name: Linting
run: cargo clippy -- -D warnings

test:
name: Test
runs-on: ubuntu-latest
needs: [fmt, clippy]
steps:
- uses: actions/checkout@v4
- uses: dtolnay/rust-toolchain@nightly
- uses: Swatinem/rust-cache@v2
- name: Run tests
run: cargo test

build-and-preview-deploy:
runs-on: ubuntu-latest
name: Build and Preview

    needs: [test, clippy, fmt]

    permissions:
    pull-requests: write

    environment:
    name: preview
    url: ${{ steps.preview.outputs.preview-url }}

    steps:
    - name: git-checkout
        uses: actions/checkout@v4

    - uses: dtolnay/rust-toolchain@nightly
    - uses: Swatinem/rust-cache@v2
    - name: Build
        run: rustup target add wasm32-unknown-unknown

    - name: Download and install Trunk binary
        run: wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.18.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-


    - name: Build with Trunk
        run: ./trunk build --release

    - name: Preview Deploy
        id: preview
        uses: amondnet/vercel-action@v25.1.1
        with:
        vercel-token: ${{ secrets.VERCEL_TOKEN }}
        github-token: ${{ secrets.GITHUB_TOKEN }}
        vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
        vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}
        github-comment: true
        working-directory: ./dist

    - name: Display Deployed URL
        run: |
        echo "Deployed app URL: ${{ steps.preview.outputs.preview-url }}" >> $GITHUB_STEP_SUMMARY
See the example repo here for more.

Spin - Serverless WebAssembly
Another option is using a serverless platform such as Spin. Although Spin is open source and you can run it on your own infrastructure (eg. inside Kubernetes), the easiest way to get started with Spin in production is to use the Fermyon Cloud.

Start by installing the Spin CLI using the instructions, here, and creating a Github repo for your Leptos CSR project, if you haven't done so already.

Open "Fermyon Cloud" > "User Settings". If you’re not logged in, choose the Login With GitHub button.

In the “Personal Access Tokens”, choose “Add a Token”. Enter the name “gh_actions” and click “Create Token”.

Fermyon Cloud displays the token; click the copy button to copy it to your clipboard.

Go into your Github repo and open "Settings" > "Secrets and Variables" > "Actions" and add the Fermyon cloud token to "Repository secrets" using the variable name "FERMYON_CLOUD_TOKEN"

Copy and paste the following Github Actions scripts (below) into your .github/workflows/<SCRIPT_NAME>.yml files

With the 'preview' and 'deploy' scripts active, Github Actions will now generate previews on pull requests & deploy automatically on updates to your 'main' branch.

Production deployment script: spin_deploy.yml

Example

# For setup instructions needed for Fermyon Cloud, see:
# https://developer.fermyon.com/cloud/github-actions

# For reference, see:
# https://developer.fermyon.com/cloud/changelog/gh-actions-spin-deploy

# For the Fermyon gh actions themselves, see:
# https://github.com/fermyon/actions

name: Release to Spin Cloud

on:
push:
branches: [main]
workflow_dispatch:

permissions:
contents: read
id-token: write

# Allow only one concurrent deployment, skipping runs queued between the run in-progress and latest queued.
# However, do NOT cancel in-progress runs as we want to allow these production deployments to complete.
concurrency:
group: "spin"
cancel-in-progress: false

jobs:
Spin-Release:

    timeout-minutes: 10

    environment:
    name: production
    url: ${{ steps.deployment.outputs.app-url }}

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4 # repo checkout

    # Install Rust Nightly Toolchain, with Clippy & Rustfmt
    - name: Install nightly Rust
        uses: dtolnay/rust-toolchain@nightly
        with:
        components: clippy, rustfmt

    - name: Add WASM & WASI targets
        run: rustup target add wasm32-unknown-unknown && rustup target add wasm32-wasi

    - name: lint
        run: cargo clippy & cargo fmt


    # If using tailwind...
    # - name: Download and install tailwindcss binary
    #   run: npm install -D tailwindcss && npx tailwindcss -i <INPUT/PATH.css> -o <OUTPUT/PATH.css>  # run tailwind


    - name: Download and install Trunk binary
        run: wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.18.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-


    - name: Build with Trunk
        run: ./trunk build --release


    # Install Spin CLI & Deploy

    - name: Setup Spin
        uses: fermyon/actions/spin/setup@v1
        # with:
        # plugins:


    - name: Build and deploy
        id: deployment
        uses: fermyon/actions/spin/deploy@v1
        with:
        fermyon_token: ${{ secrets.FERMYON_CLOUD_TOKEN }}
        # key_values: |-
            # abc=xyz
            # foo=bar
        # variables: |-
            # password=${{ secrets.SECURE_PASSWORD }}
            # apikey=${{ secrets.API_KEY }}

    # Create an explicit message to display the URL of the deployed app, as well as in the job graph
    - name: Deployed URL
        run: |
        echo "Deployed app URL: ${{ steps.deployment.outputs.app-url }}" >> $GITHUB_STEP_SUMMARY
Preview deployment script: spin_preview.yml

Example

# For setup instructions needed for Fermyon Cloud, see:
# https://developer.fermyon.com/cloud/github-actions


# For the Fermyon gh actions themselves, see:
# https://github.com/fermyon/actions

# Specifically:
# https://github.com/fermyon/actions?tab=readme-ov-file#deploy-preview-of-spin-app-to-fermyon-cloud---fermyonactionsspinpreviewv1

name: Preview on Spin Cloud

on:
pull_request:
branches: ["main", "v*"]
types: ['opened', 'synchronize', 'reopened', 'closed']
workflow_dispatch:

permissions:
contents: read
pull-requests: write

# Allow only one concurrent deployment, skipping runs queued between the run in-progress and latest queued.
# However, do NOT cancel in-progress runs as we want to allow these production deployments to complete.
concurrency:
group: "spin"
cancel-in-progress: false

jobs:
Spin-Preview:

    timeout-minutes: 10

    environment:
    name: preview
    url: ${{ steps.preview.outputs.app-url }}

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4 # repo checkout

    # Install Rust Nightly Toolchain, with Clippy & Rustfmt
    - name: Install nightly Rust
        uses: dtolnay/rust-toolchain@nightly
        with:
        components: clippy, rustfmt

    - name: Add WASM & WASI targets
        run: rustup target add wasm32-unknown-unknown && rustup target add wasm32-wasi

    - name: lint
        run: cargo clippy & cargo fmt


    # If using tailwind...
    # - name: Download and install tailwindcss binary
    #   run: npm install -D tailwindcss && npx tailwindcss -i <INPUT/PATH.css> -o <OUTPUT/PATH.css>  # run tailwind


    - name: Download and install Trunk binary
        run: wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.18.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-


    - name: Build with Trunk
        run: ./trunk build --release


    # Install Spin CLI & Deploy

    - name: Setup Spin
        uses: fermyon/actions/spin/setup@v1
        # with:
        # plugins:


    - name: Build and preview
        id: preview
        uses: fermyon/actions/spin/preview@v1
        with:
        fermyon_token: ${{ secrets.FERMYON_CLOUD_TOKEN }}
        github_token: ${{ secrets.GITHUB_TOKEN }}
        undeploy: ${{ github.event.pull_request && github.event.action == 'closed' }}
        # key_values: |-
            # abc=xyz
            # foo=bar
        # variables: |-
            # password=${{ secrets.SECURE_PASSWORD }}
            # apikey=${{ secrets.API_KEY }}


    - name: Display Deployed URL
        run: |
        echo "Deployed app URL: ${{ steps.preview.outputs.app-url }}" >> $GITHUB_STEP_SUMMARY


Deploying a Full-Stack SSR App
It's possible to deploy Leptos fullstack, SSR apps to any number of server or container hosting services. The most simple way to get a Leptos SSR app into production might be to use a VPS service and either run Leptos natively in a VM (see here for more details). Alternatively, you could containerize your Leptos app and run it in Podman or Docker on any colocated or cloud server.

There are a multitude of different deployment setups and hosting services, and in general, Leptos itself is agnostic to the deployment setup you use. With this diversity of deployment targets in mind, on this page we will go over:

creating a Containerfile (or Dockerfile) for use with Leptos SSR apps
Using a Dockerfile to deploy to a cloud service - for example, Fly.io
Deploying Leptos to serverless runtimes - for example, AWS Lambda and JS-hosted WASM runtimes like Deno & Cloudflare
Platforms that have not yet gained Leptos SSR support
Note: Leptos does not endorse the use of any particular method of deployment or hosting service.

Creating a Containerfile
The most popular way for people to deploy full-stack apps built with cargo-leptos is to use a cloud hosting service that supports deployment via a Podman or Docker build. Here’s a sample Containerfile / Dockerfile, which is based on the one we use to deploy the Leptos website.

Debian
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# If you’re using stable, use this instead
# FROM rust:1.74-bullseye as builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Build the app
RUN cargo leptos build --release -vv

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
&& apt-get install -y --no-install-recommends openssl ca-certificates \
&& apt-get autoremove -y \
&& apt-get clean -y \
&& rm -rf /var/lib/apt/lists/*

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Copy the server binary to the /app directory
COPY --from=builder /app/target/release/leptos_start /app/

# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# -- NB: update binary name from "leptos_start" to match your app name in Cargo.toml --
# Run the server
CMD ["/app/leptos_start"]
Alpine
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder

RUN apk update && \
apk add --no-cache bash curl npm libc-dev binaryen

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

COPY --from=builder /work/target/release/leptos_start /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

EXPOSE $PORT
ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/leptos_start"]
Read more: gnu and musl build files for Leptos apps.

Cloud Deployments
Deploy to Fly.io
One option for deploying your Leptos SSR app is to use a service like Fly.io, which takes a Dockerfile definition of your Leptos app and runs it in a quick-starting micro-VM; Fly also offers a variety of storage options and managed DBs to use with your projects. The following example will show how to deploy a simple Leptos starter app, just to get you up and going; see here for more about working with storage options on Fly.io if and when required.

First, create a Dockerfile in the root of your application and fill it in with the suggested contents (above); make sure to update the binary names in the Dockerfile example to the name of your own application, and make other adjustments as necessary.

Also, ensure you have the flyctl CLI tool installed, and have an account set up at Fly.io. To install flyctl on MacOS, Linux, or Windows WSL, run:

curl -L https://fly.io/install.sh | sh
If you have issues, or for installing to other platforms see the full instructions here

Then login to Fly.io

fly auth login
and manually launch your app using the command

fly launch
The flyctl CLI tool will walk you through the process of deploying your app to Fly.io.

Note

By default, Fly.io will auto-stop machines that don't have traffic coming to them after a certain period of time. Although Fly.io's lightweight VM's start up quickly, if you want to minimize the latency of your Leptos app and ensure it's always swift to respond, go into the generated fly.toml file and change the min_machines_running to 1 from the default of 0.

See this page in the Fly.io docs for more details.

If you would prefer to use Github Actions to manage your deployments, you will need to create a new access token via the Fly.io web UI.

Go to "Account" > "Access Tokens" and create a token named something like "github_actions", then add the token to your Github repo's secrets by going into your project's Github repo, then clicking "Settings" > "Secrets and Variables" > "Actions" and creating a "New repository secret" with the name "FLY_API_TOKEN".

To generate a fly.toml config file for deployment to Fly.io, you must first run the following from within the project source directory

fly launch --no-deploy
to create a new Fly app and register it with the service. Git commit your new fly.toml file.

To set up the Github Actions deployment workflow, copy the following into a .github/workflows/fly_deploy.yml file:

Example

# For more details, see: https://fly.io/docs/app-guides/continuous-deployment-with-github-actions/

name: Deploy to Fly.io
on:
push:
branches:
- main
jobs:
deploy:
name: Deploy app
runs-on: ubuntu-latest
steps:
- uses: actions/checkout@v4
- uses: superfly/flyctl-actions/setup-flyctl@master
- name: Deploy to fly
id: deployment
run: |
flyctl deploy --remote-only | tail -n 1 >> $GITHUB_STEP_SUMMARY
env:
FLY_API_TOKEN: ${{ secrets.FLY_API_TOKEN }}
On the next commit to your Github main branch, your project will automatically deploy to Fly.io.

See the example repo here.

Railway
Another provider for cloud deployments is Railway. Railway integrates with GitHub to automatically deploy your code.

There is an opinionated community template that gets you started quickly:

Deploy on Railway

The template has renovate setup to keep dependencies up to date and supports GitHub Actions to test your code before a deploy happens.

Railway has a free tier that does not require a credit card, and with how little resources Leptos needs that free tier should last a long time.

See the example repo here.

Deploy to Serverless Runtimes
Leptos supports deploying to FaaS (Function as a Service) or 'serverless' runtimes such as AWS Lambda as well as WinterCG-compatible JS runtimes such as Deno and Cloudflare. Just be aware that serverless environments do place some restrictions on the functionality available to your SSR app when compared with VM or container type deployments (see notes, below).

AWS Lambda
With a little help from the Cargo Lambda tool, Leptos SSR apps can be deployed to AWS Lambda. A starter template repo using Axum as the server is available at leptos-rs/start-aws; the instructions there can be adapted for you to use a Leptos+Actix-web server as well. The starter repo includes a Github Actions script for CI/CD, as well as instructions for setting up your Lambda functions and getting the necessary credentials for cloud deployment.

However, please keep in mind that some native server functionality does not work with FaaS services like Lambda because the environment is not necessarily consistent from one request to the next. In particular, the 'start-aws' docs state that "since AWS Lambda is a serverless platform, you'll need to be more careful about how you manage long-lived state. Writing to disk or using a state extractor will not work reliably across requests. Instead, you'll need a database or other microservices that you can query from the Lambda function."

The other factor to bear in mind is the 'cold-start' time for functions as a service - depending on your use case and the FaaS platform you use, this may or may not meet your latency requirements; you may need to keep one function running at all times to optimize the speed of your requests.

Deno & Cloudflare Workers
Currently, Leptos-Axum supports running in Javascript-hosted WebAssembly runtimes such as Deno, Cloudflare Workers, etc. This option requires some changes to the setup of your source code (for example, in Cargo.toml you must define your app using crate-type = ["cdylib"] and the "wasm" feature must be enabled for leptos_axum). The Leptos HackerNews JS-fetch example demonstrates the required modifications and shows how to run an app in the Deno runtime. Additionally, the leptos_axum crate docs are a helpful reference when setting up your own Cargo.toml file for JS-hosted WASM runtimes.

While the initial setup for JS-hosted WASM runtimes is not onerous, the more important restriction to keep in mind is that since your app will be compiled to WebAssembly (wasm32-unknown-unknown) on the server as well as the client, you must ensure that the crates you use in your app are all WASM-compatible; this may or may not be a deal-breaker depending on your app's requirements, as not all crates in the Rust ecosystem have WASM support.

If you're willing to live with the limitations of WASM server-side, the best place to get started right now is by checking out the example of running Leptos with Deno in the official Leptos Github repo.

Platforms Working on Leptos Support
Deploy to Spin Serverless WASI (with Leptos SSR)
WebAssembly on the server has been gaining steam lately, and the developers of the open source serverless WebAssembly framework Spin are working on natively supporting Leptos. While the Leptos-Spin SSR integration is still in its early stages, there is a working example you may wish to try out.

The full set of instructions to get Leptos SSR & Spin working together are available as a post on the Fermyon blog, or if you want to skip the article and just start playing around with a working starter repo, see here.

Deploy to Shuttle.rs
Several Leptos users have asked about the possibility of using the Rust-friendly Shuttle.rs service to deploy Leptos apps. Unfortunately, Leptos is not officially supported by the Shuttle.rs service at the moment.

However, the folks at Shuttle.rs are committed to getting Leptos support in the future; if you would like to keep up-to-date on the status of that work, keep an eye on this Github issue.

Additionally, some effort has been made to get Shuttle working with Leptos, but to date, deploys to the Shuttle cloud are still not working as expected. That work is available here, if you would like to investigate for yourself or contribute fixes: Leptos Axum Starter Template for Shuttle.rs.

Fly Machines are fast to start and stop, and you don’t pay for their CPU and RAM when they’re in the stopped state. For Fly Apps with a service configured, Fly Proxy can automatically start and stop existing Machines based on incoming requests, so that your app can meet demand without keeping extra Machines running. And if your app needs to have one or more Machines always running in your primary region, then you can set a minimum number of machines to keep running.

The auto start and stop feature also plays well with apps whose Machines exit from within when idle. If your app already shuts down when idle, then the proxy can restart it when there’s traffic.

You might also be interested in learning about scaling the number of machines.

Configure automatic start and stop
The auto start and stop settings apply per service, so you set them within the [[services]] or [http_service] sections of fly.toml:

For example:

Wrap textCopy to clipboard
...
[[services]]
internal_port = 8080
protocol = "tcp"
auto_stop_machines = true
auto_start_machines = true
min_machines_running = 0
...
Briefly, the settings are:

auto_start_machines: Whether Fly Proxy should automatically start Machines based on requests and capacity.
auto_stop_machines: Whether Fly Proxy should automatically stop Machines when the app is idle for several minutes.
min_machines_running: The minimum number of Machines to keep running in the primary region when auto_stop_machines = true.
Concurrency limits for services affect how automatic starts and stops work.

Default values
The default settings for apps that don’t specify any auto start and stop settings in fly.toml are auto_start_machines = true and auto_stop_machines = false.

When you create an app using the fly launch command, the default settings in fly.toml are:

Wrap textCopy to clipboard
...
[http_service]
internal_port = 8080
force_https = true
auto_stop_machines = true
auto_start_machines = true
min_machines_running = 0
...
Recommended settings
If your app already exits when idle, then you can set auto_start_machines = true and auto_stop_machines = false to have Fly Proxy automatically restart the Machines stopped by your app.

If your app doesn’t exit when idle, then we recommend setting auto_stop_machines and auto_start_machines to the same value (either both true or both false) so that Fly Proxy doesn’t stop Machines and never restart them. For example, if auto_start_machines = false and auto_stop_machines = true, then Fly Proxy automatically stops your Machines when there’s low traffic but doesn’t start them again. When all or most of your Machines stop, requests to your app could start failing.

To keep one or more Machines running all the time in your primary region, set min_machines_running to 1 or higher. min_machines_running has no effect unless you set auto_stop_machines = true.

min_machines_running does not apply to Machines running in non-primary regions. For example, if min_machines_running = 1 and there’s no traffic to your app, then Fly Proxy will stop Machines until eventually there is only one Machine running in your primary region.

There’s no “maximum machines running” setting, because the maximum number of Machines is just the total number of Machines you’ve created for your app. Learn more in the How it works section.

How it works
The Fly Proxy runs a process to automatically stop and start existing Fly Machines every few minutes.

The automatic start and stop feature only works on existing Machines and never creates or destroys Machines for you. The maximum number of running Machines is the number of Machines you’ve created for your app using fly scale count or fly machine clone. Learn more about scaling the number of Machines.

Fly Proxy stops Machines
When auto_stop_machines = true in your fly.toml, the proxy looks at Machines running in a single region and uses the concurrency soft_limit setting for each Machine to determine if there’s excess capacity. If the proxy decides there’s excess capacity, it stops exactly one Machine. The proxy repeats this process every few minutes, stopping only one Machine per region, if needed, each time.

If you have the kill_signal and kill_timeout options configured in your fly.toml file, then Fly Proxy uses those settings when it stops a Machine.

Fly Proxy determines excess capacity per region as follows:

If there’s more than one Machine in the region:
the proxy determines how many running Machines are over their soft_limit setting and then calculates excess capacity: excess capacity = num of machines - (num machines over soft limit + 1)
if excess capacity is 1 or greater, then the proxy stops one Machine
If there’s only one Machine in the region:
the proxy checks if the Machine has any traffic
if the Machine has no traffic (a load of 0), then the proxy stops the Machine
Fly Proxy starts Machines
When auto_start_machines = true in your fly.toml, the Fly Proxy restarts a Machine in the nearest region when required.

Fly Proxy determines when to start a Machine as follows:

The proxy waits for a request to your app.
If all the running Machines are above their soft_limit setting, then the proxy starts a stopped Machine in the nearest region (if there are any stopped Machines).
The proxy routes the request to the newly started Machine.
When to stop and start Fly Machines automatically, or not
If your app has highly variable request workloads, then you can set auto_stop_machines and auto_start_machines to true to manage your Fly Machines as demand decreases and increases. This could reduce costs, because you’ll never have to run excess Machines to handle peak load; you’ll only run, and get charged for, the number of Machines that you need.

The difference between this feature and what’s typical in autoscaling, is that it doesn’t create new Machines up to a specified maximum. It automatically starts only existing Machines. For example, if you want to have a maximum of 10 Machines available to service requests, then you need to create 10 Machines for your app.

If you need all your app’s Machines to run continuously, then you can set auto_stop_machines and auto_start_machines to false.

If you only need a certain number of your app’s Machines to run continuously, then you can set auto_stop_machines = true and min_machines_running to 1 or higher.

Stop a Machine by terminating its main process
Setting your app to automatically stop when there’s excess capacity using auto_stop_machines = true is a substitute for when your app doesn’t shut itself down automatically after a period of inactivity. If you want a custom shutdown process for your app, then you can code your app to exit from within when idle.


Optimizing WASM Binary Size
One of the primary downsides of deploying a Rust/WebAssembly frontend app is that splitting a WASM file into smaller chunks to be dynamically loaded is significantly more difficult than splitting a JavaScript bundle. There have been experiments like wasm-split in the Emscripten ecosystem but at present there’s no way to split and dynamically load a Rust/wasm-bindgen binary. This means that the whole WASM binary needs to be loaded before your app becomes interactive. Because the WASM format is designed for streaming compilation, WASM files are much faster to compile per kilobyte than JavaScript files. (For a deeper look, you can read this great article from the Mozilla team on streaming WASM compilation.)

Still, it’s important to ship the smallest WASM binary to users that you can, as it will reduce their network usage and make your app interactive as quickly as possible.

So what are some practical steps?

Things to Do
Make sure you’re looking at a release build. (Debug builds are much, much larger.)
Add a release profile for WASM that optimizes for size, not speed.
For a cargo-leptos project, for example, you can add this to your Cargo.toml:

[profile.wasm-release]
inherits = "release"
opt-level = 'z'
lto = true
codegen-units = 1

# ....

[package.metadata.leptos]
# ....
lib-profile-release = "wasm-release"
This will hyper-optimize the WASM for your release build for size, while keeping your server build optimized for speed. (For a pure client-rendered app without server considerations, just use the [profile.wasm-release] block as your [profile.release].)

Always serve compressed WASM in production. WASM tends to compress very well, typically shrinking to less than 50% its uncompressed size, and it’s trivial to enable compression for static files being served from Actix or Axum.

If you’re using nightly Rust, you can rebuild the standard library with this same profile rather than the prebuilt standard library that’s distributed with the wasm32-unknown-unknown target.

To do this, create a file in your project at .cargo/config.toml

[unstable]
build-std = ["std", "panic_abort", "core", "alloc"]
build-std-features = ["panic_immediate_abort"]
Note that if you're using this with SSR too, the same Cargo profile will be applied. You'll need to explicitly specify your target:

[build]
target = "x86_64-unknown-linux-gnu" # or whatever
Also note that in some cases, the cfg feature has_std will not be set, which may cause build errors with some dependencies which check for has_std. You may fix any build errors due to this by adding:

[build]
rustflags = ["--cfg=has_std"]
And you'll need to add panic = "abort" to [profile.release] in Cargo.toml. Note that this applies the same build-std and panic settings to your server binary, which may not be desirable. Some further exploration is probably needed here.

One of the sources of binary size in WASM binaries can be serde serialization/deserialization code. Leptos uses serde by default to serialize and deserialize resources created with create_resource. You might try experimenting with the miniserde and serde-lite features, which allow you to use those crates for serialization and deserialization instead; each only implements a subset of serde’s functionality, but typically optimizes for size over speed.
Things to Avoid
There are certain crates that tend to inflate binary sizes. For example, the regex crate with its default features adds about 500kb to a WASM binary (largely because it has to pull in Unicode table data!). In a size-conscious setting, you might consider avoiding regexes in general, or even dropping down and calling browser APIs to use the built-in regex engine instead. (This is what leptos_router does on the few occasions it needs a regular expression.)

In general, Rust’s commitment to runtime performance is sometimes at odds with a commitment to a small binary. For example, Rust monomorphizes generic functions, meaning it creates a distinct copy of the function for each generic type it’s called with. This is significantly faster than dynamic dispatch, but increases binary size. Leptos tries to balance runtime performance with binary size considerations pretty carefully; but you might find that writing code that uses many generics tends to increase binary size. For example, if you have a generic component with a lot of code in its body and call it with four different types, remember that the compiler could include four copies of that same code. Refactoring to use a concrete inner function or helper can often maintain performance and ergonomics while reducing binary size.

A Final Thought
Remember that in a server-rendered app, JS bundle size/WASM binary size affects only one thing: time to interactivity on the first load. This is very important to a good user experience: nobody wants to click a button three times and have it do nothing because the interactive code is still loading — but it's not the only important measure.

It’s especially worth remembering that streaming in a single WASM binary means all subsequent navigations are nearly instantaneous, depending only on any additional data loading. Precisely because your WASM binary is not bundle split, navigating to a new route does not require loading additional JS/WASM, as it does in nearly every JavaScript framework. Is this copium? Maybe. Or maybe it’s just an honest trade-off between the two approaches!

Always take the opportunity to optimize the low-hanging fruit in your application. And always test your app under real circumstances with real user network speeds and devices before making any heroic efforts.


Guide: Islands
Leptos 0.5 introduces the new experimental-islands feature. This guide will walk through the islands feature and core concepts, while implementing a demo app using the islands architecture.

The Islands Architecture
The dominant JavaScript frontend frameworks (React, Vue, Svelte, Solid, Angular) all originated as frameworks for building client-rendered single-page apps (SPAs). The initial page load is rendered to HTML, then hydrated, and subsequent navigations are handled directly in the client. (Hence “single page”: everything happens from a single page load from the server, even if there is client-side routing later.) Each of these frameworks later added server-side rendering to improve initial load times, SEO, and user experience.

This means that by default, the entire app is interactive. It also means that the entire app has to be shipped to the client as JavaScript in order to be hydrated. Leptos has followed this same pattern.

You can read more in the chapters on server-side rendering.

But it’s also possible to work in the opposite direction. Rather than taking an entirely-interactive app, rendering it to HTML on the server, and then hydrating it in the browser, you can begin with a plain HTML page and add small areas of interactivity. This is the traditional format for any website or app before the 2010s: your browser makes a series of requests to the server and returns the HTML for each new page in response. After the rise of “single-page apps” (SPA), this approach has sometimes become known as a “multi-page app” (MPA) by comparison.

The phrase “islands architecture” has emerged recently to describe the approach of beginning with a “sea” of server-rendered HTML pages, and adding “islands” of interactivity throughout the page.

Additional Reading
The rest of this guide will look at how to use islands with Leptos. For more background on the approach in general, check out some of the articles below:

Jason Miller, “Islands Architecture”, Jason Miller
Ryan Carniato, “Islands & Server Components & Resumability, Oh My!”
“Islands Architectures” on patterns.dev
Astro Islands
Activating Islands Mode
Let’s start with a fresh cargo-leptos app:

cargo leptos new --git leptos-rs/start
I’m using Actix because I like it. Feel free to use Axum; there should be approximately no server-specific differences in this guide.

I’m just going to run

cargo leptos build
in the background while I fire up my editor and keep writing.

The first thing I’ll do is to add the experimental-islands feature in my Cargo.toml. I need to add this to both leptos and leptos_actix:

leptos = { version = "0.5", features = ["nightly", "experimental-islands"] }
leptos_actix = { version = "0.5", optional = true, features = [
"experimental-islands",
] }
Next I’m going to modify the hydrate function exported from src/lib.rs. I’m going to remove the line that calls leptos::mount_to_body(App) and replace it with

leptos::leptos_dom::HydrationCtx::stop_hydrating();
Each “island” we create will actually act as its own entrypoint, so our hydrate() function just says “okay, hydration’s done now.”

Okay, now fire up your cargo leptos watch and go to http://localhost:3000 (or wherever).

Click the button, and...

Nothing happens!

Perfect.

Note

The starter templates include use app::*; in their hydrate() function definitions. Once you've switched over to islands mode, you are no longer using the imported main App function, so you might think you can delete this. (And in fact, Rust lint tools might issue warnings if you don't!)

However, this can cause issues if you are using a workspace setup. We use wasm-bindgen to independently export an entrypoint for each function. In my experience, if you are using a workspace setup and nothing in your frontend crate actually uses the app crate, those bindings will not be generated correctly. See this discussion for more.

Using Islands
Nothing happens because we’ve just totally inverted the mental model of our app. Rather than being interactive by default and hydrating everything, the app is now plain HTML by default, and we need to opt into interactivity.

This has a big effect on WASM binary sizes: if I compile in release mode, this app is a measly 24kb of WASM (uncompressed), compared to 355kb in non-islands mode. (355kb is quite large for a “Hello, world!” It’s really just all the code related to client-side routing, which isn’t being used in the demo.)

When we click the button, nothing happens, because our whole page is static.

So how do we make something happen?

Let’s turn the HomePage component into an island!

Here was the non-interactive version:

#[component]
fn HomePage() -> impl IntoView {
// Creates a reactive value to update the button
let (count, set_count) = create_signal(0);
let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
Here’s the interactive version:

#[island]
fn HomePage() -> impl IntoView {
// Creates a reactive value to update the button
let (count, set_count) = create_signal(0);
let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
Now when I click the button, it works!

The #[island] macro works exactly like the #[component] macro, except that in islands mode, it designates this as an interactive island. If we check the binary size again, this is 166kb uncompressed in release mode; much larger than the 24kb totally static version, but much smaller than the 355kb fully-hydrated version.

If you open up the source for the page now, you’ll see that your HomePage island has been rendered as a special <leptos-island> HTML element which specifies which component should be used to hydrate it:

<leptos-island data-component="HomePage" data-hkc="0-0-0">
  <h1 data-hk="0-0-2">Welcome to Leptos!</h1>
  <button data-hk="0-0-3">
    Click Me:
    <!-- <DynChild> -->11<!-- </DynChild> -->
  </button>
</leptos-island>
The typical Leptos hydration keys and markers are only present inside the island, only the island is hydrated.

Using Islands Effectively
Remember that only code within an #[island] needs to be compiled to WASM and shipped to the browser. This means that islands should be as small and specific as possible. My HomePage, for example, would be better broken apart into a regular component and an island:

#[component]
fn HomePage() -> impl IntoView {
view! {
<h1>"Welcome to Leptos!"</h1>
<Counter/>
}
}

#[island]
fn Counter() -> impl IntoView {
// Creates a reactive value to update the button
let (count, set_count) = create_signal(0);
let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
Now the <h1> doesn’t need to be included in the client bundle, or hydrated. This seems like a silly distinction now; but note that you can now add as much inert HTML content as you want to the HomePage itself, and the WASM binary size will remain exactly the same.

In regular hydration mode, your WASM binary size grows as a function of the size/complexity of your app. In islands mode, your WASM binary grows as a function of the amount of interactivity in your app. You can add as much non-interactive content as you want, outside islands, and it will not increase that binary size.

Unlocking Superpowers
So, this 50% reduction in WASM binary size is nice. But really, what’s the point?

The point comes when you combine two key facts:

Code inside #[component] functions now only runs on the server.
Children and props can be passed from the server to islands, without being included in the WASM binary.
This means you can run server-only code directly in the body of a component, and pass it directly into the children. Certain tasks that take a complex blend of server functions and Suspense in fully-hydrated apps can be done inline in islands.

We’re going to rely on a third fact in the rest of this demo:

Context can be passed between otherwise-independent islands.
So, instead of our counter demo, let’s make something a little more fun: a tabbed interface that reads data from files on the server.

Passing Server Children to Islands
One of the most powerful things about islands is that you can pass server-rendered children into an island, without the island needing to know anything about them. Islands hydrate their own content, but not children that are passed to them.

As Dan Abramov of React put it (in the very similar context of RSCs), islands aren’t really islands: they’re donuts. You can pass server-only content directly into the “donut hole,” as it were, allowing you to create tiny atolls of interactivity, surrounded on both sides by the sea of inert server HTML.

In the demo code included below, I added some styles to show all server content as a light-blue “sea,” and all islands as light-green “land.” Hopefully that will help picture what I’m talking about!

To continue with the demo: I’m going to create a Tabs component. Switching between tabs will require some interactivity, so of course this will be an island. Let’s start simple for now:

#[island]
fn Tabs(labels: Vec<String>) -> impl IntoView {
let buttons = labels
.into_iter()
.map(|label| view! { <button>{label}</button> })
.collect_view();
view! {
<div style="display: flex; width: 100%; justify-content: space-between;">
{buttons}
</div>
}
}
Oops. This gives me an error

error[E0463]: can't find crate for `serde`
--> src/app.rs:43:1
|
43 | #[island]
| ^^^^^^^^^ can't find crate
Easy fix: let’s cargo add serde --features=derive. The #[island] macro wants to pull in serde here because it needs to serialize and deserialize the labels prop.

Now let’s update the HomePage to use Tabs.

#[component]
fn HomePage() -> impl IntoView {
// these are the files we’re going to read
let files = ["a.txt", "b.txt", "c.txt"];
// the tab labels will just be the file names
let labels = files.iter().copied().map(Into::into).collect();
view! {
<h1>"Welcome to Leptos!"</h1>
<p>"Click any of the tabs below to read a recipe."</p>
<Tabs labels/>
}
}
If you take a look in the DOM inspector, you’ll see the island is now something like

<leptos-island
data-component="Tabs"
data-hkc="0-0-0"
data-props='{"labels":["a.txt","b.txt","c.txt"]}'
></leptos-island>
Our labels prop is getting serialized to JSON and stored in an HTML attribute so it can be used to hydrate the island.

Now let’s add some tabs. For the moment, a Tab island will be really simple:

#[island]
fn Tab(index: usize, children: Children) -> impl IntoView {
view! {
<div>{children()}</div>
}
}
Each tab, for now will just be a <div> wrapping its children.

Our Tabs component will also get some children: for now, let’s just show them all.

#[island]
fn Tabs(labels: Vec<String>, children: Children) -> impl IntoView {
let buttons = labels
.into_iter()
.map(|label| view! { <button>{label}</button> })
.collect_view();
view! {
<div style="display: flex; width: 100%; justify-content: space-around;">
{buttons}
</div>
{children()}
}
}
Okay, now let’s go back into the HomePage. We’re going to create the list of tabs to put into our tab box.

#[component]
fn HomePage() -> impl IntoView {
let files = ["a.txt", "b.txt", "c.txt"];
let labels = files.iter().copied().map(Into::into).collect();
let tabs = move || {
files
.into_iter()
.enumerate()
.map(|(index, filename)| {
let content = std::fs::read_to_string(filename).unwrap();
view! {
<Tab index>
<h2>{filename.to_string()}</h2>
<p>{content}</p>
</Tab>
}
})
.collect_view()
};

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <p>"Click any of the tabs below to read a recipe."</p>
        <Tabs labels>
            <div>{tabs()}</div>
        </Tabs>
    }
}
Uh... What?

If you’re used to using Leptos, you know that you just can’t do this. All code in the body of components has to run on the server (to be rendered to HTML) and in the browser (to hydrate), so you can’t just call std::fs; it will panic, because there’s no access to the local filesystem (and certainly not to the server filesystem!) in the browser. This would be a security nightmare!

Except... wait. We’re in islands mode. This HomePage component really does only run on the server. So we can, in fact, just use ordinary server code like this.

Is this a dumb example? Yes! Synchronously reading from three different local files in a .map() is not a good choice in real life. The point here is just to demonstrate that this is, definitely, server-only content.

Go ahead and create three files in the root of the project called a.txt, b.txt, and c.txt, and fill them in with whatever content you’d like.

Refresh the page and you should see the content in the browser. Edit the files and refresh again; it will be updated.

You can pass server-only content from a #[component] into the children of an #[island], without the island needing to know anything about how to access that data or render that content.

This is really important. Passing server children to islands means that you can keep islands small. Ideally, you don’t want to slap and #[island] around a whole chunk of your page. You want to break that chunk out into an interactive piece, which can be an #[island], and a bunch of additional server content that can be passed to that island as children, so that the non-interactive subsections of an interactive part of the page can be kept out of the WASM binary.

Passing Context Between Islands
These aren’t really “tabs” yet: they just show every tab, all the time. So let’s add some simple logic to our Tabs and Tab components.

We’ll modify Tabs to create a simple selected signal. We provide the read half via context, and set the value of the signal whenever someone clicks one of our buttons.

#[island]
fn Tabs(labels: Vec<String>, children: Children) -> impl IntoView {
let (selected, set_selected) = create_signal(0);
provide_context(selected);

    let buttons = labels
        .into_iter()
        .enumerate()
        .map(|(index, label)| view! {
            <button on:click=move |_| set_selected(index)>
                {label}
            </button>
        })
        .collect_view();
// ...
And let’s modify the Tab island to use that context to show or hide itself:

#[island]
fn Tab(children: Children) -> impl IntoView {
let selected = expect_context::<ReadSignal<usize>>();
view! {
<div style:display=move || if selected() == index {
"block"
} else {
"none"
}>
// ...
Now the tabs behave exactly as I’d expect. Tabs passes the signal via context to each Tab, which uses it to determine whether it should be open or not.

That’s why in HomePage, I made let tabs = move || a function, and called it like {tabs()}: creating the tabs lazily this way meant that the Tabs island would already have provided the selected context by the time each Tab went looking for it.

Our complete tabs demo is about 220kb uncompressed: not the smallest demo in the world, but still about a third smaller than the counter button! Just for kicks, I built the same demo without islands mode, using #[server] functions and Suspense. and it was 429kb. So again, this was about a 50% savings in binary size. And this app includes quite minimal server-only content: remember that as we add additional server-only components and pages, this 220 will not grow.

Overview
This demo may seem pretty basic. It is. But there are a number of immediate takeaways:

50% WASM binary size reduction, which means measurable improvements in time to interactivity and initial load times for clients.
Reduced HTML page size. This one is less obvious, but it’s true and important: HTML generated from #[component]s doesn’t need all the hydration IDs and other boilerplate added.
Reduced data serialization costs. Creating a resource and reading it on the client means you need to serialize the data, so it can be used for hydration. If you’ve also read that data to create HTML in a Suspense, you end up with “double data,” i.e., the same exact data is both rendered to HTML and serialized as JSON, increasing the size of responses, and therefore slowing them down.
Easily use server-only APIs inside a #[component] as if it were a normal, native Rust function running on the server—which, in islands mode, it is!
Reduced #[server]/create_resource/Suspense boilerplate for loading server data.
Future Exploration
The experimental-islands feature included in 0.5 reflects work at the cutting edge of what frontend web frameworks are exploring right now. As it stands, our islands approach is very similar to Astro (before its recent View Transitions support): it allows you to build a traditional server-rendered, multi-page app and pretty seamlessly integrate islands of interactivity.

There are some small improvements that will be easy to add. For example, we can do something very much like Astro's View Transitions approach:

add client-side routing for islands apps by fetching subsequent navigations from the server and replacing the HTML document with the new one
add animated transitions between the old and new document using the View Transitions API
support explicit persistent islands, i.e., islands that you can mark with unique IDs (something like persist:searchbar on the component in the view), which can be copied over from the old to the new document without losing their current state
There are other, larger architectural changes that I’m not sold on yet.

Additional Information
Check out the islands PR, roadmap, and Hackernews demo for additional discussion.

Demo Code
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
view! {
<Router>
<main style="background-color: lightblue; padding: 10px">
<Routes>
<Route path="" view=HomePage/>
</Routes>
</main>
</Router>
}
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
let files = ["a.txt", "b.txt", "c.txt"];
let labels = files.iter().copied().map(Into::into).collect();
let tabs = move || {
files
.into_iter()
.enumerate()
.map(|(index, filename)| {
let content = std::fs::read_to_string(filename).unwrap();
view! {
<Tab index>
<div style="background-color: lightblue; padding: 10px">
<h2>{filename.to_string()}</h2>
<p>{content}</p>
</div>
</Tab>
}
})
.collect_view()
};

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <p>"Click any of the tabs below to read a recipe."</p>
        <Tabs labels>
            <div>{tabs()}</div>
        </Tabs>
    }
}

#[island]
fn Tabs(labels: Vec<String>, children: Children) -> impl IntoView {
let (selected, set_selected) = create_signal(0);
provide_context(selected);

    let buttons = labels
        .into_iter()
        .enumerate()
        .map(|(index, label)| {
            view! {
                <button on:click=move |_| set_selected(index)>
                    {label}
                </button>
            }
        })
        .collect_view();
    view! {
        <div
            style="display: flex; width: 100%; justify-content: space-around;\
            background-color: lightgreen; padding: 10px;"
        >
            {buttons}
        </div>
        {children()}
    }
}

#[island]
fn Tab(index: usize, children: Children) -> impl IntoView {
let selected = expect_context::<ReadSignal<usize>>();
view! {
<div
style:background-color="lightgreen"
style:padding="10px"
style:display=move || if selected() == index {
"block"
} else {
"none"
}
>
{children()}
</div>
}
}


Appendix: How does the Reactive System Work?
You don’t need to know very much about how the reactive system actually works in order to use the library successfully. But it’s always useful to understand what’s going on behind the scenes once you start working with the framework at an advanced level.

The reactive primitives you use are divided into three sets:

Signals (ReadSignal/WriteSignal, RwSignal, Resource, Trigger) Values you can actively change to trigger reactive updates.
Computations (Memos) Values that depend on signals (or other computations) and derive a new reactive value through some pure computation.
Effects Observers that listen to changes in some signals or computations and run a function, causing some side effect.
Derived signals are a kind of non-primitive computation: as plain closures, they simply allow you to refactor some repeated signal-based computation into a reusable function that can be called in multiple places, but they are not represented in the reactive system itself.

All the other primitives actually exist in the reactive system as nodes in a reactive graph.

Most of the work of the reactive system consists of propagating changes from signals to effects, possibly through some intervening memos.

The assumption of the reactive system is that effects (like rendering to the DOM or making a network request) are orders of magnitude more expensive than things like updating a Rust data structure inside your app.

So the primary goal of the reactive system is to run effects as infrequently as possible.

Leptos does this through the construction of a reactive graph.

Leptos’s current reactive system is based heavily on the Reactively library for JavaScript. You can read Milo’s article “Super-Charging Fine-Grained Reactivity” for an excellent account of its algorithm, as well as fine-grained reactivity in general—including some beautiful diagrams!

The Reactive Graph
Signals, memos, and effects all share three characteristics:

Value They have a current value: either the signal’s value, or (for memos and effects) the value returned by the previous run, if any.
Sources Any other reactive primitives they depend on. (For signals, this is an empty set.)
Subscribers Any other reactive primitives that depend on them. (For effects, this is an empty set.)
In reality then, signals, memos, and effects are just conventional names for one generic concept of a “node” in a reactive graph. Signals are always “root nodes,” with no sources/parents. Effects are always “leaf nodes,” with no subscribers. Memos typically have both sources and subscribers.

Simple Dependencies
So imagine the following code:

// A
let (name, set_name) = create_signal("Alice");

// B
let name_upper = create_memo(move |_| name.with(|n| n.to_uppercase()));

// C
create_effect(move |_| {
log!("{}", name_upper());
});

set_name("Bob");
You can easily imagine the reactive graph here: name is the only signal/origin node, the create_effect is the only effect/terminal node, and there’s one intervening memo.

A   (name)
|
B   (name_upper)
|
C   (the effect)
Splitting Branches
Let’s make it a little more complex.

// A
let (name, set_name) = create_signal("Alice");

// B
let name_upper = create_memo(move |_| name.with(|n| n.to_uppercase()));

// C
let name_len = create_memo(move |_| name.len());

// D
create_effect(move |_| {
log!("len = {}", name_len());
});

// E
create_effect(move |_| {
log!("name = {}", name_upper());
});
This is also pretty straightforward: a signal source signal (name/A) divides into two parallel tracks: name_upper/B and name_len/C, each of which has an effect that depends on it.

__A__
|     |
B     C
|     |
E     D
Now let’s update the signal.

set_name("Bob");
We immediately log

len = 3
name = BOB
Let’s do it again.

set_name("Tim");
The log should shows

name = TIM
len = 3 does not log again.

Remember: the goal of the reactive system is to run effects as infrequently as possible. Changing name from "Bob" to "Tim" will cause each of the memos to re-run. But they will only notify their subscribers if their value has actually changed. "BOB" and "TIM" are different, so that effect runs again. But both names have the length 3, so they do not run again.

Reuniting Branches
One more example, of what’s sometimes called the diamond problem.

// A
let (name, set_name) = create_signal("Alice");

// B
let name_upper = create_memo(move |_| name.with(|n| n.to_uppercase()));

// C
let name_len = create_memo(move |_| name.len());

// D
create_effect(move |_| {
log!("{} is {} characters long", name_upper(), name_len());
});
What does the graph look like for this?

__A__
|     |
B     C
|     |
|__D__|
You can see why it's called the “diamond problem.” If I’d connected the nodes with straight lines instead of bad ASCII art, it would form a diamond: two memos, each of which depend on a signal, which feed into the same effect.

A naive, push-based reactive implementation would cause this effect to run twice, which would be bad. (Remember, our goal is to run effects as infrequently as we can.) For example, you could implement a reactive system such that signals and memos immediately propagate their changes all the way down the graph, through each dependency, essentially traversing the graph depth-first. In other words, updating A would notify B, which would notify D; then A would notify C, which would notify D again. This is both inefficient (D runs twice) and glitchy (D actually runs with the incorrect value for the second memo during its first run.)

Solving the Diamond Problem
Any reactive implementation worth its salt is dedicated to solving this issue. There are a number of different approaches (again, see Milo’s article for an excellent overview).

Here’s how ours works, in brief.

A reactive node is always in one of three states:

Clean: it is known not to have changed
Check: it is possible it has changed
Dirty: it has definitely changed
Updating a signal Dirty marks that signal Dirty, and marks all its descendants Check, recursively. Any of its descendants that are effects are added to a queue to be re-run.

    ____A (DIRTY)___
|               |
B (CHECK)    C (CHECK)
|               |
|____D (CHECK)__|
Now those effects are run. (All of the effects will be marked Check at this point.) Before re-running its computation, the effect checks its parents to see if they are dirty. So

So D goes to B and checks if it is Dirty.
But B is also marked Check. So B does the same thing:
B goes to A, and finds that it is Dirty.
This means B needs to re-run, because one of its sources has changed.
B re-runs, generating a new value, and marks itself Clean
Because B is a memo, it then checks its prior value against the new value.
If they are the same, B returns "no change." Otherwise, it returns "yes, I changed."
If B returned “yes, I changed,” D knows that it definitely needs to run and re-runs immediately before checking any other sources.
If B returned “no, I didn’t change,” D continues on to check C (see process above for B.)
If neither B nor C has changed, the effect does not need to re-run.
If either B or C did change, the effect now re-runs.
Because the effect is only marked Check once and only queued once, it only runs once.

If the naive version was a “push-based” reactive system, simply pushing reactive changes all the way down the graph and therefore running the effect twice, this version could be called “push-pull.” It pushes the Check status all the way down the graph, but then “pulls” its way back up. In fact, for large graphs it may end up bouncing back up and down and left and right on the graph as it tries to determine exactly which nodes need to re-run.

Note this important trade-off: Push-based reactivity propagates signal changes more quickly, at the expense of over-re-running memos and effects. Remember: the reactive system is designed to minimize how often you re-run effects, on the (accurate) assumption that side effects are orders of magnitude more expensive than this kind of cache-friendly graph traversal happening entirely inside the library’s Rust code. The measurement of a good reactive system is not how quickly it propagates changes, but how quickly it propagates changes without over-notifying.

Memos vs. Signals
Note that signals always notify their children; i.e., a signal is always marked Dirty when it updates, even if its new value is the same as the old value. Otherwise, we’d have to require PartialEq on signals, and this is actually quite an expensive check on some types. (For example, add an unnecessary equality check to something like some_vec_signal.update(|n| n.pop()) when it’s clear that it has in fact changed.)

Memos, on the other hand, check whether they change before notifying their children. They only run their calculation once, no matter how many times you .get() the result, but they run whenever their signal sources change. This means that if the memo’s computation is very expensive, you may actually want to memoize its inputs as well, so that the memo only re-calculates when it is sure its inputs have changed.

Memos vs. Derived Signals
All of this is cool, and memos are pretty great. But most actual applications have reactive graphs that are quite shallow and quite wide: you might have 100 source signals and 500 effects, but no memos or, in rare case, three or four memos between the signal and the effect. Memos are extremely good at what they do: limiting how often they notify their subscribers that they have changed. But as this description of the reactive system should show, they come with overhead in two forms:

A PartialEq check, which may or may not be expensive.
Added memory cost of storing another node in the reactive system.
Added computational cost of reactive graph traversal.
In cases in which the computation itself is cheaper than this reactive work, you should avoid “over-wrapping” with memos and simply use derived signals. Here’s a great example in which you should never use a memo:

let (a, set_a) = create_signal(1);
// none of these make sense as memos
let b = move || a() + 2;
let c = move || b() % 2 == 0;
let d = move || if c() { "even" } else { "odd" };

set_a(2);
set_a(3);
set_a(5);
Even though memoizing would technically save an extra calculation of d between setting a to 3 and 5, these calculations are themselves cheaper than the reactive algorithm.

At the very most, you might consider memoizing the final node before running some expensive side effect:

let text = create_memo(move |_| {
d()
});
create_effect(move |_| {
engrave_text_into_bar_of_gold(&text());
});


Appendix: The Life Cycle of a Signal
Three questions commonly arise at the intermediate level when using Leptos:

How can I connect to the component lifecycle, running some code when a component mounts or unmounts?
How do I know when signals are disposed, and why do I get an occasional panic when trying to access a disposed signal?
How is it possible that signals are Copy and can be moved into closures and other structures without being explicitly cloned?
The answers to these three questions are closely inter-related, and are each somewhat complicated. This appendix will try to give you the context for understanding the answers, so that you can reason correctly about your application's code and how it runs.

The Component Tree vs. The Decision Tree
Consider the following simple Leptos app:

use leptos::logging::log;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>"+1"</button>
        {move || if count() % 2 == 0 {
            view! { <p>"Even numbers are fine."</p> }.into_view()
        } else {
            view! { <InnerComponent count/> }.into_view()
        }}
    }
}

#[component]
pub fn InnerComponent(count: ReadSignal<usize>) -> impl IntoView {
create_effect(move |_| {
log!("count is odd and is {}", count());
});

    view! {
        <OddDuck/>
        <p>{count}</p>
    }
}

#[component]
pub fn OddDuck() -> impl IntoView {
view! {
<p>"You're an odd duck."</p>
}
}
All it does is show a counter button, and then one message if it's even, and a different message if it's odd. If it's odd, it also logs the values in the console.

One way to map out this simple application would be to draw a tree of nested components:

App
|_ InnerComponent
|_ OddDuck
Another way would be to draw the tree of decision points:

root
|_ is count even?
|_ yes
|_ no
If you combine the two together, you'll notice that they don't map onto one another perfectly. The decision tree slices the view we created in InnerComponent into three pieces, and combines part of InnerComponent with the OddDuck component:

DECISION            COMPONENT           DATA    SIDE EFFECTS
root                <App/>              (count) render <button>
|_ is count even?   <InnerComponent/>
|_ yes                                       render even <p>
|_ no                                        start logging the count
<OddDuck/>                  render odd <p>
render odd <p> (in <InnerComponent/>!)
Looking at this table, I notice the following things:

The component tree and the decision tree don't match one another: the "is count even?" decision splits <InnerComponent/> into three parts (one that never changes, one if even, one if odd), and merges one of these with the <OddDuck/> component.
The decision tree and the list of side effects correspond perfectly: each side effect is created at a specific decision point.
The decision tree and the tree of data also line up. It's hard to see with only one signal in the table, but unlike a component, which is a function that can include multiple decisions or none, a signal is always created at a specific line in the tree of decisions.
Here's the thing: The structure of your data and the structure of side effects affect the actual functionality of your application. The structure of your components is just a convenience of authoring. You don't care, and you shouldn't care, which component rendered which <p> tag, or which component created the effect to log the values. All that matters is that they happen at the right times.

In Leptos, components do not exist. That is to say: You can write your application as a tree of components, because that's convenient, and we provide some debugging tools and logging built around components, because that's convenient too. But your components do not exist at runtime: Components are not a unit of change detection or of rendering. They are simply function calls. You can write your whole application in one big component, or split it into a hundred components, and it does not affect the runtime behavior, because components don't really exist.

The decision tree, on the other hand, does exist. And it's really important!

The Decision Tree, Rendering, and Ownership
Every decision point is some kind of reactive statement: a signal or a function that can change over time. When you pass a signal or a function into the renderer, it automatically wraps it in an effect that subscribes to any signals it contains, and updates the view accordingly over time.

This means that when your application is rendered, it creates a tree of nested effects that perfectly mirrors the decision tree. In pseudo-code:

// root
let button = /* render the <button> once */;

// the renderer wraps an effect around the `move || if count() ...`
create_effect(|_| {
if count() % 2 == 0 {
let p = /* render the even <p> */;
} else {
// the user created an effect to log the count
create_effect(|_| {
log!("count is odd and is {}", count());
});

        let p1 = /* render the <p> from OddDuck */;
        let p2 = /* render the second <p> */ 

        // the renderer creates an effect to update the second <p>
        create_effect(|_| {
            // update the content of the <p> with the signal
            p2.set_text_content(count.get());
        });
    }
})
Each reactive value is wrapped in its own effect to update the DOM, or run any other side effects of changes to signals. But you don't need these effects to keep running forever. For example, when count switches from an odd number back to an even number, the second <p> no longer exists, so the effect to keep updating it is no longer useful. Instead of running forever, effects are canceled when the decision that created them changes. In other words, and more precisely: effects are canceled whenever the effect that was running when they were created re-runs. If they were created in a conditional branch, and re-running the effect goes through the same branch, the effect will be created again: if not, it will not.

From the perspective of the reactive system itself, your application's "decision tree" is really a reactive "ownership tree." Simply put, a reactive "owner" is the effect or memo that is currently running. It owns effects created within it, they own their own children, and so on. When an effect is going to re-run, it first "cleans up" its children, then runs again.

So far, this model is shared with the reactive system as it exists in JavaScript frameworks like S.js or Solid, in which the concept of ownership exists to automatically cancel effects.

What Leptos adds is that we add a second, similar meaning to ownership: a reactive owner not only owns its child effects, so that it can cancel them; it also owns its signals (memos, etc.) so that it can dispose of them.

Ownership and the Copy Arena
This is the innovation that allows Leptos to be usable as a Rust UI framework. Traditionally, managing UI state in Rust has been hard, because UI is all about shared mutability. (A simple counter button is enough to see the problem: You need both immutable access to set the text node showing the counter's value, and mutable access in the click handler, and every Rust UI framework is designed around the fact that Rust is designed to prevent exactly that!) Using something like an event handler in Rust traditionally relies on primitives for communicating via shared memory with interior mutability (Rc<RefCell<_>>, Arc<Mutex<_>>) or for shared memory by communicating via channels, either of which often requires explicit .clone()ing to be moved into an event listener. This is kind of fine, but also an enormous inconvenience.

Leptos has always used a form of arena allocation for signals instead. A signal itself is essentially an index into a data structure that's held elsewhere. It's a cheap-to-copy integer type that does not do reference counting on its own, so it can be copied around, moved into event listeners, etc. without explicit cloning.

Instead of Rust lifetimes or reference counting, the life cycles of these signals are determined by the ownership tree.

Just as all effects belong to an owning parent effect, and the children are canceled when the owner reruns, so too all signals belong to an owner, and are disposed of when the parent reruns.

In most cases, this is completely fine. Imagine that in our example above, <OddDuck/> created some other signal that it used to update part of its UI. In most cases, that signal will be used for local state in that component, or maybe passed down as a prop to another component. It's unusual for it to be hoisted up out of the decision tree and used somewhere else in the application. When the count switches back to an even number, it is no longer needed and can be disposed.

However, this means there are two possible issues that can arise.

Signals can be used after they are disposed
The ReadSignal or WriteSignal that you hold is just an integer: say, 3 if it's the 3rd signal in the application. (As always, the reality is a bit more complicated, but not much.) You can copy that number all over the place and use it to say, "Hey, get me signal 3." When the owner cleans up, the value of signal 3 will be invalidated; but the number 3 that you've copied all over the place can't be invalidated. (Not without a whole garbage collector!) That means that if you push signals back "up" the decision tree, and store them somewhere conceptually "higher" in your application than they were created, they can be accessed after being disposed.

If you try to update a signal after it was disposed, nothing bad really happens. The framework will just warn you that you tried to update a signal that no longer exists. But if you try to access one, there's no coherent answer other than panicking: there is no value that could be returned. (There are try_ equivalents to the .get() and .with() methods that will simply return None if a signal has been disposed).

Signals can be leaked if you create them in a higher scope and never dispose of them
The opposite is also true, and comes up particularly when working with collections of signals, like an RwSignal<Vec<RwSignal<_>>>. If you create a signal at a higher level, and pass it down to a component at a lower level, it is not disposed until the higher-up owner is cleaned up.

For example, if you have a todo app that creates a new RwSignal<Todo> for each todo, stores it in an RwSignal<Vec<RwSignal<Todo>>>, and then passes it down to a <Todo/>, that signal is not automatically disposed when you remove the todo from the list, but must be manually disposed, or it will "leak" for as long as its owner is still alive. (See the TodoMVC example for more discussion.)

This is only an issue when you create signals, store them in a collection, and remove them from the collection without manually disposing of them as well.

Connecting the Dots
The answers to the questions we started with should probably make some sense now.

Component Life-Cycle
There is no component life-cycle, because components don't really exist. But there is an ownership lifecycle, and you can use it to accomplish the same things:

before mount: simply running code in the body of a component will run it "before the component mounts"
on mount: create_effect runs a tick after the rest of the component, so it can be useful for effects that need to wait for the view to be mounted to the DOM.
on unmount: You can use on_cleanup to give the reactive system code that should run while the current owner is cleaning up, before running again. Because an owner is around a "decision," this means that on_cleanup will run when your component unmounts: if something can unmount, the renderer must have created an effect that's unmounting it!
Issues with Disposed Signals
Generally speaking, problems can only arise here if you are creating a signal lower down in the ownership tree and storing it somewhere higher up. If you run into issues here, you should instead "hoist" the signal creation up into the parent, and then pass the created signals down—making sure to dispose of them on removal, if needed!

Copy signals
The whole system of Copyable wrapper types (signals, StoredValue, and so on) uses the ownership tree as a close approximation of the life-cycle of different parts of your UI. In effect, it parallels the Rust language's system of lifetimes based on blocks of code with a system of lifetimes based on sections of UI. This can't always be perfectly checked at compile time, but overall we think it's a net positive.





















