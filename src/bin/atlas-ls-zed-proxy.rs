use std::{
    env,
    io::{self, BufRead, BufReader, Write},
    process::{ChildStdout, Command, Stdio},
    thread,
};

fn read_message<R: BufRead>(reader: &mut R) -> io::Result<Option<Vec<u8>>> {
    let mut content_length = None;

    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            return Ok(None);
        }
        let line = line.trim_end_matches(['\r', '\n']);
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                content_length = Some(value.trim().parse::<usize>().map_err(|err| {
                    io::Error::new(io::ErrorKind::InvalidData, format!("invalid Content-Length: {err}"))
                })?);
            }
        }
    }

    let Some(content_length) = content_length else {
        return Ok(None);
    };
    let mut body = vec![0; content_length];
    reader.read_exact(&mut body)?;
    Ok(Some(body))
}

fn write_message<W: Write>(writer: &mut W, body: &[u8]) -> io::Result<()> {
    write!(writer, "Content-Length: {}\r\n\r\n", body.len())?;
    writer.write_all(body)?;
    writer.flush()
}

fn patch_initialize_response(mut body: Vec<u8>) -> Vec<u8> {
    let Ok(mut value) = serde_json::from_slice::<serde_json::Value>(&body) else {
        return body;
    };

    let Some(commands) = value
        .pointer_mut("/result/capabilities/executeCommandProvider/commands")
    else {
        return body;
    };

    if commands.is_null() {
        *commands = serde_json::Value::Array(Vec::new());
        if let Ok(patched) = serde_json::to_vec(&value) {
            body = patched;
        }
    }

    body
}

fn forward_client_to_server(mut server_stdin: impl Write) -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    while let Some(body) = read_message(&mut reader)? {
        write_message(&mut server_stdin, &body)?;
    }
    Ok(())
}

fn forward_server_to_client(server_stdout: ChildStdout) -> io::Result<()> {
    let mut reader = BufReader::new(server_stdout);
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    while let Some(body) = read_message(&mut reader)? {
        write_message(&mut writer, &patch_initialize_response(body))?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let Some(server) = args.next() else {
        eprintln!("usage: atlas-ls-zed-proxy <server> [args...]");
        std::process::exit(2);
    };
    let server_args = args.collect::<Vec<_>>();

    let mut child = Command::new(server)
        .args(server_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let server_stdin = child.stdin.take().expect("server stdin");
    let server_stdout = child.stdout.take().expect("server stdout");

    let client_to_server = thread::spawn(move || forward_client_to_server(server_stdin));
    let server_to_client = thread::spawn(move || forward_server_to_client(server_stdout));

    let status = child.wait()?;
    let _ = client_to_server.join();
    let _ = server_to_client.join();

    std::process::exit(status.code().unwrap_or(1));
}
