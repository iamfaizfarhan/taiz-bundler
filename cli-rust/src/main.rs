use clap::Parser as ClapParser;
use std::fs;
use std::path::Path;
use swc_core::{
    ecma::{
        parser::{Syntax, EsConfig},
        transforms::base::resolver,
        minifier,
        visit::FoldWith,
        codegen::{Emitter, text_writer::JsWriter},
    },
    common::{
        SourceMap,
        FileName,
        comments::SingleThreadedComments,
        Mark,
        errors::Handler,
        sync::Lrc,
        GLOBALS,
    },
};
use swc_ecma_parser::{Parser, StringInput};
use swc_ecma_minifier::optimize;

#[derive(ClapParser)]
#[clap(name = "taiz", about = "Fastest JS runtime bundler")]
struct Args {
    #[clap(value_parser)]
    input: String,

    #[clap(short = 'o', value_parser)]
    output: String,
}

fn main() {
    let args = Args::parse();

    let input_path = Path::new(&args.input);
    let input_code = match fs::read_to_string(input_path) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Failed to read {}: {}", args.input, err);
            std::process::exit(1);
        }
    };

    let runtime_code = "const r=typeof window!='undefined'?0:typeof Deno!='undefined'?2:1;const f=[fetch,require('node-fetch'),Deno.fetch][r];const fsImpl=[{write:(p,d)=>localStorage.setItem(p,JSON.stringify(d)),read:(p)=>Promise.resolve(JSON.parse(localStorage.getItem(p)||'null'))},{write:(p,d)=>require('fs/promises').writeFile(p,JSON.stringify(d)),read:(p)=>require('fs/promises').readFile(p,'utf8').then(JSON.parse)},{write:(p,d)=>Deno.writeTextFile(p,JSON.stringify(d)),read:(p)=>Deno.readTextFile(p).then(JSON.parse)}][r];const wImpl=[(f,i)=>new Promise(r=>{const w=new Worker(URL.createObjectURL(new Blob([`onmessage=e=>postMessage((${f})(e.data))`],{type:'application/javascript'})));w.onmessage=e=>{r(e.data);w.terminate()};w.onerror=e=>r({error:e.message});w.postMessage(i,i.buffer?[i.buffer]:undefined)}),(f,i)=>new Promise((r,x)=>{const{Worker}=require('worker_threads');const w=new Worker(`process.on('message',d=>process.send((${f})(d)))`,{eval:true,transferList:i.buffer?[i.buffer]:[]});w.on('message',r);w.on('error',x);w.postMessage(i);w.unref()}),(f,i)=>new Promise(r=>{const w=new Worker(`onmessage=e=>postMessage((${f})(e.data))`,{type:'module'});w.onmessage=e=>{r(e.data);w.terminate()};w.onerror=e=>r({error:e.message});w.postMessage(i,i.buffer?[i.buffer]:[])})][r];const cImpl=[{sha256:async(s)=>Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256',new TextEncoder().encode(s)))).map(b=>b.toString(16).padStart(2,'0')).join('')},{sha256:(s)=>require('crypto').createHash('sha256').update(s).digest('hex')},{sha256:async(s)=>[...new Uint8Array(await Deno.core.opAsync('op_hash_sha256',new TextEncoder().encode(s)))].map(b=>b.toString(16).padStart(2,'0')).join('')}][r];globalThis.taiz={fetch:(u,o)=>f(u,o).then(r=>r.json()),fs:fsImpl,worker:(f,i)=>wImpl(f.toString(),i).then(r=>r.error?Promise.reject(new Error(r.error)):r),crypto:cImpl};";

    let bundled_code = format!("{}\n{}", runtime_code, input_code);

    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        FileName::Custom(args.input.clone()),
        bundled_code.into(),
    );

    // Set SWC globals to avoid thread-local panic
    GLOBALS.set(&Default::default(), || {
        let comments = SingleThreadedComments::default();
        let _handler = Handler::with_emitter_writer(Box::new(std::io::stderr()), Some(cm.clone()));
        let mut parser = Parser::new(
            Syntax::Es(EsConfig {
                jsx: false,
                fn_bind: false,
                decorators: false,
                decorators_before_export: false,
                export_default_from: false,
                import_attributes: false,
                allow_super_outside_method: false,
                allow_return_outside_function: false,
                auto_accessors: false,
                explicit_resource_management: false,
            }),
            StringInput::from(&*fm),
            Some(&comments),
        );
        let program = match parser.parse_program() {
            Ok(prog) => prog,
            Err(err) => {
                eprintln!("Parse error: {:?}", err);
                std::process::exit(1);
            }
        };

        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();
        let program = program.fold_with(&mut resolver(unresolved_mark, top_level_mark, false));

        let minified = optimize(
            program,
            cm.clone(),
            Some(&comments),
            None,
            &minifier::option::MinifyOptions {
                compress: Some(minifier::option::CompressOptions {
                    ..Default::default()
                }),
                mangle: Some(minifier::option::MangleOptions {
                    ..Default::default()
                }),
                ..Default::default()
            },
            &minifier::option::ExtraOptions {
                top_level_mark,
                unresolved_mark,
            },
        );

        let mut buf = vec![];
        let mut emitter = Emitter {
            cfg: Default::default(),
            cm: cm.clone(),
            comments: Some(&comments),
            wr: JsWriter::new(cm, "\n", &mut buf, None),
        };
        emitter.emit_program(&minified).unwrap();
        let minified_code = String::from_utf8(buf).unwrap();

        let output_path = Path::new(&args.output);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        fs::write(output_path, minified_code).unwrap();
        println!("Bundled {} -> {} for all targets", args.input, args.output);
    });
}