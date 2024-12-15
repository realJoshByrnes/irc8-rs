mod tcp;

fn main() {
    tcp::start_server("127.0.0.1:6667");
}

fn handle_message(message: &str) -> String {
    let mut response = String::new();
    let lines: Vec<&str> = message.lines().collect();
    for line in lines {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "NICK" => {
                if parts.len() > 1 {
                    response.push_str(&format!(
                        ":server 001 {} :Welcome to the IRC server\r\n",
                        parts[1]
                    ));
                } else {
                    response.push_str(":server 431 :No nickname given\r\n");
                }
            }
            "USER" => {
                if parts.len() > 4 {
                    response.push_str(&format!(
                        ":server 001 {} :Welcome to the IRC server\r\n",
                        parts[1]
                    ));
                } else {
                    response.push_str(":server 461 USER :Not enough parameters\r\n");
                }
            }
            "PING" => {
                if parts.len() > 1 {
                    response.push_str(&format!("PONG {}\r\n", parts[1]));
                } else {
                    response.push_str("PONG\r\n");
                }
            }
            _ => {
                response.push_str(&format!(":server 421 {} :Unknown command\r\n", parts[0]));
            }
        }
    }

    response
}
