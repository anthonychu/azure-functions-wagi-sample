fn main() {
    println!("Content-Type: text/html; charset=utf-8\n");
    println!("<html><body><h1>Hello from 🦀 WebAssembly on ⚡️ Azure Functions with WAGI!!</h1>");
    println!("\n\n<h2>Environment variables</h2>\n<pre>");
    std::env::vars().for_each(|v| {
        println!("{} = {}", v.0, v.1);
    });
    println!("</pre></body></html>");
}